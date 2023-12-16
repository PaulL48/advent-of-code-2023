use grid::Grid;

const INPUT: &str = include_str!("./input.txt");

fn sum_from(from: u32, to: u32) -> u32 {
    let sum1 = (from * (from + 1)) / 2;
    let sum2 = (to * (to + 1)) / 2;
    sum1 - sum2
}

fn part1(map: &Grid<char>) {
    let column_size = map.iter_cols().next().unwrap().len();

    let mut total = 0;
    for column in map.iter_cols() {
        let mut vertical_index = column_size as u32;
        let mut column_total = 0;
        let mut rocks = 0;
        for (i, c) in column.enumerate() {
            match c {
                '#' => {
                    // compute what to add to the total before
                    // changing the v index
                    column_total += sum_from(vertical_index, vertical_index - rocks);
                    vertical_index = (column_size - i - 1) as u32;
                    rocks = 0;
                }
                'O' => rocks += 1, // Count the rock
                _ => (),
            }
        }

        column_total += sum_from(vertical_index, vertical_index - rocks);
        total += column_total;
    }
    println!("{}", total);
}

fn main() {
    let columns = INPUT.lines().next().unwrap().len();
    let map = Grid::from_vec(INPUT.lines().flat_map(|l| l.chars()).collect(), columns);
    part1(&map);
}
