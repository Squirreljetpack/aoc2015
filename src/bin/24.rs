#![allow(unused_variables, unused_macros)]
use std::collections::HashSet;

use aoc_lib::parse::lines_parsed;

advent_of_code::solution!(24);

macro_rules! debug_eprintln {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            eprintln!($($arg)*);
        }
    };
}

fn parse(s: &str) -> Vec<u64> {
    let mut ret = vec![];
    for mut nums in lines_parsed(s) {
        ret.push(nums.next().unwrap());
    }

    ret
}

// type CacheBestMap = HashMap<(u8, u64), Vec<Vec<u64>>>; // (idx, traget), [..Solutions]

type Solutions = HashSet<Vec<u64>>;

fn dfs_impl(arr: &[u64], target: u64, mut hist: Vec<u64>, total: &mut Solutions) {
    if target == 0 {
        total.insert(hist);
        return;
    }
    if arr.len() == 0 {
        return;
    }

    dfs_impl(&arr[1..], target, hist.clone(), total);
    if target >= arr[0] {
        hist.push(arr[0]);
        dfs_impl(&arr[1..], target - arr[0], hist, total);
    }
}

// probably return trees best?
// fn dfs_par_impl(arr: &[u64], target: u64, mut hist: Vec<u64>) {

// }

fn dfs(arr: &[u64], target: u64) -> Solutions {
    let mut total = Solutions::new();
    dfs_impl(arr, target, vec![], &mut total);

    // dfs_par_impl(arr, target, vec![], Arc::new(Mutex::new(total)));

    // debug_eprintln!("{:?}, {:?}, {:?}", &arr, &total, &target);

    total
}

fn filter(mut g1s: Solutions) -> Solutions {
    let mut lengths: Vec<_> = g1s.iter().map(|x| x.len()).collect();
    lengths.sort();

    for ml in lengths {
        let filtered: HashSet<_> = g1s.extract_if(|x| x.len() == ml).collect();
        // debug_eprintln!("{:?}", filtered);

        let mut best: HashSet<Vec<u64>> = HashSet::new();
        let mut best_prod = u64::MAX;

        for x in filtered {
            let prod: u64 = x.iter().product();
            if prod < best_prod {
                best.clear();
                best.insert(x);
                best_prod = prod;
            } else if prod == best_prod {
                best.insert(x);
            }
        }

        // being the smallest, the complement likely contains some group which is disjoint from it in the solutions.
        // It turns out in this case we don't need to enforce the 3 groups constraint.

        if !best.is_empty() {
            debug_eprintln!("best: {:?}", best);
            return best;
        }
    }

    HashSet::new()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut arr = parse(input);
    arr.sort_by(|a, b| b.cmp(a));

    let target = {
        let sum: u64 = arr.iter().sum();
        assert!(sum % 3 == 0);
        sum / 3
    };

    let g1 = dfs(&arr, target);

    let best = filter(g1);
    let ret: u64 = best.iter().next().unwrap().iter().product();

    Some(ret)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut arr = parse(input);
    arr.sort_by(|a, b| b.cmp(a));

    let target = {
        let sum: u64 = arr.iter().sum();
        assert!(sum % 4 == 0);
        sum / 4
    };

    let g1 = dfs(&arr, target);

    let best = filter(g1);
    let ret: u64 = best.iter().next().unwrap().iter().product();

    Some(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(99));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
