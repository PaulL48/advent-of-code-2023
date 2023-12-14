use grid::Grid;

const INPUT: &str = include_str!("./input.txt");

fn vertical_reflection_line(grid: &Grid<char>) -> Option<usize> {
    let columns = grid.cols();
    // Here 0 means the reflection line is to the right of the 0th row
    for i in 0..columns - 1 {
        let columns_to_compare = (i + 1).min(columns - (i + 1));
        let mut fully_symmetric = true;
        for n in 0..columns_to_compare {
            let left_compare = i - n;
            let right_compare = i + n + 1;

            let asymmetric = grid
                .iter_col(left_compare)
                .zip(grid.iter_col(right_compare))
                .any(|(l, r)| l != r);

            if asymmetric {
                fully_symmetric = false;
                break;
            }
        }

        if fully_symmetric {
            return Some(i);
        }
    }
    None
}

fn horizontal_reflection_line(grid: &Grid<char>) -> Option<usize> {
    let rows = grid.rows();
    // Here 0 means the reflection line is to the right of the 0th row
    for i in 0..rows - 1 {
        let rows_to_compare = (i + 1).min(rows - (i + 1));
        let mut fully_symmetric = true;
        for n in 0..rows_to_compare {
            let top_compare = i - n;
            let bottom_compare = i + n + 1;

            let asymmetric = grid
                .iter_row(top_compare)
                .zip(grid.iter_row(bottom_compare))
                .any(|(l, r)| l != r);

            if asymmetric {
                fully_symmetric = false;
                break;
            }
        }

        if fully_symmetric {
            return Some(i);
        }
    }
    None
}

fn main() {
    let mut grids = Vec::new();

    for l in INPUT.split("\n\n") {
        let line_length = l.lines().next().unwrap().len();
        let grid = Grid::from_vec(
            l.lines().flat_map(|line| line.chars()).collect(),
            line_length,
        );
        grids.push(grid);
    }

    let mut total = 0;
    for g in grids {
        match (vertical_reflection_line(&g), horizontal_reflection_line(&g)) {
            (Some(vr), None) => total += vr + 1,
            (None, Some(hr)) => total += 100 * (hr + 1),
            e => println!("Bad {:?}", e),
        }
    }

    println!("{}", total);
}
