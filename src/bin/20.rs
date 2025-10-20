#![allow(unused_variables, unused_macros)]

advent_of_code::solution!(20);

macro_rules! debug_eprintln {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            eprintln!($($arg)*);
        }
    };
}

pub fn part_one_orig(input: &str) -> Option<u64> {
    let target = input.trim().parse::<usize>().unwrap() / 10;

    let mut arr = vec![1; target];

    let mut curr = 2;
    let mut upper = target;

    while curr < upper {
        for i in (curr - 1..upper).step_by(curr) {
            arr[i] += curr;
            if arr[i] >= target {
                upper = i;
                break;
            }
        }
        curr += 1;
    }

    Some(upper as u64 + 1)
}

// apparently faster
pub fn part_one(input: &str) -> Option<u64> {
    let target = input.trim().parse::<usize>().unwrap() / 10;

    let mut arr = vec![1; target];

    let mut curr = 2;

    while curr < target {
        for i in (curr - 1..target).step_by(curr) {
            arr[i] += curr;
        }
        curr += 1;
    }

    Some(
        arr.iter()
            .enumerate()
            .find_map(|(i, &n)| {
                if n >= target {
                    Some(i as u64 + 1)
                } else {
                    None
                }
            })
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let target = input.trim().parse::<usize>().unwrap();

    let mut arr = vec![11; target / 11];

    let mut curr = 2;

    while curr < target {
        for i in (curr - 1..arr.len()).step_by(curr).take(50) {
            arr[i] += curr * 11;
        }
        curr += 1;
    }

    Some(
        arr.iter()
            .enumerate()
            .find_map(|(i, &n)| {
                if n >= target {
                    Some(i as u64 + 1)
                } else {
                    None
                }
            })
            .unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
