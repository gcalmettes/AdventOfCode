use std::path::Path;
use std::fs;
use std::collections::HashMap;


fn parse_input(content: &str) -> HashMap<usize, isize> {

    let fishs: HashMap<usize, isize> = content
        .lines()
        .flat_map(|entry| entry.split(",")
                .map(|i| i.parse().unwrap())
                .collect::<Vec<usize>>())
        .enumerate()
        .fold(HashMap::new(), |mut acc, (i, d)| {acc.insert(i, d as isize); acc});
       fishs

}


fn part1(input: &str) -> usize {

    let mut fishs = parse_input(input);
    
    let mut to_add = 0;

    for _day in 0..80 {

        let keys: Vec<usize> = fishs.keys().cloned().collect();
        let mut n_fishs = fishs.len();

        if to_add > 0 {
            for _i in 0..to_add {
                fishs.insert(n_fishs, 8);
                n_fishs += 1
            }
        } 
        
        to_add = 0;

        for key in keys {


            let v = fishs.entry(key).or_insert(0);
            let mut v = *v - 1;
            if v < 0 {
                v = 6
            }
            fishs.insert(key, v);
            if v == 0 {
                to_add += 1;
            }
        }

        // let mut f: Vec<_> = fishs.iter().collect();
        // f.sort_by(|x,y| x.0.cmp(&y.0));
        // println!("{:?}", f);
    }

    fishs.len()
}

fn part2(input: &str) -> usize {

    // Change of strategy, let's store the state of each cycle point.
    let mut state: HashMap<usize, usize> = HashMap::new();

    // Inital state.
    for i in 0..9 {
        state.insert(i, 0);
    }

    // Let's fill in the state based on the input.
    input
        .lines()
        .flat_map(|entry| entry.split(",")
                .map(|i| i.parse().unwrap())
                .collect::<Vec<usize>>())
        .for_each(|v| {
            let n = state.entry(v).or_insert(0);
            let n = *n + 1;
            state.insert(v, n);
        });
    
    for _day in 0..256 {
        
        // New fishes to be created.
        let new = state.get(&0).unwrap().clone();

        // Update the old generation.
        for key in 0..6 {
            let n_generation = state.get(&(key + 1)).unwrap().clone();
            state.insert(key, n_generation);
        }
        let n_seven = state.get(&7).unwrap().clone();
        state.insert(6, new + n_seven);
        
        // Update the new generation.
        let n_height = state.get(&8).unwrap().clone();
        state.insert(7, n_height);
        state.insert(8, new);

        // let n: usize = state.values().sum();
        // let mut f: Vec<_> = state.iter().collect();
        // f.sort_by(|x,y| x.0.cmp(&y.0));
        // println!("{} -- {} -- {:?}", day, n, f);
    }
    
    state.values().sum()
}


fn main() {
    let path = Path::new("./inputs/day06.txt");
    let input = fs::read_to_string(path).expect("Unable to read file");

    let p1 = part1(&input);
    println!("{}", p1);
    let p2 = part2(&input);
    println!("{}", p2);
}
