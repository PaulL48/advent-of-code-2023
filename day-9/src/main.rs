const INPUT: &str = include_str!("./input.txt");

fn get_tower(line: &[i64]) -> Vec<Vec<i64>> {
    let mut tower = Vec::new();
    tower.push(line.to_vec());
    let mut current_line = tower.last().unwrap();
    while current_line.iter().any(|v| *v != 0) {
        tower.push(differences(current_line));
        current_line = tower.last().unwrap();
    }
    tower.pop();
    tower
}

fn differences(line: &[i64]) -> Vec<i64> {
    line.windows(2).map(|v| v[1] - v[0]).collect()
}

fn next_value_end(line: &[i64]) -> i64 {
    let tower = get_tower(line);
    let mut current_end_value = 0;
    for level in tower.iter().rev() {
        let end_value = level.last().unwrap();
        current_end_value += end_value;
    }
    current_end_value
}

fn next_value_start(line: &[i64]) -> i64 {
    let tower = get_tower(line);
    let mut current_start_value = 0;
    for level in tower.iter().rev() {
        let start_value = level.first().unwrap();
        current_start_value = start_value - current_start_value;
    }
    current_start_value
}

fn part1(lines: &[Vec<i64>]) {
    let result = lines.iter().map(|l| next_value_end(l)).sum::<i64>();
    println!("{}", result)
}

fn part2(lines: &[Vec<i64>]) {
    let result = lines.iter().map(|l| next_value_start(l)).sum::<i64>();
    println!("{}", result)
}

fn main() {
    let lines = INPUT
        .lines()
        .map(|l| {
            l.split(' ')
                .map(|d| d.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    part1(&lines);
    part2(&lines);
}
