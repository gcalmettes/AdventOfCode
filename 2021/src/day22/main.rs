use std::path::Path;
use std::fs;
use std::ops::RangeInclusive;
use regex::Regex;
use itertools::iproduct;
use std::collections::HashMap;
use std::cmp::{max, min};


 // #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
 #[derive(Clone, Debug, PartialEq, Hash, Eq, Copy)]
 enum Mode {
     ON,
     OFF,
 }

 #[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
 struct Instruction {
     state: Mode,
     x_min: isize,
     x_max: isize,
     y_min: isize,
     y_max: isize,
     z_min: isize,
     z_max: isize,
 }

 impl Instruction {
     fn x_range(&self) -> RangeInclusive<isize> {
         self.x_min..=self.x_max
     }

     fn y_range(&self) -> RangeInclusive<isize> {
         self.y_min..=self.y_max
     }

     fn z_range(&self) -> RangeInclusive<isize> {
         self.z_min..=self.z_max
     }

     fn is_in_small_range(&self) -> bool {
         self.x_min >= -50 && self.x_max <= 50 && self.y_min >= -50 && self.y_max <= 50 && self.z_min >= -50 && self.z_max <= 50
     }

 }

 #[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
 struct Cube {
     x_min: isize,
     x_max: isize,
     y_min: isize,
     y_max: isize,
     z_min: isize,
     z_max: isize,
 }

 impl Cube {
     fn intersect(&self, other: &Cube) -> Option<Cube> {
         let (lx, rx) = (max(self.x_min, other.x_min), min(self.x_max, other.x_max));
         let (ly, ry) = (max(self.y_min, other.y_min), min(self.y_max, other.y_max));
         let (lz, rz) = (max(self.z_min, other.z_min), min(self.z_max, other.z_max));

         // No intersection, we break
         if lx > rx || ly > ry || lz > rz {
             return None
         }

         // Inner itersection cube
         Some(Cube{
             x_min: lx,
             x_max: rx,
             y_min: ly,
             y_max: ry,
             z_min: lz,
             z_max: rz,
         })
     }

     fn coords(&self) -> [isize; 6] {
         [self.x_min, self.x_max, self.y_min, self.y_max, self.z_min, self.z_max]
     }
 }

 fn parse_input(content: &str) -> Vec<Instruction> {
     let mut instructions: Vec<Instruction> = Vec::new();
     let re = Regex::new(r"(on|off) x=(\-?[0-9]+)..(\-?[0-9]+),y=(\-?[0-9]+)..(\-?[0-9]+),z=(\-?[0-9]+)..(\-?[0-9]+)").unwrap();

     content.lines()
         .for_each(|line| {
             let m = re.captures(line).unwrap();
             instructions.push(Instruction{
                 state: if m.get(1).unwrap().as_str() == "on" {Mode::ON} else {Mode::OFF},
                 x_min: m.get(2).unwrap().as_str().parse().unwrap(),
                 x_max: m.get(3).unwrap().as_str().parse().unwrap(),
                 y_min: m.get(4).unwrap().as_str().parse().unwrap(),
                 y_max: m.get(5).unwrap().as_str().parse().unwrap(),
                 z_min: m.get(6).unwrap().as_str().parse().unwrap(),
                 z_max: m.get(7).unwrap().as_str().parse().unwrap(),
             })
         });
     instructions
 }

 fn part1(input: &str) -> usize {
     let instructions = parse_input(input);

     let mut cubes: HashMap<(isize, isize, isize), Mode> = HashMap::new();
     instructions.iter()
         .for_each(|ins| {
             if ins.is_in_small_range() {
                 for (x, y, z) in iproduct!(ins.x_range(), ins.y_range(), ins.z_range()) {
                     cubes.insert((x, y, z), ins.state.clone());
                 }
             }
         });
     cubes.values()
         .filter(|v| **v == Mode::ON)
         .count()
 }

 fn part2(input: &str) -> isize {
     let instructions = parse_input(input);

     let mut cubes_ranges: HashMap<[isize; 6], isize> = HashMap::new();
     instructions.into_iter()
         .for_each(|ins| {
             let mut new_cubes_ranges = cubes_ranges.clone();

             let ins_cube = Cube{
                 x_min: ins.x_min,
                 x_max: ins.x_max,
                 y_min: ins.y_min,
                 y_max: ins.y_max,
                 z_min: ins.z_min,
                 z_max: ins.z_max,
             };

             // we iterate over all the already existing cubes ranges and
             // find intersections if any
             for (cube, n) in cubes_ranges.iter_mut() {
                 let cube = Cube{
                     x_min: cube[0],
                     x_max: cube[1],
                     y_min: cube[2],
                     y_max: cube[3],
                     z_min: cube[4],
                     z_max: cube[5],
                 };
                 if let Some(intersection) = cube.intersect(&ins_cube) {
                     *new_cubes_ranges.entry(intersection.coords()).or_insert(0) -= *n;
                 }
             }

             if ins.state == Mode::ON {
                 *new_cubes_ranges.entry(ins_cube.coords()).or_insert(0) += 1;
             }
             cubes_ranges = new_cubes_ranges;

         });

     cubes_ranges
         .into_iter()
         .map(|(cube, n)| {
             let xs = cube[1] - cube[0] + 1;
             let ys = cube[3] - cube[2] + 1;
             let zs = cube[5] - cube[4] + 1;
             xs * ys * zs * n
         })
         .sum::<isize>()
 }


 fn main() {
     let path = Path::new("./inputs/day22.txt");
     let input = fs::read_to_string(path).expect("Unable to read file");

     let p1 = part1(&input);
     println!("{}", p1);
     let p2 = part2(&input);
     println!("{}", p2);
 }

