mod graph;
use graph::Graph;

fn main() -> anyhow::Result<()> {
    let graph = graph::GraphBuilder::new(4)
        .add_edge(0, 1, 8)?
        .add_edge(0, 2, 4)?
        .add_edge(1, 3, 2)?
        .add_edge(2, 2, 2)?
        .add_edge(2, 3, 5)?
        .add_edge(3, 0, 2)?
        .build_mat();

    println!("{}", graph);
    println!("{}", graph.contains_odd_dominated_loop());
    Ok(())
}
