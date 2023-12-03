use std::collections::HashMap;

const INPUT: &str = include_str!("./input.txt");

fn game_qualifies(game: &str, constraints: &HashMap<&str, u32>) -> Option<u32> {
    let mut game_iter = game.split(':');
    let game_str = game_iter.next().unwrap();
    let game_id: u32 = game_str.split(' ').nth(1).unwrap().parse().unwrap();

    let sets = game_iter.next().unwrap();
    for set in sets.split(';') {
        for block_str in set.split(',') {
            let block_str = block_str.trim();
            let mut iter = block_str.split(' ');
            let number: u32 = iter.next().unwrap().parse().unwrap();
            let color = iter.next().unwrap();
            if let Some(limit) = constraints.get(color) {
                if number > *limit {
                    return None;
                }
            }
        }
    }
    Some(game_id)
}

fn insert_if_greater(key: &str, value: u32, map: &mut HashMap<&str, u32>) {
    if value > *map.get(key).unwrap() {
        *map.get_mut(key).unwrap() = value;
    }
}

fn game_power(game: &str) -> u32 {
    let mut min_colors = HashMap::new();
    min_colors.insert("red", 0);
    min_colors.insert("green", 0);
    min_colors.insert("blue", 0);

    let sets = game.split(':').nth(1).unwrap();
    for set in sets.split(';') {
        for block_str in set.split(',') {
            let block_str = block_str.trim();
            let mut iter = block_str.split(' ');
            let number: u32 = iter.next().unwrap().parse().unwrap();
            let color = iter.next().unwrap();
            insert_if_greater(color, number, &mut min_colors);
        }
    }

    min_colors.values().product()
}

fn main() {
    let constraints = {
        let mut m = HashMap::new();
        m.insert("red", 12);
        m.insert("blue", 14);
        m.insert("green", 13);
        m
    };

    let sum: u32 = INPUT
        .lines()
        .filter_map(|l| game_qualifies(l, &constraints))
        .sum();
    println!("Sum of disqualified game IDs: {}", sum);

    let power_sum: u32 = INPUT.lines().map(game_power).sum();
    println!("Sum of game powers: {}", power_sum);
}
