use std::ops::Range;

const INPUT: &str = include_str!("./input.txt");

#[derive(Debug, Clone)]
struct RangeRelation {
    input_range: Range<u64>,
    output_range: Range<u64>,
}

impl RangeRelation {
    fn new(input_range: Range<u64>, output_range: Range<u64>) -> Self {
        RangeRelation {
            input_range,
            output_range,
        }
    }

    fn apply(&self, input: u64) -> Option<u64> {
        // Check if the number is in the range
        if input >= self.input_range.start && input < self.input_range.end {
            let input_diff = input - self.input_range.start;
            return Some(self.output_range.start + input_diff);
        }
        None
    }
}

#[derive(Debug, Clone)]
struct RangeMapping {
    ranges: Vec<RangeRelation>,
}

impl RangeMapping {
    fn new() -> Self {
        RangeMapping { ranges: Vec::new() }
    }

    fn add_range(&mut self, range: RangeRelation) {
        self.ranges.push(range);
    }

    fn apply(&self, input: u64) -> u64 {
        for range in &self.ranges {
            if let Some(value) = range.apply(input) {
                return value;
            }
        }
        input
    }
}
const MAP_COUNT: usize = 7;

fn part1(seeds: &[u64], mappings: &[RangeMapping; 7]) {
    let min_location = seeds
        .iter()
        .map(|input| {
            mappings
                .iter()
                .fold(*input, |acc, mapping| mapping.apply(acc))
        })
        .min()
        .unwrap();

    println!("{}", min_location)
}

fn part2(seeds: &[u64], mappings: &[RangeMapping]) {
    // Convert seeds to ranges
    let seed_ranges = seeds
        .windows(2)
        .step_by(2)
        .map(|x| x[0]..x[0] + x[1])
        .collect::<Vec<_>>();
    let mut min_location = u64::MAX;
    for seed_range in seed_ranges {
        for seed in seed_range {
            let location = mappings
                .iter()
                .fold(seed, |acc, mapping| mapping.apply(acc));
            if location < min_location {
                min_location = location;
            }
        }
    }

    println!("{}", min_location);
}

fn main() {
    let mut all_maps: [RangeMapping; MAP_COUNT] = [
        RangeMapping::new(),
        RangeMapping::new(),
        RangeMapping::new(),
        RangeMapping::new(),
        RangeMapping::new(),
        RangeMapping::new(),
        RangeMapping::new(),
    ];

    // Fill range relations
    let mut section_iter = INPUT.split("\n\n");
    let seeds = section_iter.by_ref().next().unwrap();
    let seeds = seeds
        .split(' ')
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    for (section_index, section) in section_iter.enumerate() {
        for line in section.lines().skip(1) {
            // Parse the line and add it to ith range mapping
            let mut number_iter = line.split(' ').map(|x| x.parse::<u64>().unwrap());

            let output_range_start = number_iter.next().unwrap();
            let input_range_start = number_iter.next().unwrap();
            let length = number_iter.next().unwrap();

            all_maps[section_index].add_range(RangeRelation::new(
                input_range_start..input_range_start + length,
                output_range_start..output_range_start + length,
            ))
        }
    }

    part1(&seeds, &all_maps);
    part2(&seeds, &all_maps);
}
