use grid::Grid;

const INPUT: &str = include_str!("./input.txt");

// We seed this with the starting position as prev and one of the two
// connecting pipes as the current
fn next_location(
    grid: &Grid<char>,
    position: &(u32, u32),
    previous_position: &(u32, u32),
) -> (u32, u32) {
    let c = grid.get(position.1 as usize, position.0 as usize).unwrap();
    let relative_pos = (
        previous_position.0 as i32 - position.0 as i32,
        previous_position.1 as i32 - position.1 as i32,
    );
    let motion = match (c, relative_pos) {
        ('|', (0, -1)) => (0, 1),
        ('|', (0, 1)) => (0, -1),

        ('L', (0, -1)) => (1, 0),
        ('L', (1, 0)) => (0, -1),

        ('J', (0, -1)) => (-1, 0),
        ('J', (-1, 0)) => (0, -1),

        ('7', (-1, 0)) => (0, 1),
        ('7', (0, 1)) => (-1, 0),

        ('F', (1, 0)) => (0, 1),
        ('F', (0, 1)) => (1, 0),

        ('-', (1, 0)) => (-1, 0),
        ('-', (-1, 0)) => (1, 0),

        (p, r) => panic!("Bad pipe direction {}: {:?}", p, r),
    };

    (
        (position.0 as i32 + motion.0) as u32,
        (position.1 as i32 + motion.1) as u32,
    )
}

fn get_all_connections(grid: &Grid<char>, position: &(u32, u32)) -> [(u32, u32); 2] {
    let mut output = Vec::new();

    let north = grid
        .get(position.1 as usize - 1, position.0 as usize)
        .unwrap();
    match north {
        '|' => output.push((position.0, position.1 - 1)),
        '7' => output.push((position.0, position.1 - 1)),
        'F' => output.push((position.0, position.1 - 1)),
        _ => (),
    }

    let west = grid
        .get(position.1 as usize, position.0 as usize - 1)
        .unwrap();
    match west {
        'L' => output.push((position.0 - 1, position.1)),
        '-' => output.push((position.0 - 1, position.1)),
        'F' => output.push((position.0 - 1, position.1)),
        _ => (),
    }

    let east = grid
        .get(position.1 as usize, position.0 as usize + 1)
        .unwrap();
    match east {
        '-' => output.push((position.0 + 1, position.1)),
        '7' => output.push((position.0 + 1, position.1)),
        'J' => output.push((position.0 + 1, position.1)),
        _ => (),
    }

    let south = grid
        .get(position.1 as usize + 1, position.0 as usize)
        .unwrap();
    match south {
        '|' => output.push((position.0, position.1 + 1)),
        'J' => output.push((position.0, position.1 + 1)),
        'L' => output.push((position.0, position.1 + 1)),
        _ => (),
    }

    output.try_into().unwrap()
}

fn main() {
    let columns = INPUT.lines().next().unwrap().len();
    let flat_data = INPUT.lines().flat_map(|l| l.chars()).collect();
    let map = Grid::from_vec(flat_data, columns);

    // Find S
    let mut start_position = (0_u32, 0_u32);
    for (row, columns) in map.iter_rows().enumerate() {
        for (col, c) in columns.enumerate() {
            if *c == 'S' {
                start_position = (col as u32, row as u32);
            }
        }
    }

    let a = get_all_connections(&map, &start_position);
    println!("{:?}", a);

    let mut steps = 1;
    let mut cursor1_prev = start_position;
    let mut cursor1 = a[0];

    let mut cursor2_prev = start_position;
    let mut cursor2 = a[1];
    while cursor1 != cursor2 {
        let cursor1_next = next_location(&map, &cursor1, &cursor1_prev);
        cursor1_prev = cursor1;
        cursor1 = cursor1_next;

        let cursor2_next = next_location(&map, &cursor2, &cursor2_prev);
        cursor2_prev = cursor2;
        cursor2 = cursor2_next;
        steps += 1;
    }

    println!("{}", steps);
}
