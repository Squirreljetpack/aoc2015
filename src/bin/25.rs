#![allow(unused_variables, unused_macros)]

use aoc_lib::parse::parse_first_line;

advent_of_code::solution!(25);

macro_rules! debug_eprintln {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            eprintln!($($arg)*);
        }
    };
}

fn pow(mut base: u64, mut exp: usize, modulo: u64) -> u64 {
    let mut result = 1;
    base %= modulo;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % modulo;
        }
        base = base * base % modulo;
        exp >>= 1;
    }
    result
}

static N: u64 = 20151125;
static B: u64 = 252533;
static M: u64 = 33554393;

pub fn parse(s: &str) -> usize {
    let x: [usize; 2] = parse_first_line(s); // row, column
                                             // 3, 4 -> T_5 + 4
    let n = x[0] + x[1] - 2;
    let t_n = (n * (n + 1)) / 2;
    let ret = t_n + x[1];
    debug_eprintln!("{x:?}, {ret}");

    ret - 1
}

pub fn part_one(input: &str) -> Option<u64> {
    let iters = parse(input);

    let mut ret = pow(B, iters, M);
    ret = (ret * N) % M;

    Some(ret)
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
        assert_eq!(result, Some(31916031));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
