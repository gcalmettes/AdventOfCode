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

// =============================================================================================
// OLD SOLUTION
// =============================================================================================

// use hashbrown::HashMap;
// use itertools::Itertools;

// fn order(file: &HashMap<usize, (usize, i64)>) -> Vec<i64> {
//     let nums = file
//         .iter()
//         .sorted_by(|a, b| a.0.cmp(&b.0))
//         .map(|(_, (_, v))| *v)
//         .collect();
//     nums
// }

// fn decrypt(input: &str, iterations: usize, decryption_key: i64) -> i64 {
//     let mut file = HashMap::new();
//     input.lines().enumerate().for_each(|(i, line)| {
//         let num = line.parse::<i64>().unwrap() * decryption_key;
//         file.insert(i, (i, num));
//     });

//     for i in 0..file.len() {
//         let payload = file[&i];
//         *file.get_mut(&i).unwrap() = payload;
//     }
//     for _ in 0..iterations {
//         for i in 0..file.len() {
//             // find current position of num
//             let (pos, (origin, shift_by)) = file.iter().find(|(_, (idx, _))| idx == &i).unwrap();
//             let payload = (*origin, *shift_by);

//             let target_pos =
//                 ((*pos as i64) + shift_by).rem_euclid((file.len() - 1) as i64) as usize;

//             if pos == &target_pos {
//                 // already is already in place
//                 continue;
//             } else if &target_pos > pos {
//                 // move all numbers in between
//                 (*pos..target_pos).for_each(|p| {
//                     *file.get_mut(&p).unwrap() = file[&(p + 1)];
//                 });
//                 // then move to target
//                 *file.get_mut(&target_pos).unwrap() = payload;
//             } else {
//                 // move all numbers in between
//                 let start = target_pos + 1;
//                 let end = *pos;
//                 (start..=end).rev().for_each(|p| {
//                     *file.get_mut(&p).unwrap() = file[&(p - 1)];
//                 });
//                 // then move to target
//                 *file.get_mut(&target_pos).unwrap() = payload;
//             }
//             // println!("{:?}", order(&file));
//         }
//     }
//     let nums = order(&file);
//     let zero_pos = nums.iter().position(|x| x == &(0 as i64)).unwrap();
//     [1000, 2000, 3000]
//         .iter()
//         .map(|i| (i + zero_pos) % file.len())
//         .map(|i| nums[i])
//         .sum()
// }

// #[aoc::main()]
// fn main(input: &str) -> (i64, i64) {
//     let p1 = decrypt(input, 1, 1);
//     let p2 = decrypt(input, 10, 811589153);
//     (p1, p2)
// }
