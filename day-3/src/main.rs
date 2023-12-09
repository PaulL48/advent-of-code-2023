use grid::Grid;
use itertools::Itertools;
use std::collections::HashSet;

const INPUT: &str = include_str!("./input.txt");

fn generate_box_indices(row: usize, columns: &[usize]) -> Vec<(i32, i32)> {
    let mut indices = Vec::new();
    let row = row as i32;
    // Add ends to box to indices
    let first = *columns.first().unwrap() as i32;
    indices.push((first - 1, row - 1));
    indices.push((first - 1, row));
    indices.push((first - 1, row + 1));

    let last = *columns.last().unwrap() as i32;
    indices.push((last + 1, row - 1));
    indices.push((last + 1, row));
    indices.push((last + 1, row + 1));

    // Add top and bottom
    for column in columns {
        let column = *column as i32;
        indices.push((column, row - 1));
        indices.push((column, row + 1));
    }

    indices
}

fn indices_contain_symbol(
    indices: &[(i32, i32)],
    symbols: &HashSet<char>,
    grid: &Grid<u8>,
) -> bool {
    let grid_size = grid.size();
    for index in indices {
        if *index > (0, 0) && *index < (grid_size.1 as i32, grid_size.0 as i32) {
            if let Some(c) = grid.get(index.1 as usize, index.0 as usize) {
                // println!("{:?}: {}", index, Into::<char>::into(*c));

                if symbols.contains(&Into::<char>::into(*c)) {
                    return true;
                }
            }
        }
    }
    false
}

fn main() {
    let symbols = {
        let mut s = HashSet::new();
        s.insert('!');
        s.insert('#');
        s.insert('$');
        s.insert('%');
        s.insert('&');
        s.insert('*');
        s.insert('-');
        s.insert('+');
        s.insert('=');
        s.insert('/');
        s.insert('@');
        s
    };

    // Get line length
    let line_length = INPUT.lines().next().unwrap().len();
    // println!("Line length is {}", line_length);

    let vec_data = INPUT
        .lines()
        .join("")
        .as_bytes()
        .to_vec();

    //     let test_data = r#"467..114..
    // ...*......
    // ..35..633.
    // ......#...
    // 617*......
    // .....+.58.
    // ..592.....
    // ......755.
    // ...$.*....
    // .664.598..
    // "#;
    //     let vec_data = test_data.as_bytes().iter().copied().collect::<Vec<_>>();

    // parse input into grid
    let grid = Grid::from_vec(vec_data, line_length);

    // iterate over the input
    // when encountering a number, two things must be done
    // parse the number
    // check its surroundings to see if it is valid

    let mut sum: u32 = 0;
    for (row_index, row) in grid.iter_rows().enumerate() {
        for group in row
            .enumerate()
            .group_by(|(_, v)| v.is_ascii_digit())
            .into_iter()
            .filter_map(|(key, g)| key.then_some(g))
        {
            let (columns, digits): (Vec<usize>, Vec<u8>) = group.unzip();
            let s = std::str::from_utf8(&digits).unwrap();
            let box_indices = generate_box_indices(row_index, &columns);
            if indices_contain_symbol(&box_indices, &symbols, &grid) {
                // println!("{:?} is valid", s);
                sum += s.parse::<u32>().unwrap();
            } else {
                // println!("{:?} is invalid: {:?}", s, box_indices);
            }
        }
    }

    println!("sum: {}", sum);
}
