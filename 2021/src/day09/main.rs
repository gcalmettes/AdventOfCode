use std::path::Path;
use std::fs;
use std::collections::HashMap;

fn parse_input(content: &str) -> HashMap<(usize, usize), usize> {

    let mut map = HashMap::new();
    
    const RADIX: u32 = 10;

    content
        .lines()
        .enumerate()
        .for_each(|(y, line)| line.chars()
             .enumerate()
             .for_each(|(x, v)| {
                    let value = v.to_digit(RADIX).unwrap() as usize;
                    map.insert((x, y), value);
             }));
    map
}

fn get_neighbors(coords: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    neighbors.push((coords.0, coords.1 + 1));
    neighbors.push((coords.0 + 1, coords.1));
    // prevent panic overflow for unsigned
    if coords.1 != 0 {
        neighbors.push((coords.0, coords.1 - 1));
    };
    if coords.0 != 0 {
        neighbors.push((coords.0 - 1, coords.1));
    }
    neighbors
}

fn part1(input: &str) -> usize {
    let seamap = parse_input(input);
    
    seamap.iter()
        .filter(|(coord, val)| {
            let neighbors = get_neighbors(**coord);
            neighbors.iter()
                .filter_map(|c| seamap.get(c))
                .all(|c| c > val)
                })
        .map(|(_, val)| val + 1)
        .sum()
}

fn crawl_neighbors(
    coords: &(usize, usize),
    seamap: &HashMap<(usize, usize), usize>,
    basin: &mut Vec<HashMap<(usize, usize), bool>>,
    index: usize,
    ) {

    let ref_value = seamap.get(coords).unwrap();

    let neighbors = get_neighbors(*coords);
    let _valid_neighbors = neighbors.iter()
        .filter_map(|c| {
            let value = seamap.get(c);
            match value {
                Some(value) => Some((c, value)),
                None => None,
            }
        })
        .filter(|(_, v)| **v != 9 && v >= &ref_value)
        .map(|(c, _)| *c)
        .for_each(|c| {
            let v = basin[index].get(&c);
            // only crawl if we did not visit this coord already
            match v {
                Some(_) => (),
                None => {
                    basin[index].entry(c).or_insert(true);
                    crawl_neighbors(&c, seamap, basin, index)
                },
            }
        });
}

fn part2(input: &str) -> usize {
    let seamap = parse_input(input);
    
    let low_points = seamap.iter()
        .filter(|(coord, val)| {
            let neighbors = get_neighbors(**coord);
            neighbors.iter()
                .filter_map(|c| seamap.get(c))
                // .collect::<Vec<_>>();
                .all(|c| c > val)
                })
        .map(|(coords, _)| coords)
        .collect::<Vec<&(usize, usize)>>();

    let mut basins: Vec<HashMap<(usize, usize), bool>> = vec![];

    low_points.iter()
        .enumerate()
        .for_each(|(i, c)| {
            // println!("------- low point {:?}", c);
            let mut b = HashMap::new();
            b.insert(**c, true);
            basins.push(b);
            crawl_neighbors(c, &seamap, &mut basins, i);
            ()
        });

    let mut sizes = basins.iter()
        .map(|b| b.len())
        .collect::<Vec<usize>>();
    
    sizes.sort_by(|a, b| b.cmp(a));

    let res = &sizes[0..3].iter()
        .fold(1, |acc, v| acc * v);

    *res
}

fn main() {
    let path = Path::new("./inputs/day09.txt");
    let input = fs::read_to_string(path).expect("Unable to read file");

    let p1 = part1(&input);
    println!("{}", p1);
    let p2 = part2(&input);
    println!("{}", p2);
}
