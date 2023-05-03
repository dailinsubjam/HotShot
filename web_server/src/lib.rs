pub mod api_config;

use api_config::{
    ProposalWithEncSecret, ServerEncKey, DEFAULT_WEB_SERVER_PORT, MAX_TXNS, MAX_VIEWS,
};
use async_compatibility_layer::channel::OneShotReceiver;
use async_lock::RwLock;
use bincode::Options as BincodeOpts;
use clap::Args;
use futures::FutureExt;

use hotshot_types::traits::signature_key::SignatureKey;
use hotshot_utils::bincode::bincode_opts;
use jf_primitives::aead::EncKey;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use rand_chacha::ChaCha20Rng;

use std::collections::HashMap;
use std::io;
use std::path::PathBuf;
use tide_disco::api::ApiError;
use tide_disco::error::ServerError;
use tide_disco::method::ReadState;
use tide_disco::method::WriteState;
use tide_disco::Api;
use tide_disco::App;
use tide_disco::StatusCode;
use tracing::{error, info};

type State<KEY> = RwLock<WebServerState<KEY>>;
type Error = ServerError;

/// State that tracks proposals and votes the server receives
/// Data is stored as a `Vec<u8>` to not incur overhead from deserializing
struct WebServerState<KEY> {
    /// view number -> (secret, proposal)
    old_proposals_struct: HashMap<u64, (String, Vec<u8>)>,
    /// view number -> (plaintext secret for this view, proposal to be sent out with encrypted secret for the next view)
    proposals: HashMap<u64, ProposalWithEncSecret>,
    /// view for oldest proposals in memory
    oldest_proposal: u64,
    /// view number -> Vec(index, vote)
    votes: HashMap<u64, Vec<(u64, Vec<u8>)>>,
    /// view number -> highest vote index for that view number
    vote_index: HashMap<u64, u64>,
    /// view number of oldest votes in memory
    oldest_vote: u64,
    /// index -> transaction
    transactions: HashMap<u64, Vec<u8>>,
    /// highest transaction index
    num_txns: u64,
    /// shutdown signal
    shutdown: Option<OneShotReceiver<()>>,
    /// stake table with leader keys
    stake_table: Vec<(KEY, EncKey)>,
    /// prng for generating endpoint
    _prng: StdRng,
    /// view number -> plaintext secret
    secrets: HashMap<u64, String>,
}

impl<KEY: SignatureKey + 'static> WebServerState<KEY> {
    fn new() -> Self {
        Self {
            old_proposals_struct: HashMap::new(),
            proposals: HashMap::new(),
            votes: HashMap::new(),
            num_txns: 0,
            oldest_vote: 0,
            oldest_proposal: 0,
            shutdown: None,
            stake_table: Vec::new(),
            vote_index: HashMap::new(),
            transactions: HashMap::new(),
            _prng: StdRng::from_entropy(),
            secrets: HashMap::new(),
            //generate first 10 views' secrets
            // secrets: (0..10)
            //     .map(|view| {
            //         (
            //             view,
            //             thread_rng()
            //                 .sample_iter(&Alphanumeric)
            //                 .take(30)
            //                 .map(char::from)
            //                 .collect(),
            //         )
            //     })
            //     .collect(),
        }
    }
    pub fn with_shutdown_signal(mut self, shutdown_listener: Option<OneShotReceiver<()>>) -> Self {
        if self.shutdown.is_some() {
            panic!("A shutdown signal is already registered and can not be registered twice");
        }
        self.shutdown = shutdown_listener;
        self
    }
}

