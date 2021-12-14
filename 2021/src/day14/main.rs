use std::path::Path;
use std::fs;
use std::collections::HashMap;

fn parse_input(content: &str) -> (HashMap<String, usize>, HashMap<&str, (String, String)>) {
    let parts = content
        .split("\n\n")
        .collect::<Vec<&str>>();
    
    let mut template: HashMap<String, usize> = HashMap::new();
    for i in 0..parts[0].len()-1 {
        let t = &parts[0][i..i+2];
        template.insert(t.to_string(), 1);
    };

    let mut rules: HashMap<&str, (String, String)> = HashMap::new();
    parts[1].lines()
        .for_each(|line| {
            let insertion = line.split_once(" -> ").unwrap();
            let c1 = insertion.0.chars().nth(0).unwrap().to_string() + insertion.1;
            let c2 = insertion.1.to_owned() + &insertion.0.chars().nth(1).unwrap().to_string();
            rules.insert(insertion.0, (c1, c2));
        });
    
    (template, rules)
}


fn part(input: &str, n: usize) -> usize {

    let (template, rules) = parse_input(&input);
    let mut template = template;

    for _ in 0..n {
        let pairs = template.iter()
            .filter(|(_, v)| **v > 0)
            .map(|(k, _)| k)
            .cloned()
            .collect::<Vec<_>>();

        let values = template.iter()
            .filter(|(_, v)| **v > 0)
            .map(|(_, v)| v)
            .cloned()
            .collect::<Vec<_>>();

        pairs.iter()
            .enumerate()
            .for_each(|(i, key)| {
                let children = rules.get(key.as_str()).unwrap();
                let n = values[i];
                *template.entry(children.0.to_string()).or_insert(0) += n;
                *template.entry(children.1.to_string()).or_insert(0) += n;
                *template.entry(key.to_string()).or_insert(0) -= n;
            });
    }

    let mut counts: HashMap<char, usize> = HashMap::new();
    template.iter()
        .for_each(|(k, v)| {
            k.chars().for_each(|c| *counts.entry(c).or_insert(0) += v)
        });

    let first = 'C';
    let last = 'O';
    *counts.entry(first).or_insert(0) += 1;
    *counts.entry(last).or_insert(0) += 1;
    counts.iter_mut()
        .for_each(|(_, v)| *v /= 2);

    let min = counts.values().min().unwrap();
    let max = counts.values().max().unwrap();

    max - min
}

fn main() {
    let path = Path::new("./inputs/day14.txt");
    let input = fs::read_to_string(path).expect("Unable to read file");

    let p1 = part(&input, 10);
    println!("{}", p1);
    let p2 = part(&input, 40);
    println!("{}", p2);
}
