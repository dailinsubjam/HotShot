use plotters::prelude::*;

// time (s), data (bytes), rate limit
pub const RAW_THROUGHPUTS: [(f64, f64, u64); 4] = [
    (37.509553384, 3203300.0, 100),
    (56.49525265, 3203300.0, 50),
    (117.991896265, 3203300.0, 25),
    (371.381557543, 3203300.0, 12)
];


// 32000 bytes / message
// 100 messages


fn main() {
    // gets us bytes / s
    let throughputs = RAW_THROUGHPUTS.iter().map(|(time, data, rate)| {
        // now in kb
        ( *rate as i32, (((*data) / 1000.0)/(*time)) as i32)
    }).collect::<Vec<_>>();

    println!("{:?}", throughputs);



    let root_area = BitMapBackend::new("/Users/jrestivo/dev/work/VERYTMP/phaselock-plots/2.6.png", (600, 400))
    .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("30 node throughput vs throttle", ("sans-serif", 40))
        .build_cartesian_2d(0..120, 0..120)
        .unwrap();

    ctx.configure_mesh()
       .axis_desc_style(("sans-serif", 15))
       .y_desc("Throughput (Kb/s)")
       .x_desc("Throttle (Mbit)")
       .draw().unwrap();

    ctx.draw_series(
        throughputs.iter().map(|point| TriangleMarker::new(*point, 5, &BLUE)),
    )
    .unwrap();
}