/// Trait defining methods needed for the `WebServerState`
pub trait WebServerDataSource<KEY> {
    fn get_proposal(&self, view_number: u64) -> Result<Option<ProposalWithEncSecret>, Error>;
    fn get_votes(&self, view_number: u64, index: u64) -> Result<Option<Vec<Vec<u8>>>, Error>;
    fn get_transactions(&self, index: u64) -> Result<Option<Vec<Vec<u8>>>, Error>;
    fn post_vote(&mut self, view_number: u64, vote: Vec<u8>) -> Result<(), Error>;
    fn post_proposal(&mut self, view_number: u64, proposal: Vec<u8>) -> Result<(), Error>;
    fn post_transaction(&mut self, txn: Vec<u8>) -> Result<(), Error>;
    fn post_staketable(&mut self, keypair: Vec<u8>) -> Result<(), Error>;
    fn post_secret_proposal(&mut self, view_number: u64, proposal: Vec<u8>) -> Result<(), Error>;
    fn proposal(&self, view_number: u64) -> Option<ProposalWithEncSecret>;
    fn secret(&self, view_number: u64) -> Option<String>;
    fn next_leader(&self, view_number: u64) -> u64;
}

impl<KEY: SignatureKey> WebServerDataSource<KEY> for WebServerState<KEY> {
    fn proposal(&self, view_number: u64) -> Option<ProposalWithEncSecret> {
        self.proposals.get(&view_number).cloned()
    }
    fn secret(&self, view_number: u64) -> Option<String> {
        self.secrets.get(&view_number).cloned()
    }
    fn next_leader(&self, view_number: u64) -> u64 {
        (view_number + 1) % self.stake_table.len() as u64
    }
    /// Return the proposal the server has received for a particular view
    fn get_proposal(&self, view_number: u64) -> Result<Option<ProposalWithEncSecret>, Error> {
        match self.proposals.get(&view_number) {
            Some(proposal) => {
                if proposal.proposal.is_empty() {
                    Err(ServerError {
                        status: StatusCode::NotImplemented,
                        message: format!("Proposal not found for view {view_number}"),
                    })
                } else {
                    Ok(Some(proposal.clone()))
                }
            }
            None => Err(ServerError {
                status: StatusCode::NotImplemented,
                message: format!("Proposal not found for view {view_number}"),
            }),
        }
    }

    /// Return all votes the server has received for a particular view from provided index to most recent
    fn get_votes(&self, view_number: u64, index: u64) -> Result<Option<Vec<Vec<u8>>>, Error> {
        let votes = self.votes.get(&view_number);
        let mut ret_votes = vec![];
        if let Some(votes) = votes {
            for i in index..*self.vote_index.get(&view_number).unwrap() {
                ret_votes.push(votes[i as usize].1.clone());
            }
        }
        if !ret_votes.is_empty() {
            Ok(Some(ret_votes))
        } else {
            Ok(None)
        }
    }

    /// Return the transaction at the specified index (which will help with Nginx caching, but reduce performance otherwise)
    /// In the future we will return batches of transactions
    fn get_transactions(&self, index: u64) -> Result<Option<Vec<Vec<u8>>>, Error> {
        let mut txns = vec![];
        if let Some(txn) = self.transactions.get(&index) {
            txns.push(txn.clone())
        }
        if !txns.is_empty() {
            Ok(Some(txns))
        } else {
            Err(ServerError {
                // TODO ED: Why does NoContent status code cause errors?
                status: StatusCode::NotImplemented,
                message: format!("Transaction not found for index {index}"),
            })
        }
    }

