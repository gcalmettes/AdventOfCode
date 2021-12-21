use std::collections::HashMap;
use itertools::iproduct;

fn regular_roll(start: usize) -> usize {
    let end = start + 3;
    (end * (end + 1) / 2) - (start * (start + 1) / 2)
}

fn play_quantum(cache: &mut HashMap<(usize,usize,usize,usize,usize),(usize,usize)>, starting_spots_and_scores: [usize; 4], player: usize) -> (usize, usize) {

    let [pos1, pos2, score1, score2] = starting_spots_and_scores;
    if score1 >= 21 {
        return (1, 0)
    }
    if score2 >= 21 {
        return (0, 1)
    }
    if let Some(&wins) = cache.get(&(pos1, pos2, score1, score2, player)) {
        return wins;
    }

    let mut wins = (0, 0);
        for (a, b, c) in iproduct!(1..=3, 1..=3, 1..=3) {
            let roll = a+b+c;
            let mut spots_and_scores = starting_spots_and_scores.clone();
            spots_and_scores[player] += roll;
            spots_and_scores[player] = ((spots_and_scores[player] - 1) % 10) + 1;
            spots_and_scores[player + 2] += spots_and_scores[player];
            let (win1, win2) = play_quantum(cache, spots_and_scores.clone(), (player + 1) % 2 );
            wins.0 += win1;
            wins.1 += win2;
        }
    cache.insert((pos1, pos2, score1, score2, player), wins);
    wins
}

fn part1() -> usize {
    let mut players_scores: Vec<usize> = vec![0, 0];
    let mut players_spots: Vec<usize> = vec![6, 4];

    let mut die_count = 0;

    'game: loop {
        for i in 0..=1 {
            let score = regular_roll(die_count);
            die_count += 3;
            players_spots[i] += score;
            players_spots[i] = ((players_spots[i] - 1) % 10) + 1;
            players_scores[i] += players_spots[i];
            if players_scores[i] >= 1000 {
                break 'game;
            }

        }
    }
    die_count * players_scores.iter().min().unwrap()
}

fn part2() -> usize {
    let wins = play_quantum(&mut HashMap::new(), [6, 4, 0, 0], 0);
    if wins.0 > wins.1 {wins.0} else {wins.1}
}

fn main() {
    let p1 = part1();
    println!("{}", p1);
    let p2 = part2();
    println!("{}", p2);
}
