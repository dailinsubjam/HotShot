use futures::{
    future::{join_all, BoxFuture},
    FutureExt,
};

use crate::{
    global_registry::{GlobalRegistry, HotShotTaskId},
    task::{HotShotTaskCompleted, HotShotTaskTypes, TaskErr},
    task_impls::TaskBuilder,
};

// TODO use genericarray + typenum to make this use the number of tasks as a parameter
/// runner for tasks
/// `N` specifies the number of tasks to ensure that the user
/// doesn't forget how many tasks they wished to add.
pub struct TaskRunner
// <
//     const N: usize,
// >
{
    /// this is the most noncommital thing ever
    /// as we're allowing entirely different generics for each task.
    tasks: Vec<(
        HotShotTaskId,
        String,
        BoxFuture<'static, HotShotTaskCompleted<dyn TaskErr>>,
    )>,
    /// global registry
    pub registry: GlobalRegistry,
}

impl TaskRunner /* <N> */ {
    /// create new runner
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            registry: GlobalRegistry::new(),
        }
    }

    /// to support builder pattern
    // pub fn add_task<HSTT: HotShotTaskTypes<Error = (dyn TaskErr + 'static)>>(&mut self, id: HotShotTaskId, name: String, builder: TaskBuilder<HSTT>) -> TaskRunner<N+1>{
    pub fn add_task<HSTT: HotShotTaskTypes<Error = dyn TaskErr + 'static>>(
        mut self,
        id: HotShotTaskId,
        name: String,
        builder: TaskBuilder<HSTT>,
    ) -> TaskRunner {
        self.tasks
            .push((id, name, HSTT::build(builder).launch().boxed()));
        self
    }

    /// returns a `Vec` because type isn't known
    pub async fn launch(self) -> Vec<(String, HotShotTaskCompleted<dyn TaskErr>)> {
        let names = self
            .tasks
            .iter()
            .map(|(_id, name, _)| name.clone())
            .collect::<Vec<_>>();
        let result = join_all(self.tasks.into_iter().map(|(_, _, task)| task)).await;

        names.into_iter().zip(result).collect::<Vec<_>>()
    }
}