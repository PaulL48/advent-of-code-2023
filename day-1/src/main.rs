const INPUT: &str = include_str!("./input.txt");

fn get_line_value_part1(line: &str) -> u64 {
    // iterate
    let first = line
        .chars()
        .find(|c| c.is_numeric())
        .unwrap()
        .to_digit(10)
        .unwrap();

    let last = line
        .chars()
        .rev()
        .find(|c| c.is_numeric())
        .unwrap()
        .to_digit(10)
        .unwrap();

    (first * 10 + last) as u64
}

// fn get_line_value_part1_2(line: &str) -> u64 {
//     let digits = Vec::new();

// }

fn advance_parser_state<const SIZE: usize>(c: char, parser: &mut (u32, &[u8; SIZE], u32)) -> Option<u32> {
    if c == parser.1[parser.0 as usize] as char {
        parser.0 += 1;
        if parser.0 as usize == SIZE {
            parser.0 = 0; // reset the parser
            return Some(parser.2);
        }
    } else {
        // reset parser (while rechecking to see if we immediately begin a new match)
        if c == parser.1[0] as char {
            parser.0 = 1;
        } else {
            parser.0 = 0;
        }
    }
    None
}

fn get_string_digits(s: &str) -> Vec<u32> {
    // to linearly check which digit it is, we advance a counter for each digit string
    // if it matches the current character and reset it if it doesn't.
    // Once a parser reaches its string length, it is a match
    let mut one = (0, b"one", 1);
    let mut two = (0, b"two", 2);
    let mut three = (0, b"three", 3);
    let mut four = (0, b"four", 4);
    let mut five = (0, b"five", 5);
    let mut six = (0, b"six", 6);
    let mut seven = (0, b"seven", 7);
    let mut eight = (0, b"eight", 8);
    let mut nine = (0, b"nine", 9);

    let mut digits = Vec::new();
    for c in s.chars() {
        // check if it is a digit
        if let Some(digit) = c.to_digit(10) {
            digits.push(digit);

            // reset all parsers
            one.0 = 0;
            two.0 = 0;
            three.0 = 0;
            four.0 = 0;
            five.0 = 0;
            six.0 = 0;
            seven.0 = 0;
            eight.0 = 0;
            nine.0 = 0;
            continue
        }

        if let Some(digit) = advance_parser_state(c, &mut one) {
            digits.push(digit);
        }

        if let Some(digit) = advance_parser_state(c, &mut two) {
            digits.push(digit);
        }

        if let Some(digit) = advance_parser_state(c, &mut three) {
            digits.push(digit);
        }

        if let Some(digit) = advance_parser_state(c, &mut four) {
            digits.push(digit);
        }

        if let Some(digit) = advance_parser_state(c, &mut five) {
            digits.push(digit);
        }

        if let Some(digit) = advance_parser_state(c, &mut six) {
            digits.push(digit);
        }

        if let Some(digit) = advance_parser_state(c, &mut seven) {
            digits.push(digit);
        }

        if let Some(digit) = advance_parser_state(c, &mut eight) {
            digits.push(digit);
        }

        if let Some(digit) = advance_parser_state(c, &mut nine) {
            digits.push(digit);
        }
    }

    digits
}

fn get_line_value_part2(line: &str) -> u64 {
    let digits = get_string_digits(line);
    (digits.first().unwrap() * 10 + digits.last().unwrap()) as u64
}

fn main() {
    // let result = INPUT.lines().map(get_line_value_part1).sum::<u64>();
    // println!("Part 1 result is {result}");

    let result = INPUT.lines().map(get_line_value_part2).sum::<u64>();
    println!("Part 2 result is {result}");
}
