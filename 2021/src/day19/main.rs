use std::path::Path;
use std::fs;
use itertools::Itertools;
use itertools::iproduct;
use std::collections::{HashMap, HashSet};

// Reference article:
// http://nghiaho.com/?page_id=671

// Algorithm steps:
// 1- compute all distances of each beacon with the other beacons into a scanner's view
// 2- initialize grid, using the first scanner as referentiel
// 3- select scanner having the most common beacons with grid by comparing inter-beacons-distances (see 1)
// 4- compute center of gravity point for grid and beacons'scanner, and offset both to their COG
// 5- uses offseted points to calculate rotation, rotate all points, and get translation
// 6- cache translation for part 2 (As they also represent scanner position)
// 7- use rotation and translation from 5 to get the absolute position of all the beacons of the selected scanner and add them to the grid
// 8- remove the merged scanner from the list of scanners and repeat from 3 untill there is no more scanner to treat


type Beacon = (isize, isize, isize);

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Scanner {
    id: usize,
    beacons: Vec<Beacon>
}

impl Scanner {

    fn beacons_distance_to_others(&self) -> HashMap<Beacon, HashSet<usize>> {
        compute_beacons_distance_to_others(&HashSet::from_iter(self.beacons.clone()))
    }

    fn get_scanner_grid_correspondance_map(&self, grid_beacons_inter_distances: &HashMap<Beacon, HashSet<usize>>) -> HashMap<Beacon, Beacon> {
        let scanner_beacons_inter_distances = self.beacons_distance_to_others();

        // we filter by beacon having at least 11 similar distances
        // (group of 12 beacons = 11 neighbors each)
        iproduct!(grid_beacons_inter_distances.iter(), scanner_beacons_inter_distances.iter())
            .map(|((gk, gv), (sk, sv))| (*gk, *sk, gv.intersection(sv).count()))
            .filter(|(_, _, count)| *count >= 11)
            .map(|(gk, sk, _)| (sk, gk))
            .collect()
    }
}

fn compute_beacons_distance_to_others(beacons: &HashSet<Beacon>) -> HashMap<Beacon, HashSet<usize>> {
    let mut distances: HashMap<Beacon, HashSet<usize>> = HashMap::new();
    beacons.iter()
        .tuple_combinations()
        .for_each(|(b1, b2)| {
            let d = distance_between(b1, b2);
            distances.entry(*b1).or_insert(HashSet::new()).insert(d);
            distances.entry(*b2).or_insert(HashSet::new()).insert(d);
        });
    distances
}

fn distance_between(b1: &Beacon, b2: &Beacon) -> usize {
    (((b1.0 - b2.0).pow(2) + (b1.1 - b2.1).pow(2) + (b1.2 - b2.2).pow(2)) as f64).sqrt() as usize
}

fn get_best_candidate_scanner_idx_and_mapping(scanners: &Vec<Scanner>, grid_distances: &HashMap<Beacon, HashSet<usize>>) -> (usize, HashMap<Beacon, Beacon>) {
        scanners.iter()
            .map(|s| s.get_scanner_grid_correspondance_map(grid_distances))
            .enumerate()
            .max_by(|b1, b2| b1.1.len().cmp(&b2.1.len()))
            .unwrap()
}

