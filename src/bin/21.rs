#![allow(unused_variables, unused_macros)]

use std::u64;

use itertools::{iproduct, Itertools};

advent_of_code::solution!(21);

macro_rules! debug_eprintln {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            eprintln!($($arg)*);
        }
    };
}

fn get_num(s: &str) -> impl Iterator<Item = u64> + '_ {
    s.split_whitespace().filter_map(|w| w.parse::<u64>().ok())
}

pub fn parse(s: &str) -> Stats {
    let mut lines = s.lines();
    Stats {
        health: get_num(lines.next().unwrap()).next().unwrap(),
        dmg: get_num(lines.next().unwrap()).next().unwrap(),
        armor: get_num(lines.next().unwrap()).next().unwrap(),
    }
}

#[derive(Debug)]
pub struct Stats {
    dmg: u64,
    armor: u64,
    health: u64,
}

const WEAPONS: &[(u64, u64, u64)] = &[(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)];

const ARMORS: &[(u64, u64, u64)] = &[
    (0, 0, 0),
    (13, 0, 1),
    (31, 0, 2),
    (53, 0, 3),
    (75, 0, 4),
    (102, 0, 5),
];

const RINGS: &[(u64, u64, u64)] = &[
    (0, 0, 0),
    (25, 1, 0),
    (50, 2, 0),
    (100, 3, 0),
    (20, 0, 1),
    (40, 0, 2),
    (80, 0, 3),
];

#[derive(Debug)]
pub struct Choice {
    w: usize,
    a: usize,
    r: (usize, usize),
}

impl Choice {
    pub fn compute(&self) -> (u64, u64, u64) {
        let w = WEAPONS[self.w];
        let a = ARMORS[self.a];
        let r0 = RINGS[self.r.0];
        let r1 = RINGS[self.r.1];

        (
            w.0 + a.0 + r0.0 + r1.0,
            w.1 + a.1 + r0.1 + r1.1,
            w.2 + a.2 + r0.2 + r1.2,
        )
    }
}

impl Stats {
    pub fn wins_against(&self, other: &Self) -> bool {
        let my_dps = Self::dps_against(self, other);
        let other_dps = Self::dps_against(other, self);

        let my_survive = self.health.div_ceil(other_dps);
        let other_survive = other.health.div_ceil(my_dps);

        my_survive >= other_survive // we go first
    }

    fn dps_against(&self, other: &Self) -> u64 {
        self.dmg.saturating_sub(other.armor).max(1)
    }

    fn new(health: u64) -> Self {
        Self {
            health,
            dmg: 0,
            armor: 0,
        }
    }

    fn with(&self, dmg: u64, armor: u64) -> Self {
        Self {
            health: self.health,
            dmg,
            armor,
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let boss = parse(input);
    let p = Stats::new(100);
    let mut bc = u64::MAX;

    for (w, a, r) in iproduct!(
        0..WEAPONS.len(),
        0..ARMORS.len(),
        (0..RINGS.len()).combinations(2)
    ) {
        let choice = Choice {
            w,
            a,
            r: (r[0], r[1]),
        };
        let (cost, dmg, armor) = choice.compute();
        if cost < bc {
            let cp = p.with(dmg, armor);
            if cp.wins_against(&boss) {
                debug_eprintln!("{:?}: c: {}, {}, {}, {:?}", cp, cost, w, a, r);
                bc = cost;
            }
        }
    }

    Some(bc)
}

// is the lesson here meant to be that the scale aoc operates on favors brute force over dp/optimization?
pub fn part_two(input: &str) -> Option<u64> {
    let boss = parse(input);
    let p = Stats::new(100);
    let mut bc = 0;

    for (w, a, r) in iproduct!(
        0..WEAPONS.len(),
        0..ARMORS.len(),
        (0..RINGS.len()).combinations(2)
    ) {
        let choice = Choice {
            w,
            a,
            r: (r[0], r[1]),
        };
        let (cost, dmg, armor) = choice.compute();
        if cost > bc {
            let cp = p.with(dmg, armor);
            if !cp.wins_against(&boss) {
                debug_eprintln!("{:?}: c: {}, {}, {}, {:?}", cp, cost, w, a, r);
                bc = cost;
            }
        }
    }

    Some(bc)
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
