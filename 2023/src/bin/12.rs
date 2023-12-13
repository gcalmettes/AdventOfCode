use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
struct Record {
    data: Vec<char>,
    pattern: Vec<usize>,
}

impl Record {
    fn unfold(&self, times: usize) -> Self {
        Record {
            data: (0..times)
                .flat_map(|i| match i {
                    x if x == times - 1 => self.data.clone(),
                    _ => {
                        let mut d = self.data.clone();
                        d.push('?');
                        d
                    }
                })
                .collect::<Vec<char>>(),
            pattern: (0..times)
                .flat_map(|_| self.pattern.clone())
                .collect::<Vec<usize>>(),
        }
    }

    fn arrangements(&self, cache: &mut HashMap<(String, String, usize), usize>) -> usize {
        Record::is_valid(cache, &self.data, &self.pattern, None)
    }

    fn is_valid(
        cache: &mut HashMap<(String, String, usize), usize>,
        blocks: &[char],
        pattern_to_match: &[usize],
        current_pos: Option<usize>,
    ) -> usize {
        if blocks.is_empty() {
            return match (pattern_to_match.len(), current_pos) {
                // nothing left, it's a match
                (0, None) => 1,
                // only one pattern left and it corresponds to the current block
                (1, Some(v)) if v == pattern_to_match[0] => 1,
                (_, _) => 0,
            };
        }
        if current_pos.is_some() && pattern_to_match.is_empty() {
            return 0;
        }

        let key = (
            blocks.iter().collect::<String>(),
            pattern_to_match.iter().join("-"), // blocks.len(),
            current_pos.unwrap_or(0),
        );
        if let Some(n) = cache.get(&key) {
            return *n;
        }

        let n = match (blocks[0], current_pos) {
            // we left a block but length is not enough
            ('.', Some(v)) if v != pattern_to_match[0] => 0,
            // we left a block and length is possible, go on to next block
            ('.', Some(_)) => Record::is_valid(cache, &blocks[1..], &pattern_to_match[1..], None),
            // we were not in a block, we continue
            ('.', None) => Record::is_valid(cache, &blocks[1..], pattern_to_match, None),
            // we are inside a block, let's continue
            ('#', Some(_)) => Record::is_valid(
                cache,
                &blocks[1..],
                pattern_to_match,
                current_pos.and_then(|i| Some(i + 1)),
            ),
            // we enter a new block, continue
            ('#', None) => Record::is_valid(cache, &blocks[1..], pattern_to_match, Some(1)),
            // we are within a block and we have a joker
            ('?', Some(v)) => {
                // let's try to continue on
                let mut n = Record::is_valid(
                    cache,
                    &blocks[1..],
                    pattern_to_match,
                    current_pos.and_then(|i| Some(i + 1)),
                );
                // if we have the right lenght for the current block, let's continue to the next
                // pattern to match
                if v == pattern_to_match[0] {
                    n += Record::is_valid(cache, &blocks[1..], &pattern_to_match[1..], None)
                }
                n
            }
            // the world of possibilities is opened
            ('?', None) => {
                // we try both possibilities, starting a block or moving on to next character
                Record::is_valid(cache, &blocks[1..], pattern_to_match, Some(1))
                    + Record::is_valid(cache, &blocks[1..], pattern_to_match, None)
            }
            _ => unreachable!(),
        };
        cache.insert(key, n);
        n
    }
}

fn parse_records(input: &str) -> Vec<Record> {
    input
        .lines()
        .map(|line| {
            let (data, pattern) = line.split_once(" ").unwrap();
            let data = data.chars().collect::<Vec<char>>();
            let pattern = pattern
                .split(",")
                .filter_map(|d| d.parse::<usize>().ok())
                .collect::<Vec<usize>>();
            Record { data, pattern }
        })
        .collect()
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let records = parse_records(input);

    let mut cache: HashMap<(String, String, usize), usize> = HashMap::new();

    cache.clear();
    let p1 = records.iter().map(|r| r.arrangements(&mut cache)).sum();
    cache.clear();

    let p2 = records
        .iter()
        .map(|r| r.unfold(5))
        .map(|r| r.arrangements(&mut cache))
        .sum();
    (p1, p2)
}
