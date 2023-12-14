use itertools::Itertools;

const INPUT: &str = include_str!("./input.txt");

#[derive(Debug)]
struct Line<'a> {
    arrangement: &'a str,
    groupings: Vec<u32>,
}

impl<'a> From<&'a str> for Line<'a> {
    fn from(line: &'a str) -> Self {
        let mut iter = line.split(' ');
        let arrangement = iter.next().unwrap();
        let groupings = iter
            .next()
            .unwrap()
            .split(',')
            .map(|c| c.parse().unwrap())
            .collect();
        Self {
            arrangement,
            groupings,
        }
    }
}

impl<'a> Line<'a> {
    fn number_of_valid_arrangements(&self, permutation_cache: &[Vec<Vec<char>>]) -> u32 {
        // generate permutations of the raw unknown sequence (This is a candidate to cache)
        let unknown_count = self.arrangement.chars().filter(|c| *c == '?').count();
        let permutations = permutation_cache.get(unknown_count).unwrap();

        // Now build a string and check its validity
        let mut valid_arrangements = 0;
        for permutation in permutations {
            let mut built_string = String::with_capacity(self.arrangement.len());

            let mut permute_iter = permutation.iter();
            for c in self.arrangement.chars() {
                if c == '?' {
                    built_string.push(*permute_iter.next().unwrap());
                } else {
                    built_string.push(c);
                }
            }

            if self.is_valid(&built_string) {
                valid_arrangements += 1;
            }
        }

        valid_arrangements
    }

    fn is_valid(&self, s: &str) -> bool {
        let groups = s
            .chars()
            .group_by(|c| *c == '#')
            .into_iter()
            .filter_map(|(k, g)| k.then_some(g.collect::<String>()))
            .map(|g| g.len() as u32)
            .collect::<Vec<u32>>();

        groups == self.groupings
    }
}

fn main() {
    let lines = INPUT.lines().map(Line::from).collect::<Vec<_>>();

    let set_elements = ['.', '#'];
    let mut permutation_cache = Vec::new();
    for i in 0..19 {
        let a = itertools::repeat_n(set_elements, i)
            .multi_cartesian_product()
            .collect::<Vec<_>>();
        permutation_cache.push(a);
    }

    let result = lines
        .iter()
        .map(|l| l.number_of_valid_arrangements(&permutation_cache))
        .sum::<u32>();

    println!("{}", result);
}
