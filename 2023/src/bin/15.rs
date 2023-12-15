use std::collections::HashMap;

#[derive(Debug)]
enum Lens<'a> {
    Add((&'a str, usize)),
    Remove(&'a str),
}

impl Lens<'_> {
    fn parse(step: &str) -> Lens {
        let mut eq = step.split("=");
        match eq.clone().count() == 2 {
            true => Lens::Add((
                eq.next().unwrap(),
                eq.next().map(|d| d.parse::<usize>().ok().unwrap()).unwrap(),
            )),
            false => Lens::Remove(eq.next().unwrap().trim_end_matches("-")),
        }
    }

    fn is_remove(&self) -> bool {
        match self {
            Lens::Add(_) => false,
            Lens::Remove(_) => true,
        }
    }

    fn label(&self) -> &str {
        match self {
            Lens::Add((l, _)) => *l,
            Lens::Remove(l) => *l,
        }
    }

    fn focal_length(&self) -> usize {
        match self {
            Lens::Add((_, f)) => *f,
            Lens::Remove(_) => 0,
        }
    }

    fn find_box(&self) -> usize {
        hash(self.label())
    }
}

fn hash(step: &str) -> usize {
    step.chars()
        .fold(0, |hash, c| ((hash + c as usize) * 17) % 256)
}

fn focusing_power(steps: &str) -> usize {
    let mut boxes: HashMap<usize, Vec<(String, usize)>> = HashMap::new();

    steps.split(",").map(Lens::parse).for_each(|s| {
        let box_number = s.find_box();
        let b = boxes.entry(box_number).or_insert(vec![]);
        let label = s.label().to_string();

        if s.is_remove() {
            if let Some(idx) = b.iter().position(|(label, _)| label == &s.label()) {
                b.swap_remove(idx);
            }
        } else {
            match b.iter().position(|(label, _)| label == &s.label()) {
                Some(idx) => {
                    b[idx] = (label, s.focal_length());
                }
                None => {
                    b.push((label.clone(), s.focal_length()));
                }
            }
        }
    });

    boxes
        .iter()
        .map(|(i, lens)| {
            let a = i + 1;
            let b = lens
                .iter()
                .enumerate()
                .map(|(i, (_, v))| (i + 1) * v)
                .sum::<usize>();
            a * b
        })
        .sum()
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let p1 = input.split(",").map(hash).sum();
    let p2 = focusing_power(input);
    (p1, p2)
}