    /// Stores a received vote in the `WebServerState`
    fn post_vote(&mut self, view_number: u64, vote: Vec<u8>) -> Result<(), Error> {
        // Only keep vote history for MAX_VIEWS number of views
        if self.votes.len() >= MAX_VIEWS {
            self.votes.remove(&self.oldest_vote);
            while !self.votes.contains_key(&self.oldest_vote) {
                self.oldest_vote += 1;
            }
        }
        let highest_index = self.vote_index.entry(view_number).or_insert(0);
        self.votes
            .entry(view_number)
            .and_modify(|current_votes| current_votes.push((*highest_index, vote.clone())))
            .or_insert_with(|| vec![(*highest_index, vote)]);
        self.vote_index
            .entry(view_number)
            .and_modify(|index| *index += 1);
        Ok(())
    }
    /// Stores a received proposal in the `WebServerState`
    fn post_proposal(&mut self, view_number: u64, mut proposal: Vec<u8>) -> Result<(), Error> {
        error!("Received proposal for view {}", view_number);

        // Only keep proposal history for MAX_VIEWS number of view
        if self.old_proposals_struct.len() >= MAX_VIEWS {
            self.old_proposals_struct.remove(&self.oldest_proposal);
            while !self
                .old_proposals_struct
                .contains_key(&self.oldest_proposal)
            {
                self.oldest_proposal += 1;
            }
        }
        self.old_proposals_struct
            .entry(view_number)
            .and_modify(|(_, empty_proposal)| empty_proposal.append(&mut proposal))
            .or_insert_with(|| (String::new(), proposal));
        Ok(())
    }
    /// Stores a received group of transactions in the `WebServerState`
    fn post_transaction(&mut self, txn: Vec<u8>) -> Result<(), Error> {
        // Only keep MAX_TXNS in memory
        if self.transactions.len() >= MAX_TXNS {
            self.transactions.remove(&(self.num_txns - MAX_TXNS as u64));
        }
        self.transactions.insert(self.num_txns, txn);
        self.num_txns += 1;

        Ok(())
    }

    fn post_staketable(&mut self, keypair: Vec<u8>) -> Result<(), Error> {
        // KALEY TODO: need security checks here
        let keypair = bincode_opts().deserialize::<(KEY, ServerEncKey)>(&keypair);
        if let Ok((pub_key, enc_key)) = keypair {
            //let new_enc_key = jf_primitives::aead::EncKey::from(keypair.1.1);
            let node_index = self.stake_table.len() as u64;
            //generate secret for leader's first submission endpoint when key is added
            //secret should be random
            let secret = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(30)
                .map(char::from)
                .collect();
            self.secrets.insert(node_index, secret);
            self.stake_table.push((pub_key, enc_key.enc_key));
            Ok(())
        } else {
            Err(ServerError {
                status: StatusCode::BadRequest,
                message: "Only signature keys can be added to stake table".to_string(),
            })
        }
    }

    //KALEY TODO: this will be merged with post_proposal once it is fully working,
    //but keeping it separate to not break things in the meantime
    fn post_secret_proposal(
        &mut self,
        view_number: u64,
        proposal: Vec<u8>,
    ) -> Result<(), Error> {
        info!("Received proposal for view {}", view_number);

        // Only keep proposal history for MAX_VIEWS number of views
        if self.proposals.len() >= MAX_VIEWS {
            self.proposals.remove(&self.oldest_proposal);
            while !self.proposals.contains_key(&self.oldest_proposal) {
                self.oldest_proposal += 1;
            }
        }

        //KALEY TODO: current implementation relies on proposal being submitted/requested
        //in the sequential order. Leader of view n+1 can only get the secret by requesting the proposal
        //for view n, which has the encrypted secret

        //generate new secret for the next time this node is leader
        let secret = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();
        let next_view_for_leader = view_number + self.stake_table.len() as u64;
        self.secrets.insert(next_view_for_leader, secret);

        //add next leader's encrypted secret to proposal
        let next_leader_keys = &self.stake_table[self.next_leader(view_number) as usize];
        let mut rng = ChaCha20Rng::from_seed([0u8; 32]);
        let next_view_secret = self.secrets.get(&(view_number + 1));
        let encrypted_secret = if let Some(next_view_secret) = next_view_secret {
            next_leader_keys.1.encrypt(
                &mut rng,
                next_view_secret.as_bytes(),
                &next_leader_keys.0.to_bytes().0,
            )
        } else {
            let next_secret: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(30)
                .map(char::from)
                .collect();
            self.secrets.insert(next_view_for_leader, next_secret.clone());
            next_leader_keys.1.encrypt(
                &mut rng,
                next_secret.as_bytes(),
                &next_leader_keys.0.to_bytes().0,
            )
        }
        .expect("Failed to encrypt secret");

        self.proposals.insert(
            view_number,
            ProposalWithEncSecret {
                secret: encrypted_secret,
                proposal: proposal,
            },
        );

        Ok(())
    }
}

