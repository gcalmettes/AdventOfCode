use std::collections::HashSet;

fn get_marker_position(data: &str, window_size: usize) -> usize {
    let len = data.len();
    let mut n: usize = 0;
    for i in window_size..(len - 1) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_stop_marker() {
        let win = 4;
        assert_eq!(get_marker_position("bvwbjplbgvbhsrlpgdmjqwftvncz", win), 5);
        assert_eq!(get_marker_position("nppdvjthqldpwncqszvftbrmjlhg", win), 6);
        assert_eq!(
            get_marker_position("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", win),
            10
        );
    }

    #[test]
    fn find_start_marker() {
        let win = 14;
        assert_eq!(
            get_marker_position("mjqjpqmgbljsphdztnvjfqwrcgsmlb", win),
            19
        );
        assert_eq!(get_marker_position("bvwbjplbgvbhsrlpgdmjqwftvncz", win), 23);
        assert_eq!(
            get_marker_position("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", win),
            29
        );
    }
}
