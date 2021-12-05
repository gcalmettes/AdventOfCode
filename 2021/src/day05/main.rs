use std::path::Path;
use std::fs;
use std::collections::HashMap;


fn parse_input(content: &str) -> Vec<(i32, i32, i32, i32)> {
    let fissures = content
        .lines()
        .map(|entry| entry.split(" -> ")
                .flat_map(|coord| coord.split(","))
                .map(|i| i.parse().unwrap())
                .collect::<Vec<_>>())
        .map(|coords| {
            let t: (i32, i32, i32, i32) = (coords[0], coords[1], coords[2], coords[3]);
            t
        })
        .collect();
       fissures

}

fn get_coords(fissures:Vec<(i32, i32, i32, i32)>) -> HashMap<(i32, i32), usize>{
    let mut coords = HashMap::new();

    for (x1, y1, x2, y2) in &fissures {
        // get the signed step directly
        let dx = (x2-x1).signum();
        let dy = (y2-y1).signum();
        
        let (mut x, mut y) = (*x1, *y1);

        // println!("x: {} -> {} -> {}", x1, dx, x2);
        // println!("y: {} -> {} -> {}", y1, dy, y2);

        while (x,y) != (x2+dx, y2+dy) {
            let v = coords.entry((x,y)).or_insert(0);
            let new_v = *v + 1;
            coords.insert((x,y), new_v);

            x += dx;
            y += dy;
        }
    }

    coords
}

fn part1(input: &str) -> usize {

    let fissures = parse_input(input)
        .iter()
        .cloned()
        .filter(|(x1, y1, x2, y2)| x1==x2 || y1==y2)
        .collect::<Vec<(i32, i32, i32, i32)>>();

    let coords = get_coords(fissures);

    coords.values().filter(|&&v| v > 1).count()
}


fn part2(input: &str) -> usize {

    let fissures = parse_input(input);

    let coords = get_coords(fissures);

    coords.values().filter(|&&v| v > 1).count()
}


fn main() {
    let path = Path::new("./inputs/day05.txt");
    let input = fs::read_to_string(path).expect("Unable to read file");

    let p1 = part1(&input);
    println!("{}", p1);
    let p2 = part2(&input);
    println!("{}", p2);
}
