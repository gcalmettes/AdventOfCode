use std::path::Path;
use std::fs;
use std::collections::HashMap;
use fast_paths::InputGraph;

fn parse_input(content: &str) -> (HashMap<(usize, usize), usize>, HashMap<usize, (usize, usize)>, InputGraph) {
    let mut data: HashMap<(usize, usize), usize> = HashMap::new();

    // data
    content
        .lines()
        .enumerate()
        .for_each(|(y, line)| {
            line.chars()
                .enumerate()
                .for_each(|(x, c)| {
                    let n = c.to_string().parse().unwrap();
                    data.insert((x, y), n);
                })
        });

    // content
    //     .lines()
    //     .enumerate()
    //     .for_each(|(y, line)| {
    //         line.chars()
    //             .zip(1..=line.len())
    //             .enumerate()
    //             .for_each(|(x1, (c, x2))| {
    //                 let n = c.to_string().parse().unwrap();
    //                 let idx1 = x1 + y * line.len();
    //                 let idx2 = x2 + y * line.len();
    //                 mapping.insert(idx1, (x1, y));
    //                 mapping.insert(idx2, (x2, y));
    //                 input_graph.add_edge(idx1, idx2, n);
    //             })
    //     });
    let mut mapping: HashMap<usize, (usize, usize)> = HashMap::new();
    let mut input_graph = InputGraph::new();
    let n_rows = content.lines().next().unwrap().len();
    let n_cols = content.lines().collect::<Vec<_>>().len();
    
    // rows wise
    for y in 0..n_rows {
        for x in 0..n_cols {
            let weight1 = data.get(&(x, y)).unwrap();
            let idx1 = x + y * n_rows + y;
            let idx2 = x + 1 + y * n_rows + y;
            mapping.insert(idx1, (x, y));
            mapping.insert(idx2, (x + 1, y));
            input_graph.add_edge(idx1, idx2, *weight1);

            if y < n_rows-1 {
                let weight2 = data.get(&(x, y + 1)).unwrap();
                let idx3 = x + y * n_rows + y + (n_cols + 1);
                mapping.insert(idx3, (x, y + 1));
                input_graph.add_edge(idx1, idx3, *weight2);
                // println!("{} {} {} - {} {} {}", idx1, idx2, weight1, idx1, idx3, weight2);
            }
        }
    }

    // // columns wise
    // for y in 0..n_rows {
    //     for x in 0..n_cols {
    //         let weight = data.get(&(x, y)).unwrap();
    //         let idx1 = x + y * n_rows + y;
    //         let idx2 = x + 1 + y * n_rows + y;
    //         input_graph.add_edge(idx1, idx2, *weight);
    //         // println!("{} {} {}", idx1, idx2, weight);
    //     }
    // }

    // column wise
    // content
    //     .lines()
    //     .enumerate()
    //     .for_each(|(y, line)| {
    //         line.chars()
    //             .zip(1..=line.len())
    //             .enumerate()
    //             .for_each(|(x1, (c, x2))| {
    //                 let n = c.to_string().parse().unwrap();
    //                 let idx1 = x1 + y * line.len();
    //                 let idx2 = x2 + y * line.len();
    //                 mapping.insert(idx1, (x1, y));
    //                 mapping.insert(idx2, (x2, y));
    //                 input_graph.add_edge(idx1, idx2, n);
    //             })
    //     });
       (data, mapping, input_graph)
}


fn part1(input: &str) -> usize {

    let (_data, _mapping, mut input_graph) = parse_input(input);

    input_graph.freeze();
    let fast_graph = fast_paths::prepare(&input_graph);
    let shortest_path = fast_paths::calc_path(&fast_graph, 0, 108);

    match shortest_path {
        Some(p) => {
            // the weight of the shortest path
            let weight = p.get_weight();
            println!("{:?}", weight);
            
            // all nodes of the shortest path (including source and target)
            let nodes = p.get_nodes();
            println!("{:?}", nodes);
        },
        None => {
            // no path has been found (nodes are not connected in this graph)
        }
    }
    0
}

fn main() {
    let path = Path::new("./inputs/day15.txt");
    let input = fs::read_to_string(path).expect("Unable to read file");

    let p1 = part1(&input);
    println!("{}", p1);
}
