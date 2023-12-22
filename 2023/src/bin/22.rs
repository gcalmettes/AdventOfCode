use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Block {
    xmin: isize,
    xmax: isize,
    ymin: isize,
    ymax: isize,
    zmin: isize,
    zmax: isize,
}

impl Block {
    fn from_str(s: &str) -> Block {
        let (start, end) = s.split_once("~").unwrap();
        let mut start = start.split(",");
        let mut end = end.split(",");
        Block {
            xmin: start.next().map(|d| d.parse::<isize>().unwrap()).unwrap(),
            xmax: end.next().map(|d| d.parse::<isize>().unwrap()).unwrap(),
            ymin: start.next().map(|d| d.parse::<isize>().unwrap()).unwrap(),
            ymax: end.next().map(|d| d.parse::<isize>().unwrap()).unwrap(),
            zmin: start.next().map(|d| d.parse::<isize>().unwrap()).unwrap(),
            zmax: end.next().map(|d| d.parse::<isize>().unwrap()).unwrap(),
        }
    }

    fn overlap_with(&self, block: &Block) -> bool {
        (self.xmin.max(block.xmin) <= self.xmax.min(block.xmax))
            && (self.ymin.max(block.ymin) <= self.ymax.min(block.ymax))
    }

    fn fall_downward(&self, other_blocks: &Vec<Block>) -> Block {
        // println!("\nReceiving block: {:?}", self);
        // println!("    other block: {:?}", other_blocks);

        match other_blocks
            .iter()
            .filter(|b| self.overlap_with(b))
            .map(|b| b.zmax + 1)
            .max()
        {
            Some(new_z_min) => Block {
                xmin: self.xmin,
                xmax: self.xmax,
                ymin: self.ymin,
                ymax: self.ymax,
                zmin: new_z_min,
                zmax: self.zmax - (self.zmin - new_z_min),
            },
            None => Block {
                xmin: self.xmin,
                xmax: self.xmax,
                ymin: self.ymin,
                ymax: self.ymax,
                zmin: 1,
                zmax: self.zmax - (self.zmin - 1),
            },
        }
    }
}

fn settle(blocks: &Vec<Block>) -> Vec<Block> {
    let blocks = blocks
        .iter()
        .sorted_unstable_by_key(|b| b.zmin)
        .collect::<Vec<&Block>>();
    blocks.into_iter().fold(vec![], |mut blocks, block| {
        let new_block = block.fall_downward(&blocks);
        blocks.push(new_block);
        blocks
    })
}

fn part1(blocks: &Vec<Block>) -> usize {
    let blocks = blocks
        .iter()
        .sorted_unstable_by_key(|b| b.zmax)
        .rev()
        .map(|b| *b)
        .collect::<Vec<Block>>();
    blocks
        .clone()
        .into_iter()
        .filter_map(|b| {
            let above_blocks = blocks
                .iter()
                .filter(|bl| (bl.zmin == b.zmax + 1) && b.overlap_with(bl))
                .collect::<Vec<_>>();
            match above_blocks.len() > 0 {
                // do not support any other blocks
                false => Some(b),
                true => {
                    // are the other blocks overlaping with other blocks from the same z as b?
                    let same_level_blocks = blocks
                        .iter()
                        .filter(|bl| bl.zmax == b.zmax)
                        .collect::<Vec<_>>();

                    // println!("  Block above. Same level blocks: {:?}", same_level_blocks);
                    above_blocks
                        .iter()
                        .map(|a| {
                            same_level_blocks
                                .iter()
                                .filter(|bl| bl.overlap_with(a))
                                .count()
                        })
                        .all(|c| c > 1)
                        .then(|| b)
                }
            }
        })
        .count()
}

fn part2(blocks: &Vec<Block>) -> usize {
    let blocks = blocks
        .iter()
        .sorted_unstable_by_key(|b| b.zmax)
        .rev()
        .map(|b| *b)
        .collect::<Vec<Block>>();

    let depend_on = blocks
        .clone()
        .into_iter()
        .map(|b| {
            // below blocks
            (
                b,
                blocks
                    .iter()
                    .filter(|bl| (bl.zmax == b.zmin - 1) && b.overlap_with(bl))
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<HashMap<Block, Vec<&Block>>>();

    let cause_fall = blocks
        .clone()
        .into_iter()
        .filter_map(|b| {
            let above_blocks = blocks
                .iter()
                .filter(|bl| (bl.zmin == b.zmax + 1) && b.overlap_with(bl))
                .collect::<Vec<_>>();
            match above_blocks.len() > 0 {
                // do not support any other blocks
                false => None,
                true => {
                    // are the other blocks overlaping with other blocks from the same z as b?
                    let same_level_blocks = blocks
                        .iter()
                        .filter(|bl| bl.zmax == b.zmax)
                        .collect::<Vec<_>>();

                    let will_fall = above_blocks
                        .iter()
                        .map(|a| {
                            (
                                same_level_blocks
                                    .iter()
                                    .filter(|bl| bl.overlap_with(a))
                                    .count(),
                                a,
                            )
                        })
                        .filter_map(|(c, bl)| (c == 1).then(|| *bl))
                        .collect::<Vec<&Block>>();

                    (will_fall.len() > 0).then(|| (b, will_fall))
                }
            }
        })
        .collect::<HashMap<Block, Vec<&Block>>>();

    let mut ans = 0;

    for (_a, b) in cause_fall.into_iter() {
        let mut before = 0;
        let mut have_fallen: HashSet<Block> = HashSet::from_iter(b.clone().into_iter().map(|b| *b));
        let mut after = have_fallen.len();

        while after != before {
            before = after;
            // iterate over all blocks depend on and if depend check if they will fall
            let will_fall = depend_on
                .clone()
                .into_iter()
                .filter(|(_b, bdo)| bdo.len() > 0 && bdo.iter().all(|d| have_fallen.contains(d)))
                .map(|(b, _bdo)| b)
                .collect::<Vec<Block>>();

            have_fallen.extend(will_fall);
            after = have_fallen.len();
        }
        ans += after;
    }

    ans
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let blocks = input
        .lines()
        .map(|line| Block::from_str(line))
        .collect::<Vec<Block>>();

    let settled_blocks = settle(&blocks);

    let p1 = part1(&settled_blocks);
    let p2 = part2(&settled_blocks);
    (p1, p2)
}
