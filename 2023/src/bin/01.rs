fn str_to_digit(number_str: &str) -> usize {
    match number_str {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => unreachable!(),
    }
}

fn parse_calibration(line: &str, numbers_str: &Vec<String>) -> usize {
    let mut numerics = (0..line.len())
        .filter_map(|i| {
            numbers_str
                .iter()
                .find_map(|num_str| match line[i..].starts_with(num_str) {
                    true => Some(num_str),
                    false => None,
                })
        })
        .map(|n| match n.parse::<usize>() {
            Ok(d) => char::from_digit(d as u32, 10).unwrap(),
            Err(_) => char::from_digit(str_to_digit(n) as u32, 10).unwrap(),
        })
        .collect::<Vec<_>>();

    // only one detection, double it
    if numerics.len() == 1 {
        numerics.push(numerics[0]);
    };
    // multiple detection, take first and last
    if numerics.len() > 2 {
        let (first, last) = (numerics.first().unwrap(), numerics.last().unwrap());
        numerics = vec![*first, *last];
    };

    numerics
        .iter()
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let numbers_str = (0..=9)
        .map(|x| x.to_string())
        .chain(
            [
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ]
            .iter()
            .map(|s| s.to_string()),
        )
        .collect::<Vec<String>>();

    let p1 = input
        .split("\n")
        .map(|line| parse_calibration(line, &numbers_str[..=9].to_vec()))
        .sum::<usize>();

    let p2 = input
        .split("\n")
        .map(|line| parse_calibration(line, &numbers_str))
        .sum::<usize>();

    (p1, p2)
}
