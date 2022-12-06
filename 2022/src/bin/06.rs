use std::collections::HashSet;

fn get_marker_position(data: &str, window_size: usize) -> usize {
    let len = data.len();
    let mut n: usize = 0;
    for i in window_size..(len - window_size) {
        let uniq: HashSet<&u8> = HashSet::from_iter(&data.as_bytes()[i - window_size..i]);
        if uniq.len() == window_size {
            n = i;
            break;
        } else {
            continue;
        }
    }
    n
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let p1 = get_marker_position(input, 4);
    let p2 = get_marker_position(input, 14);
    (p1, p2)
}
