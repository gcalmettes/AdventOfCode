use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

fn component_size(graph: &HashMap<&str, HashSet<&str>>, a: &str) -> usize {
    let (mut seen, mut s) = (HashSet::new(), vec![a]);
    while let Some(x) = s.pop() {
        if seen.insert(x) {
            s.extend(&graph[x]);
        }
    }
    seen.len()
}

#[aoc::main()]
fn main(input: &str) -> (usize, &str) {
    let mut graph = HashMap::<_, HashSet<_>>::new();
    let mut edges = HashSet::new();
    for l in input.split("\n") {
        let (a, rest) = l.split_once(": ").unwrap();
        for b in rest.split(" ") {
            graph.entry(a).or_default().insert(b);
            graph.entry(b).or_default().insert(a);
            edges.insert(if a < b { (a, b) } else { (b, a) });
        }
    }

    let mut dot = String::from("graph {\n");
    for (left, right) in edges.iter().sorted() {
        dot += &format!("  {} -- {};\n", left, right);
    }
    dot += "}";
    // generate a visualization for the graph:
    // https://graphviz.org/doc/info/command.html
    //   dot -Tsvg -Kneato img/25.dot > img/25.svg
    std::fs::write("img/25.dot", dot).unwrap();

    // Pairs detected in graph:
    for (a, b) in [("ssd", "xqh"), ("nrs", "khn"), ("mqb", "qlc")] {
        graph.get_mut(a).unwrap().remove(b);
        graph.get_mut(b).unwrap().remove(a);
    }
    let size = component_size(&graph, "qqq");
    ((graph.len() - size) * size, "🦌🎄🎅")
}
