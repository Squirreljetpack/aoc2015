

advent_of_code::solution!(16);

use itertools::Itertools;
use phf::phf_map;

static SUE_CONSTRAINT: phf::Map<&'static str, u8> = phf_map! {
    "children" => 3,
    "cats" => 7,
    "samoyeds" => 2,
    "pomeranians" => 3,
    "akitas" => 0,
    "vizslas" => 0,
    "goldfish" => 5,
    "trees" => 3,
    "cars" => 2,
    "perfumes" => 1,
};

pub fn parse(line: &str) -> impl Iterator<Item = (&str, u8)> {
    line.split_ascii_whitespace().skip(2).tuples().filter_map(|(k, v)| {
        Some((k.trim_end_matches(':'), v.trim_end_matches(',').parse::<u8>().ok()?))
    })
}

pub fn check<'a> (mut iter: impl Iterator<Item = (&'a str, u8)>) -> bool {
    iter.all(|(k, v)| *SUE_CONSTRAINT.get(k).unwrap() == v)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut count = 1;
    for line in input.lines() {
        if check(parse(line)) {
            return Some(count)
        }
        count = count + 1;
    }
    None        
}

pub fn check2<'a> (mut iter: impl Iterator<Item = (&'a str, u8)>) -> bool {
    iter.all(|(k, v)| {
            match k {
                "cats" | "trees" => *SUE_CONSTRAINT.get(k).unwrap() < v,
                "pomeranians" | "goldfish" => *SUE_CONSTRAINT.get(k).unwrap() > v,
                _ => *SUE_CONSTRAINT.get(k).unwrap() == v
            }
            
        }
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut count = 1;
    for line in input.lines() {
        if check2(parse(line)) {
            return Some(count)
        }
        count = count + 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
