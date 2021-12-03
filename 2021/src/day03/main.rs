use std::path::Path;
use std::fs;


fn parse_input(content: &str) -> Vec<Vec<u32>> {
    let binaries = content
        .lines()
        .map(|entry| {
            let digits: Vec<u32> = entry
                .chars()
                .map(|c| (c.to_string()).parse().unwrap())
                .collect();
            digits
        })
        .collect();

    return binaries
}

fn sum_column_wise(matrix: Vec<Vec<u32>>) -> Vec<u32> {
    let n_columns = matrix[0].len();

    let sums = matrix.iter()
        .fold(vec![0; n_columns], |mut sums, row| {
            sums.iter_mut()
                .zip(row.iter())
                .for_each(|(sum, cell)| *sum += cell);
            sums
    });

    sums
}


fn sum_one_column(matrix: Vec<Vec<u32>>, col: usize) -> usize {
    // Compute sum column wise
    let sum = matrix.iter()
        .fold(0, |mut sum, row| {sum += row[col]; sum});
    // println!("** {}", sum);
    sum as usize
}

fn part1() -> isize {

    let path = Path::new("./inputs/day03.txt");
    let input = fs::read_to_string(path).expect("Unable to read file");

    let binaries = parse_input(&input);
    let n_rows = binaries.len();
    
    let sums = sum_column_wise(binaries);


    let most_commons:Vec<usize> = sums.iter()
        .map(|x| ( (*x as usize) > n_rows / 2) as usize)
        .collect();

    let least_commons:Vec<usize> = most_commons.iter()
        .map(|x| 1 - x)
        .collect();

    // concat vec of digit into single digit
    let gamma_binary = most_commons.iter().fold(0, |acc, elem| acc * 10 + elem );
    let epsilon_binary = least_commons.iter().fold(0, |acc, elem| acc * 10 + elem );

    // transform to decimal
    let gamma = isize::from_str_radix(&gamma_binary.to_string(), 2).unwrap();
    let epsilon = isize::from_str_radix(&epsilon_binary.to_string(), 2).unwrap();

    gamma * epsilon

}


fn part2() -> isize {

    let path = Path::new("./inputs/day03.txt");
    let input = fs::read_to_string(path).expect("Unable to read file");

    let binaries = parse_input(&input);

    let mut oxygen = binaries.clone();

    for i in 0..binaries[0].len() {
        let n_rows = oxygen.len();
        let bit_most = {
            let sum_of_col = sum_one_column(oxygen.to_vec(), i);
            let mut to_compare_to = n_rows / 2;
            let rem = n_rows % 2;
            if rem > 0 {
                to_compare_to += 1
            };
            (sum_of_col >= to_compare_to) as u32
        };

        oxygen = oxygen.iter()
            .filter(|r| r[i] == bit_most)
            .cloned()
            .collect::<Vec<Vec<u32>>>();

        if oxygen.len() <= 1 {
            break
        }
    }

    let mut co2 = binaries.clone();

    for i in 0..binaries[0].len() {
        let n_rows = co2.len();

        let bit_least = {
            let sum_of_col = sum_one_column(co2.to_vec(), i);
            let to_compare_to = n_rows / 2;
            let rem = n_rows % 2;
            if (rem == 0) && (sum_of_col == to_compare_to) {
                 0
            } else {
                (sum_of_col <= to_compare_to) as u32
            }
        };

        co2 = co2.iter()
            .filter(|r| r[i] == bit_least)
            .cloned()
            .collect::<Vec<Vec<u32>>>();

        if co2.len() <= 1 {
            break
        }
    }

    // concat vec of digit into single digit
    let oxygen_binary = oxygen[0].iter().fold(0, |acc, elem| acc * 10 + (*elem as usize) );
    let co2_binary = co2[0].iter().fold(0, |acc, elem| acc * 10 + (*elem as usize) );

    // transform to decimal
    let oxygen_rating = isize::from_str_radix(&oxygen_binary.to_string(), 2).unwrap();
    let co2_rating = isize::from_str_radix(&co2_binary.to_string(), 2).unwrap();

    oxygen_rating * co2_rating
}

fn main() {

    let p1 = part1();
    println!("{}", p1);
    let p2 = part2();
    println!("{}", p2);
}