fn compute_translation_and_rotation(mapping: &HashMap<Beacon, Beacon>) -> (Vec<isize>, HashMap<usize, (usize, isize)>) {

    let n = mapping.len() as f32;

    // center of gravity for grid
    let grid_cog: Vec<f32> = mapping.values()
        .fold([0; 3], |acc, (x, y, z)| {
            [acc[0] + x, acc[1] + y, acc[2] + z]
        })
        .iter()
        .map(|v| (*v as f32) / n)
        .collect();

    // center of gravity for sensor
    let sensor_cog: Vec<f32> = mapping.keys()
        .fold([0; 3], |acc, (x, y, z)| {
            [acc[0] + x, acc[1] + y, acc[2] + z]
        })
        .iter()
        .map(|v| (*v as f32) / n)
        .collect();

    // offset a beacon compared to center of gravity
    let sensor_beacon = mapping.keys().last().unwrap();
    let grid_beacon = mapping.get(sensor_beacon).unwrap();
    
    let offsetted_sensor_beacon = [
        (sensor_beacon.0 as f32 - sensor_cog[0]).round(),
        (sensor_beacon.1 as f32 - sensor_cog[1]).round(),
        (sensor_beacon.2 as f32 - sensor_cog[2]).round(),
    ];
    let offsetted_grid_beacon = [
        (grid_beacon.0 as f32 - grid_cog[0]).round(),
        (grid_beacon.1 as f32 - grid_cog[1]).round(),
        (grid_beacon.2 as f32 - grid_cog[2]).round(),
    ];

    // compute rotation by finding common offsetted coordinates
    let mut rotation: HashMap<usize, (usize, isize)> = HashMap::new();
    iproduct!(
        offsetted_grid_beacon.iter().enumerate(),
        offsetted_sensor_beacon.iter().enumerate()
    ).for_each(|((i, c1), (j, c2))| match  c1.abs() == c2.abs() {
            true => {rotation.insert(i, (j, (c1 / c2) as isize));},
            _ => {},
        });
    
    // transform coordinate to match grid coords
    let sensor_beacon = [sensor_beacon.0, sensor_beacon.1, sensor_beacon.2];
    let mut rotated_sensor_beacon: [isize; 3] = [0; 3];
    rotation.iter()
        .for_each(|(axis, rotation)| {
            rotated_sensor_beacon[*axis] = sensor_beacon[rotation.0] * rotation.1;
        });

    // translation
    let grid_beacon = [grid_beacon.0, grid_beacon.1, grid_beacon.2];
    let translation = rotated_sensor_beacon.iter()
        .zip(grid_beacon)
        .map(|(rotated, reference)| rotated - reference)
        .collect::<Vec<isize>>();
    
    (translation, rotation)
}

fn apply_transformation(trans: &Vec<isize>, rot: &HashMap<usize, (usize, isize)>, beacons: &Vec<Beacon>) -> HashSet<Beacon> {
    beacons.iter()
        .map(|b| {
            let original = [b.0, b.1, b.2];
            let mut transformed = [0;3];
            rot.iter()
                .for_each(|(axis, rotation)| {
                    transformed[*axis] = original[rotation.0] * rotation.1 - trans[*axis];
                });
            (transformed[0], transformed[1], transformed[2])
        })
    .collect()
}

fn part1(input: &str) -> (usize, Vec<Vec<isize>>) {

    let mut scanners = parse_input(input);

    let mut grid: HashSet<Beacon> = HashSet::from_iter(scanners[0].beacons.clone());
    scanners.remove(0);

    // for part 2
    let mut positions: Vec<Vec<isize>> = Vec::new();

    while scanners.len() > 0 {
        // get scanner with most beacons in common
        let grid_distances = compute_beacons_distance_to_others(&grid);
        let (idx, beacon_mapping) = get_best_candidate_scanner_idx_and_mapping(&scanners, &grid_distances);
        let selected_scanner = &scanners[idx];

        // compute transformation (translation + rotation) of the scanner referentiel into grid referentiel
        let (translation, rotation) = compute_translation_and_rotation(&beacon_mapping);

        // beacons coordinates in grid referentiel
        let transformed_beacons = apply_transformation(&translation, &rotation, &selected_scanner.beacons);

        grid.extend(transformed_beacons);
        scanners.remove(idx);

        // for part 2
        positions.push(translation);
    }

    (grid.len(), positions)
}


fn manhattan_distance(p1: (isize, isize, isize), p2: (isize, isize, isize)) -> isize {
    (p2.0 - p1.0).abs() + (p2.1 - p1.1).abs() + (p2.2 - p1.2).abs()
}


fn part2(positions: Vec<Vec<isize>>) -> isize {
    iproduct!(positions.clone(), positions.clone())
        .map(|(p1, p2)| manhattan_distance((p1[0], p1[1], p1[2]), (p2[0], p2[1], p2[2])))
        .max()
        .unwrap()
}


fn parse_input(content: &str) -> Vec<Scanner> {
    content.split("\n\n")
        .map(|line| {
            let mut beacons = line.lines();

            let scanner_id: usize = beacons.next().unwrap()
                .split(" ")
                .collect::<Vec<&str>>()[2]
                .parse().unwrap();
            let beacons = beacons
                .filter_map(|b| b.split(",")
                     .map(|d| d.parse().unwrap())
                     .collect_tuple())
                .collect::<Vec<Beacon>>();

            Scanner{
                id: scanner_id,
                beacons
            }
        })
    .collect()
}

fn main() {
    let path = Path::new("./inputs/day19.txt");
    let input = fs::read_to_string(path).expect("Unable to read file");

    let (p1, positions) = part1(&input);
    println!("{}", p1);
    let p2 = part2(positions);
    println!("{}", p2);
}
