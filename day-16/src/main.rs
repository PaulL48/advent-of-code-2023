use std::{collections::HashSet, io::Write};

use grid::Grid;

const INPUT: &str = include_str!("./input.txt");

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Beam {
    location: (i64, i64),
    direction: Direction,
    dead: bool,
}

impl Beam {
    fn initial_beam() -> Self {
        Self {
            location: (0, 0),
            direction: Direction::Right,
            dead: false,
        }
    }

    fn new(location: (i64, i64), direction: Direction) -> Self {
        Self {
            location,
            direction,
            dead: false,
        }
    }

    fn is_dead(&self, grid: &Grid<char>, cycle_checker: &HashSet<Beam>) -> bool {
        if self.dead
            || cycle_checker.contains(self)
            || self.location.0 < 0
            || self.location.0 >= grid.cols() as i64
            || self.location.1 < 0
            || self.location.1 >= grid.rows() as i64
        {
            return true;
        }

        false
    }

    fn advance(&mut self, grid: &Grid<char>, cycle_checker: &mut HashSet<Beam>) -> Option<Beam> {
        std::io::stdout().flush().unwrap();
        let current_tile = grid
            .get(self.location.1 as usize, self.location.0 as usize)
            .unwrap();

        let mut new_beam = match (current_tile, self.direction) {
            ('\\', Direction::Up) => {
                self.location.0 += -1;
                self.direction = Direction::Left;
                None
            }
            ('\\', Direction::Down) => {
                self.location.0 += 1;
                self.direction = Direction::Right;
                None
            }
            ('\\', Direction::Left) => {
                self.location.1 += -1;
                self.direction = Direction::Up;
                None
            }
            ('\\', Direction::Right) => {
                self.location.1 += 1;
                self.direction = Direction::Down;
                None
            }

            ('/', Direction::Up) => {
                self.location.0 += 1;
                self.direction = Direction::Right;
                None
            }
            ('/', Direction::Down) => {
                self.location.0 += -1;
                self.direction = Direction::Left;
                None
            }
            ('/', Direction::Left) => {
                self.location.1 += 1;
                self.direction = Direction::Down;
                None
            }
            ('/', Direction::Right) => {
                self.location.1 += -1;
                self.direction = Direction::Up;
                None
            }

            ('|', Direction::Up) => {
                self.location.1 += -1;
                None
            }
            ('|', Direction::Down) => {
                self.location.1 += 1;
                None
            }
            ('|', Direction::Left) => {
                self.location.1 += -1;
                self.direction = Direction::Up;

                // new beam
                Some(Beam::new(
                    (self.location.0, self.location.1 + 2),
                    Direction::Down,
                ))
            }
            ('|', Direction::Right) => {
                self.location.1 += -1;
                self.direction = Direction::Up;

                // new beam
                Some(Beam::new(
                    (self.location.0, self.location.1 + 2),
                    Direction::Down,
                ))
            }

            ('-', Direction::Up) => {
                self.location.0 += -1;
                self.direction = Direction::Left;

                // new beam
                Some(Beam::new(
                    (self.location.0 + 2, self.location.1),
                    Direction::Right,
                ))
            }
            ('-', Direction::Down) => {
                self.location.0 += -1;
                self.direction = Direction::Left;

                // new beam
                Some(Beam::new(
                    (self.location.0 + 2, self.location.1),
                    Direction::Right,
                ))
            }
            ('-', Direction::Left) => {
                self.location.0 += -1;
                None
            }
            ('-', Direction::Right) => {
                self.location.0 += 1;
                None
            }

            (_, Direction::Down) => {
                self.location.1 += 1;
                None
            }
            (_, Direction::Up) => {
                self.location.1 += -1;
                None
            }
            (_, Direction::Left) => {
                self.location.0 += -1;
                None
            }
            (_, Direction::Right) => {
                self.location.0 += 1;
                None
            }
        };

        // Check current beam for deadness
        if self.is_dead(grid, cycle_checker) {
            self.dead = true;
        } else {
            // add it to the cycle checker
            cycle_checker.insert(*self);
        }

        // Evaluate dead condition for new beam
        if let Some(new_beam) = &mut new_beam {
            if new_beam.is_dead(grid, cycle_checker) {
                None
            } else {
                cycle_checker.insert(*self);
                Some(*new_beam)
            }
        } else {
            None
        }
    }
}

fn trace_beam_path(grid: &Grid<char>) {
    let mut set = HashSet::new();
    let mut cycle_checker = HashSet::new();
    let mut beams = vec![Beam::initial_beam()];
    set.insert(beams[0].location);

    while beams.iter().any(|b| !b.dead) {
        let mut beams_to_add = Vec::new();
        for beam in beams.iter_mut().filter(|b| !b.dead) {
            set.insert(beam.location);
            if let Some(new_beam) = beam.advance(grid, &mut cycle_checker) {
                beams_to_add.push(new_beam);
            }
        }

        beams.extend(beams_to_add.iter());
    }

    println!("Energized count: {}", set.len());
}

fn main() {
    let columns = INPUT.lines().next().unwrap().len();
    let grid = Grid::from_vec(INPUT.lines().flat_map(|l| l.chars()).collect(), columns);
    trace_beam_path(&grid);
}
