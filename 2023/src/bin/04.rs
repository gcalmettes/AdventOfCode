use std::collections::HashMap;

struct ScratchCard {
    id: usize,
    winning: Vec<usize>,
    numbers: Vec<usize>,
}

impl ScratchCard {
    fn matching_numbers_count(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|n| self.winning.contains(n))
            .count() as u32
    }

    fn score(&self) -> usize {
        let n = self.matching_numbers_count();
        let base: i32 = 2;
        match n > 0 {
            true => base.pow(n - 1) as usize,
            false => 0,
        }
    }

    fn cards_id_won(&self) -> Vec<usize> {
        let n = self.matching_numbers_count() as usize;
        ((self.id + 1)..=(self.id + n)).collect::<Vec<usize>>()
    }
}

fn parse_numbers(numbers: &str) -> Vec<usize> {
    numbers
        .split_whitespace()
        .filter_map(|n| n.parse::<usize>().ok())
        .collect::<Vec<usize>>()
}

fn parse(input: &str) -> Vec<ScratchCard> {
    input
        .split("\n")
        .map(|line| {
            let (winning, numbers) = line.split_once(" | ").unwrap();
            let (id, winning) = winning.split_once(": ").unwrap();
            let (_, id) = id.split_once(" ").unwrap();
            let id = id.trim().parse::<usize>().unwrap();
            let winning = parse_numbers(winning);
            let numbers = parse_numbers(numbers);
            ScratchCard {
                id,
                winning,
                numbers,
            }
        })
        .collect::<Vec<ScratchCard>>()
}

fn play(cards: &Vec<ScratchCard>) -> usize {
    let cardsmap = cards
        .iter()
        .map(|c| (c.id, c.cards_id_won()))
        .collect::<HashMap<usize, Vec<usize>>>();
    let mut pile: Vec<usize> = cards.iter().map(|c| c.id).collect();
    let mut leftover: Vec<usize> = Vec::new();
    while let Some(card) = pile.pop() {
        let won = cardsmap.get(&card).unwrap();
        if won.len() > 0 {
            pile.extend(won);
            leftover.push(card);
        } else {
            leftover.push(card);
        }
    }
    leftover.len()
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let scratchcards = parse(input);
    let p1 = scratchcards.iter().map(|c| c.score()).sum();
    let p2 = play(&scratchcards);
    (p1, p2)
}
