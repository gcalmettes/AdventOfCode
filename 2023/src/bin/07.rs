use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Debug)]
struct Hand {
    cards: [usize; 5],
    bid: usize,
    original_cards: [usize; 5],
}

impl Hand {
    fn from_str(line: &str, p2: bool) -> Self {
        let (cards, bid) = line.split_once(" ").unwrap();
        let cards = cards.chars().enumerate().fold([0; 5], |mut acc, (i, c)| {
            acc[i] = to_digit(c, p2);
            acc
        });
        let bid = bid.parse::<usize>().ok().unwrap();
        Hand {
            cards,
            bid,
            original_cards: cards.clone(),
        }
    }
    fn compare_cards(&self, other: &Hand) -> Ordering {
        let mut i = 0;
        let (mut c1, mut c2) = (self.original_cards[i], other.original_cards[i]);
        while c1 == c2 {
            i += 1;
            (c1, c2) = (self.original_cards[i], other.original_cards[i]);
        }
        c1.cmp(&c2)
    }

    fn jokerize(&self) -> Self {
        let mut self_cards = self.cards.clone();

        (0..self_cards.len()).for_each(|i| {
            if self_cards[i] == 0 {
                let max_groups = self_cards
                    .into_iter()
                    .into_group_map_by(|a| *a)
                    .iter()
                    .sorted_unstable_by_key(|(_k, v)| v.len())
                    .filter(|(_k, v)| v.len() > 1)
                    .map(|(k, _v)| k.clone())
                    .collect::<Vec<usize>>();

                let max_card = *self_cards.iter().max().unwrap();

                let replace_by = match max_groups.len() > 0 {
                    true => {
                        let mut i = 0;
                        let mut max_group = max_groups[i];
                        // we do not want to replace a joker by a joker
                        while max_group == 0 {
                            i += 1;
                            if i > max_groups.len() - 1 {
                                max_group = max_card;
                                break;
                            }
                            max_group = max_groups[i];
                        }
                        max_group
                    }
                    false => max_card,
                };
                self_cards[i] = replace_by;
            }
        });
        Hand {
            cards: self_cards,
            original_cards: self.original_cards,
            bid: self.bid,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_groups = self.cards.into_iter().into_group_map_by(|a| *a);
        let other_groups = other.cards.into_iter().into_group_map_by(|a| *a);

        let self_max = self_groups.iter().map(|(_k, v)| v.len()).max().unwrap();
        let other_max = other_groups.iter().map(|(_k, v)| v.len()).max().unwrap();

        match (self_max, other_max) {
            // 5 of a kind
            (5, 5) => self.compare_cards(&other),
            (5, _) => Ordering::Greater,
            (_, 5) => Ordering::Less,
            // 4 of a kind
            (4, 4) => self.compare_cards(&other),
            (4, _) => Ordering::Greater,
            (_, 4) => Ordering::Less,
            (_, _) => {
                // full (3, 2)
                let self_full = self_groups.len() == 2;
                let other_full = other_groups.len() == 2;
                match (self_full, other_full) {
                    (true, true) => self.compare_cards(&other),
                    (true, false) => Ordering::Greater,
                    (false, true) => Ordering::Less,
                    (_, _) => {
                        if self_max != other_max {
                            self_max.cmp(&other_max)
                        } else {
                            // check for double pairs
                            let self_double =
                                self_groups.iter().filter(|(_k, v)| v.len() == 2).count();
                            let other_double =
                                other_groups.iter().filter(|(_k, v)| v.len() == 2).count();

                            if self_double != other_double {
                                self_double.cmp(&other_double)
                            } else {
                                self.compare_cards(&other)
                            }
                        }
                    }
                }
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn to_digit(card: char, p2: bool) -> usize {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => {
            if p2 {
                0
            } else {
                11
            }
        }
        'T' => 10,
        c => c.to_digit(10).unwrap() as usize,
    }
}

fn parse_input(input: &str, p2: bool) -> Vec<Hand> {
    input.split("\n").map(|h| Hand::from_str(h, p2)).collect()
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let hands_p1 = parse_input(input, false);
    let hands_p2 = parse_input(input, true)
        .into_iter()
        .map(|h| h.jokerize())
        .collect::<Vec<Hand>>();

    let p1 = hands_p1
        .iter()
        .sorted()
        .enumerate()
        .map(|(r, h)| h.bid * (r + 1))
        .sum();

    let p2 = hands_p2
        .iter()
        .sorted()
        .enumerate()
        .map(|(r, h)| h.bid * (r + 1))
        .sum();

    (p1, p2)
}
