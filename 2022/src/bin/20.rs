fn parse_input(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
}

fn decrypt_file(file: &[i64], iterations: usize) -> i64 {
    let mut indices = (0..file.len()).collect::<Vec<_>>();
    for _ in 0..iterations {
        for (idx, &num) in file.iter().enumerate() {
            let pos = indices.iter().position(|&p| p == idx).unwrap();
            indices.remove(pos);
            let target_pos = (pos as i64 + num).rem_euclid(indices.len() as i64) as usize;
            indices.insert(target_pos, idx);
        }
    }

    const TARGET: i64 = 0;
    // get current position of target
    let target_starting_idx = file.iter().position(|&i| i == TARGET).unwrap();
    let target_idx = indices
        .iter()
        .position(|&i| i == target_starting_idx)
        .unwrap();

    [1000, 2000, 3000]
        .iter()
        // retrieve corresponding relative index
        .map(|i| indices[(target_idx + i) % indices.len()])
        // and retrieve corresponding number from original file
        .map(|i| file[i])
        .sum()
}

fn part1(file: &Vec<i64>) -> i64 {
    decrypt_file(file, 1)
}

fn part2(file: &Vec<i64>) -> i64 {
    let decryption_key = 811589153;
    let file = file.iter().map(|x| x * decryption_key).collect::<Vec<_>>();
    decrypt_file(&file, 10)
}

#[aoc::main()]
fn main(input: &str) -> (i64, i64) {
    let data = parse_input(input);
    let p1 = part1(&data);
    let p2 = part2(&data);
    (p1, p2)
}