#[derive(Args, Default)]
pub struct Options {
    #[arg(long = "web-server-api-path", env = "WEB_SERVER_API_PATH")]
    pub api_path: Option<PathBuf>,
}

/// Sets up all API routes
fn define_api<State, KEY>(options: &Options) -> Result<Api<State, Error>, ApiError>
where
    State: 'static + Send + Sync + ReadState + WriteState,
    <State as ReadState>::State: Send + Sync + WebServerDataSource<KEY>,
    KEY: SignatureKey,
{
    let mut api = match &options.api_path {
        Some(path) => Api::<State, Error>::from_file(path)?,
        None => {
            let toml = toml::from_str(include_str!("../api.toml")).map_err(|err| {
                ApiError::CannotReadToml {
                    reason: err.to_string(),
                }
            })?;
            Api::<State, Error>::new(toml)?
        }
    };
    api.get("getproposal", |req, state| {
        async move {
            let view_number: u64 = req.integer_param("view_number")?;
            state.get_proposal(view_number)
        }
        .boxed()
    })?
    .get("getvotes", |req, state| {
        async move {
            let view_number: u64 = req.integer_param("view_number")?;
            let index: u64 = req.integer_param("index")?;
            state.get_votes(view_number, index)
        }
        .boxed()
    })?
    .get("gettransactions", |req, state| {
        async move {
            let index: u64 = req.integer_param("index")?;
            state.get_transactions(index)
        }
        .boxed()
    })?
    .post("postvote", |req, state| {
        async move {
            let view_number: u64 = req.integer_param("view_number")?;
            // Using body_bytes because we don't want to deserialize; body_auto or body_json deserializes automatically
            let vote = req.body_bytes();
            state.post_vote(view_number, vote)
        }
        .boxed()
    })?
    .post("postproposal", |req, state| {
        async move {
            let view_number: u64 = req.integer_param("view_number")?;
            let proposal = req.body_bytes();
            state.post_proposal(view_number, proposal)
        }
        .boxed()
    })?
    .post("posttransaction", |req, state| {
        async move {
            let txns = req.body_bytes();
            state.post_transaction(txns)
        }
        .boxed()
    })?
    .post("poststaketable", |req, state| {
        async move {
            //works one key at a time for now
            let keypair = req.body_bytes();
            state.post_staketable(keypair)
        }
        .boxed()
    })?
    .post("secret", |req, state| {
        async move {
            let view_number: u64 = req.integer_param("view_number")?;
            let secret: &str = req.string_param("secret")?;
            //if secret is correct and view_number->proposal is empty, proposal is valid
            if let None = state.proposal(view_number) {
                if let Some(server_secret) = state.secret(view_number) {
                    if server_secret == secret {
                        let proposal = req.body_bytes();
                        state.post_secret_proposal(view_number, proposal)
                    } else {
                        Err(ServerError {
                            status: StatusCode::BadRequest,
                            message: format!(
                                "Wrong secret value for proposal for view {:?}",
                                view_number
                            ),
                        })
                    }
                } else {
                    Err(ServerError {
                        status: StatusCode::BadRequest,
                        message: format!("No endpoint for view number {} yet", view_number),
                    })
                }
            } else {
                Err(ServerError {
                    status: StatusCode::BadRequest,
                    message: format!("Proposal already submitted for view {:?}", view_number),
                })
            }
        }
        .boxed()
    })?;
    Ok(api)
}

pub async fn run_web_server<KEY: SignatureKey + 'static>(
    shutdown_listener: Option<OneShotReceiver<()>>,
) -> io::Result<()> {
    let options = Options::default();
    let api = define_api(&options).unwrap();
    let state = State::new(WebServerState::new().with_shutdown_signal(shutdown_listener));
    let mut app = App::<State<KEY>, Error>::with_state(state);

    app.register_module("api", api).unwrap();
    app.serve(format!("http://0.0.0.0:{DEFAULT_WEB_SERVER_PORT}"))
        .await
}

#[cfg(test)]
#[cfg(feature = "demo")]
mod test {
    use crate::config::{
        get_proposal_route, get_transactions_route, get_vote_route, post_proposal_route,
        post_transactions_route, post_vote_route,
    };

