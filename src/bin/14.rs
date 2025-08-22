#![allow(unused_variables)]

use std::collections::HashMap;

use num_integer::Integer;

advent_of_code::solution!(14);

type ContestantStats = (u64, u64, u64);
type Contestants = HashMap<String,ContestantStats>;

pub fn parse(input: &str) -> Option<Contestants> {
    let mut contestants = HashMap::new();

    for line in input.lines() {
        let line = line.trim_end_matches('.');

        let (name, rest) = line.split_once(" can fly ")?;

        let (speed_str, rest) = rest.split_once(" km/s for ")?;
        let speed = speed_str.parse::<u64>().ok()?;

        let (duration_str, rest) = rest.split_once(" seconds, but then must rest for ")?;
        let duration = duration_str.parse::<u64>().ok()?;

        let rest_time = rest.split_once(" seconds").map(|(r, _)| r)?.parse::<u64>().ok()?;

        contestants.insert(name.to_string(), (speed, duration, rest_time));
    }

    Some(contestants)
}

pub fn compute(duration: u64, stats: &ContestantStats) -> u64 {
    let (v, t, r) = stats;
    let (q, rem) = duration.div_rem(&(t+r));
    q * t * v + t.min(&rem)*v
}

pub fn ahead(ds: &[u32; 2503], stats2: &ContestantStats) -> ([u32; 2503], bool) {
    todo!()
}

pub fn part_one(input: &str) -> Option<u64> {
    let contestants = parse(input)?;
    contestants
        .values()
        .map(|stats| {
            compute(2503, stats)
        })
        .max()
}

pub fn part_two(input: &str) -> Option<u64> {
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
