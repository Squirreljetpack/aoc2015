

use std::collections::HashMap;

use num_integer::Integer;

advent_of_code::solution!(14);

pub struct ContestantStats { v: u64, l: u64, r: u64 }
type Contestants = HashMap<String,ContestantStats>;

pub fn parse(input: &str) -> Option<Contestants> {
    let mut contestants = HashMap::new();

    for line in input.lines() {
        let line = line.trim_end_matches('.');

        let (name, rest) = line.split_once(" can fly ")?;

        let (speed_str, rest) = rest.split_once(" km/s for ")?;
        let v = speed_str.parse::<u64>().ok()?;

        let (duration_str, rest) = rest.split_once(" seconds, but then must rest for ")?;
        let l = duration_str.parse::<u64>().ok()?;

        let r = rest.split_once(" seconds").map(|(r, _)| r)?.parse::<u64>().ok()?;

        contestants.insert(name.to_string(), ContestantStats {v, l, r});
    }

    Some(contestants)
}

pub fn compute(duration: u64, stats: &ContestantStats) -> u64 {
    let ContestantStats {v, l, r} = stats;
    let (q, rem) = duration.div_rem(&(l+r));
    q * l * v + l.min(&rem)*v
}

pub fn ahead(ds: &[u64], stats: &ContestantStats) -> ([u64; 2503], bool) {
    let mut theirs = [0u64; 2503];
    let mut t = 0usize;
    let mut d = 0;
    let mut ahead = 0;
    while t < 2503 {
        if (t as u64) % (stats.l + stats.r) < stats.l {
            d += stats.v;
        }
        t += 1;
        theirs[t] = d;

        if ds[t] > d {
            ahead += 1
        } else if d < ds[t] {
            ahead -= 1
        }
    }

    (theirs, ahead < 0)
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
    let contestants = parse(input)?;
    let n = contestants.len();
    let mut ds = vec![0; n];
    let mut ss = vec![0u64; n];
    for t in 0..2503 {
        for (idx, stat) in contestants.values().enumerate() {
            if (t as u64) % (stat.l + stat.r) < stat.l {
                ds[idx] += stat.v;
            }
        }
        let max = ds.iter().max().unwrap();
        for i in ds.iter()
            .enumerate()
            .filter_map(|(i, &val)|
                if val == *max { Some(i) } else { None }
            ) {
                ss[i] += 1
            }
    }
    ss.iter().max().copied()
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
