use std::iter;

use grid::Grid;

const INPUT: &str = include_str!("./input.txt");

fn expand_map(map: &mut Grid<char>) {
    // Expand horizontally
    let rows = map.rows();
    let columns_to_expand = map
        .iter_cols()
        .enumerate()
        .filter_map(|(i, r)| r.into_iter().all(|c| *c != '#').then_some(i))
        .collect::<Vec<_>>();

    for column in columns_to_expand.iter().rev() {
        map.insert_col(*column, iter::repeat('.').take(rows).collect());
    }

    // Expand vertically
    let columns = map.cols();
    let rows_to_expand = map
        .iter_rows()
        .enumerate()
        .filter_map(|(i, r)| r.into_iter().all(|c| *c != '#').then_some(i))
        .collect::<Vec<_>>();

    for row in rows_to_expand.iter().rev() {
        map.insert_row(*row, iter::repeat('.').take(columns).collect());
    }
}

fn sum_of_shortest_paths(galaxies: &[(usize, usize)]) -> u64 {
    let mut sum_of_shortest_paths = 0;
    for (i_index, i) in galaxies.iter().enumerate() {
        for j in galaxies.iter().skip(i_index + 1) {
            let shortest_path = i.0.abs_diff(j.0) + i.1.abs_diff(j.1);
            sum_of_shortest_paths += shortest_path as u64;
        }
    }

    sum_of_shortest_paths
}

fn main() {
    let row_length = INPUT.lines().next().unwrap().len();
    let mut map = Grid::from_vec(INPUT.lines().flat_map(|r| r.chars()).collect(), row_length);
    // for row in map.iter_rows() {
    //     for c in row {
    //         print!("{}", c);
    //     }
    //     println!();
    // }
    expand_map(&mut map);
    // for row in map.iter_rows() {
    //     for c in row {
    //         print!("{}", c);
    //     }
    //     println!();
    // }

    let mut galaxies = Vec::new();
    for (row_index, row) in map.iter_rows().enumerate() {
        for (column_index, c) in row.enumerate() {
            if *c == '#' {
                galaxies.push((column_index, row_index));
            }
        }
    }

    println!("{}", sum_of_shortest_paths(&galaxies));

}
