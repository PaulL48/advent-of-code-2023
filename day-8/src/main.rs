use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("./input.txt");

#[derive(Debug)]
struct Node<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str
}

impl<'a> From<&'a str> for Node<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            name: &value[0..3],
            left: &value[7..10],
            right: &value[12..15]
        }
    }
}

fn part1(directions: &str, node_map: &HashMap<&str, &Node>) {
    let mut steps = 0;
    let mut current_location = "AAA";
    for direction in directions.chars().cycle() {
        match direction {
            'L' => current_location = node_map.get(current_location).unwrap().left,
            'R' => current_location = node_map.get(current_location).unwrap().right,
            d => panic!("Bad direction: {}", d)
        }
        steps += 1;

        if current_location == "ZZZ" {
            break;
        }
    }

    println!("Took {} steps", steps)
}

fn number_of_steps(location: &str, directions: &str, node_map: &HashMap<&str, &Node>) -> u128 {
    let mut steps = 0;
    let mut current_location = location;
    for direction in directions.chars().cycle() {
        match direction {
            'L' => current_location = node_map.get(current_location).unwrap().left,
            'R' => current_location = node_map.get(current_location).unwrap().right,
            d => panic!("Bad direction: {}", d)
        }
        steps += 1;

        if current_location.ends_with('Z') {
            break;
        }
    }

    steps as u128
}

fn part2(directions: &str, nodes: &[Node], node_map: &HashMap<&str, &Node>) {
    let mut current_positions = nodes
        .iter()
        .map(|n| n.name)
        .filter(|n| n.ends_with('A'))
        .collect::<Vec<_>>();

    let mut steps: u128 = 0;
    for direction in directions.chars().cycle() {
        match direction {
            'L' => {
                for position in &mut current_positions {
                    *position = node_map.get(position).unwrap().left
                }
            },
            'R' => {
                for position in &mut current_positions {
                    *position = node_map.get(position).unwrap().right
                }
            },
            d => panic!("Bad direction: {}", d)
        }
        steps += 1;

        if current_positions.iter().all(|p| p.ends_with('Z')) {
            break;
        }
    }

    println!("Took {} steps", steps)
}

fn part2_2(directions: &str, nodes: &[Node], node_map: &HashMap<&str, &Node>) {
    let mut starting_positions = nodes
        .iter()
        .map(|n| n.name)
        .filter(|n| n.ends_with('A'))
        .map(|n| number_of_steps(n, directions, node_map))
        .collect::<Vec<_>>();

    
    println!("{:?}", starting_positions);

    let mut multiplier = 1;
    while true {
        let a = starting_positions.iter().map(|n| n * multiplier).collect::<Vec<_>>();
        let v = a[0];
        if a.iter().all(|val| *val == v) {
            println!("{:?}", a);
        }
        multiplier += 1;
    }
    // println!("{}", starting_positions);
}

fn part2_3() {
    let mut multiple = 79 * 73;
    let mut factors = HashSet::new();
    factors.insert(43);
    factors.insert(67);
    factors.insert(61);
    factors.insert(47);

    let mut multiplier: u128 = 1;

    while !factors.is_empty() {
        let result = multiple * multiplier;

        let mut remove_factor = None;
        for factor in &factors {
            if result % factor == 0 {
                multiple = result;
                remove_factor = Some(*factor);
                break;
            }
        }

        if let Some(factor) = remove_factor {
            factors.remove(&factor);
        }
        multiplier += 1;
    }

    println!("{:?}", multiple * 281)
}

fn main() {
    let mut line_iter = INPUT.lines();
    let directions = line_iter.next().unwrap();

    let nodes = line_iter
        .skip(1)
        .map(|l| l.into())
        .collect::<Vec<Node>>();

    let node_map = nodes
        .iter()
        .map(|n| (n.name, n))
        .collect::<HashMap<_, _>>();

    part1(&directions, &node_map);
    part2_3()
}