    use super::*;
    use async_compatibility_layer::art::async_spawn;
    use hotshot_types::traits::signature_key::ed25519::Ed25519Pub;
    use portpicker::pick_unused_port;
    use surf_disco::error::ClientError;

    type State = RwLock<WebServerState<Ed25519Pub>>;
    type Error = ServerError;

    #[cfg_attr(
        feature = "tokio-executor",
        tokio::test(flavor = "multi_thread", worker_threads = 2)
    )]
    #[cfg_attr(feature = "async-std-executor", async_std::test)]
    async fn test_web_server() {
        let port = pick_unused_port().unwrap();
        let base_url = format!("0.0.0.0:{port}");
        let options = Options::default();
        let api = define_api(&options).unwrap();
        let mut app = App::<State, Error>::with_state(State::new(WebServerState::new()));

        app.register_module("api", api).unwrap();
        let _handle = async_spawn(app.serve(base_url.clone()));

        let base_url = format!("http://{base_url}").parse().unwrap();
        let client = surf_disco::Client::<ClientError>::new(base_url);
        assert!(client.connect(None).await);

        // Test posting and getting proposals
        let prop1 = "prop1";
        client
            .post::<()>(&post_proposal_route(1))
            .body_binary(&prop1)
            .unwrap()
            .send()
            .await
            .unwrap();
        let resp = client
            .get::<Option<Vec<Vec<u8>>>>(&get_proposal_route(1))
            .send()
            .await
            .unwrap()
            .unwrap();
        let res1: &str = bincode::deserialize(&resp[0]).unwrap();
        assert_eq!(res1, prop1);

        let prop2 = "prop2";
        client
            .post::<()>(&post_proposal_route(2))
            .body_binary(&prop2)
            .unwrap()
            .send()
            .await
            .unwrap();
        let resp = client
            .get::<Option<Vec<Vec<u8>>>>(&get_proposal_route(2))
            .send()
            .await
            .unwrap()
            .unwrap();

        let res2: &str = bincode::deserialize(&resp[0]).unwrap();
        assert_eq!(res2, prop2);
        assert_ne!(res1, res2);

        assert_eq!(
            client
                .get::<Option<Vec<u8>>>(&get_proposal_route(3))
                .send()
                .await,
            Err(ClientError {
                status: StatusCode::NotImplemented,
                message: "Proposal not found for view 3".to_string()
            })
        );

        // Test posting and getting votes
        let vote1 = "vote1";
        client
            .post::<()>(&post_vote_route(1))
            .body_binary(&vote1)
            .unwrap()
            .send()
            .await
            .unwrap();

        let vote2 = "vote2";
        client
            .post::<()>(&post_vote_route(1))
            .body_binary(&vote2)
            .unwrap()
            .send()
            .await
            .unwrap();
        let resp = client
            .get::<Option<Vec<Vec<u8>>>>(&get_vote_route(1, 0))
            .send()
            .await
            .unwrap()
            .unwrap();
        let res1: &str = bincode::deserialize(&resp[0]).unwrap();
        let res2: &str = bincode::deserialize(&resp[1]).unwrap();
        assert_eq!(vote1, res1);
        assert_eq!(vote2, res2);
        //check for proper indexing
        let resp = client
            .get::<Option<Vec<Vec<u8>>>>(&get_vote_route(1, 1))
            .send()
            .await
            .unwrap()
            .unwrap();
        let res: &str = bincode::deserialize(&resp[0]).unwrap();
        assert_eq!(vote2, res);

        //test posting/getting transactions
        let txns1 = "abc";
        client
            .post::<()>(&post_transactions_route())
            .body_binary(&txns1)
            .unwrap()
            .send()
            .await
            .unwrap();
        let resp = client
            .get::<Option<Vec<Vec<u8>>>>(&get_transactions_route(0))
            .send()
            .await
            .unwrap()
            .unwrap();

        let txn_resp1: &str = bincode::deserialize(&resp[0]).unwrap();
        assert_eq!(txns1, txn_resp1);
    }
}
