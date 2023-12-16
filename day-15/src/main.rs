const INPUT: &str = include_str!("./input.txt");

fn hash_string2(s: &str) -> u8 {
    let mut result: u8 = 0;
    for c in s.as_bytes() {
        result = result.wrapping_add(*c);
        result = result.wrapping_mul(17);
    }
    println!("{}: {}", s, result);
    result
}

fn main() {
    let result: u32 = INPUT
        .trim()
        .split(',')
        .map(|s| hash_string2(s) as u32)
        .sum();
    println!("{}", result);
}
