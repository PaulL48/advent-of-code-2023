const INPUT: &str = include_str!("./input.txt");

struct Race {
    time: u64,
    best_distance: u64,
}

impl Race {
    pub fn new(time: u64, best_distance: u64) -> Self {
        Race {
            time,
            best_distance,
        }
    }

    pub fn distance_traveled(held_time: u64, total_time: u64) -> u64 {
        let travel_time = total_time - held_time;
        let travel_speed = held_time;
        travel_time * travel_speed
    }

    pub fn number_of_better_configs(&self) -> u64 {
        let mut better_configs = 0;
        for held_time in 0..self.time {
            if Race::distance_traveled(held_time, self.time) > self.best_distance {
                better_configs += 1;
            }
        }
        better_configs
    }
}

fn part1(races: &[Race]) {
    let result: u64 = races.iter().map(|r| r.number_of_better_configs()).product();
    println!("{}", result)
}

fn part2(race: &Race) {
    println!("{}", race.number_of_better_configs())
}

fn main() {
    let mut line_iter = INPUT.lines();
    let time_line = line_iter.next().unwrap();
    let distance_line = line_iter.next().unwrap();

    let race_times = time_line
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let race_distances = distance_line
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let races = race_times
        .iter()
        .zip(race_distances)
        .map(|(&time, distance)| Race::new(time, distance))
        .collect::<Vec<_>>();

    let part2_race_time = time_line
        .split_whitespace()
        .skip(1)
        .flat_map(|s| s.chars())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let part2_race_best_distance = distance_line
        .split_whitespace()
        .skip(1)
        .flat_map(|s| s.chars())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    part1(&races);
    part2(&Race::new(part2_race_time, part2_race_best_distance));
}
