use itertools::Itertools;

fn parse_input(
    input: &str,
) -> (
    Vec<usize>,
    Vec<(usize, usize, usize)>,
    Vec<(usize, usize, usize)>,
    Vec<(usize, usize, usize)>,
    Vec<(usize, usize, usize)>,
    Vec<(usize, usize, usize)>,
    Vec<(usize, usize, usize)>,
    Vec<(usize, usize, usize)>,
) {
    let (seeds, rest) = input.split_once("\n\n").unwrap();
    let (seeds_to_soil, rest) = rest.split_once("\n\n").unwrap();
    let (soil_to_fertilizer, rest) = rest.split_once("\n\n").unwrap();
    let (fertilizer_to_water, rest) = rest.split_once("\n\n").unwrap();
    let (water_to_light, rest) = rest.split_once("\n\n").unwrap();
    let (ligth_to_temperature, rest) = rest.split_once("\n\n").unwrap();
    let (temperature_to_humidity, rest) = rest.split_once("\n\n").unwrap();
    let humidity_to_location = rest;

    let (_, seeds) = seeds.split_once(": ").unwrap();
    let seeds = seeds
        .split(" ")
        .map(|d| d.parse::<usize>().ok().unwrap())
        .collect::<Vec<_>>();

    let seeds_to_soil = parse_mapping(seeds_to_soil);
    let soil_to_fertilizer = parse_mapping(soil_to_fertilizer);
    let fertilizer_to_water = parse_mapping(fertilizer_to_water);
    let water_to_light = parse_mapping(water_to_light);
    let ligth_to_temperature = parse_mapping(ligth_to_temperature);
    let temperature_to_humidity = parse_mapping(temperature_to_humidity);
    let humidity_to_location = parse_mapping(humidity_to_location);

    (
        seeds,
        seeds_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        ligth_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    )
}

fn parse_mapping(mapping: &str) -> Vec<(usize, usize, usize)> {
    mapping
        .split("\n")
        .skip(1)
        .map(|line| {
            let (destination, source, range) = line
                .splitn(3, " ")
                .map(|d| d.parse::<usize>().ok().unwrap())
                .collect_tuple()
                .unwrap();
            (source, source + range - 1, destination)
        })
        .collect()
}

fn get_destination(val: &usize, mapping: &Vec<(usize, usize, usize)>) -> usize {
    let res = mapping
        .iter()
        .find_map(|(source_min, source_max, destination_min)| {
            match val >= source_min && val <= source_max {
                true => Some(destination_min + (val - source_min)),
                false => None,
            }
        });
    match res {
        Some(res) => res,
        None => *val,
    }
}

fn part1(seeds: &Vec<usize>, transformations: Vec<&Vec<(usize, usize, usize)>>) -> usize {
    seeds
        .iter()
        .map(|s| {
            transformations
                .iter()
                .fold(*s, |acc, t| get_destination(&acc, t))
        })
        .min()
        .unwrap()
}

fn part2(seeds: &Vec<usize>, transformations: Vec<&Vec<(usize, usize, usize)>>) -> usize {
    let seeds = seeds
        .iter()
        .tuples::<(_, _)>()
        .map(|(s_min, s_range)| (*s_min, *s_min + s_range))
        .collect::<Vec<(usize, usize)>>();

    let boundaries = transformations.into_iter().fold(seeds, |acc, mappings| {
        acc.into_iter()
            .flat_map(|(start, end)| {
                // we start with the full range unmapped
                let mut unmapped = vec![(start, end)];
                let mut mapped = Vec::new();

                for (s, e, d) in mappings {
                    let mut temp_unmapped = vec![];

                    for (start, end) in unmapped {
                        match (start < *s, end < *s, start > *e, end > *e) {
                            // fully below interval
                            (true, true, _, _) => temp_unmapped.push((start, end)),
                            // fully above interval
                            (_, _, true, true) => temp_unmapped.push((start, end)),
                            // fully inside interval
                            (false, false, false, false) => {
                                let offset_from_start = start - s;
                                let offset_from_end = end - s;
                                mapped.push((d + offset_from_start, d + offset_from_end))
                            }
                            // overlap lower part
                            (true, false, _, false) => {
                                temp_unmapped.push((start, s - 1));
                                let offset_from_end = end - s;
                                mapped.push((*d, d + offset_from_end));
                            }
                            // overlap upper part
                            (false, false, false, true) => {
                                let range = e - s;
                                temp_unmapped.push((e + 1, end));
                                let offset_from_start = start - s;
                                mapped.push((d + offset_from_start, d + range));
                            }
                            // overlap both sides
                            (true, false, false, true) => {
                                let range = e - s;
                                temp_unmapped.push((start, s - 1));
                                temp_unmapped.push((e + 1, end));
                                mapped.push((*d, d + range));
                            }
                            (_, _, _, _) => unreachable!(),
                        }
                    }
                    unmapped = temp_unmapped;
                }

                mapped.extend(unmapped);
                mapped
            })
            .collect::<Vec<_>>()
    });
    boundaries.iter().map(|(s, _e)| *s).min().unwrap()
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let (
        seeds,
        seeds_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    ) = parse_input(input);

    let p1 = part1(
        &seeds,
        vec![
            &seeds_to_soil,
            &soil_to_fertilizer,
            &fertilizer_to_water,
            &water_to_light,
            &light_to_temperature,
            &temperature_to_humidity,
            &humidity_to_location,
        ],
    );

    let p2 = part2(
        &seeds,
        vec![
            &seeds_to_soil,
            &soil_to_fertilizer,
            &fertilizer_to_water,
            &water_to_light,
            &light_to_temperature,
            &temperature_to_humidity,
            &humidity_to_location,
        ],
    );

    (p1, p2)
}
