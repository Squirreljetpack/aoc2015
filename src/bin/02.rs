advent_of_code::solution!(2);

use std::collections::BinaryHeap;
use std::fmt::Debug;

fn smallest_k<T: Ord + Clone + Debug>(vec: &[T], k: usize) -> Vec<T> {
    let mut heap = BinaryHeap::with_capacity(k);

    for (i, val) in vec.iter().enumerate() {
        let entry = (val.clone(), i);
        if heap.len() < k {
            heap.push(entry);
        } else if heap.peek().unwrap() > &entry {
            heap.pop();
            heap.push(entry);
        }
    }

    let result: Vec<T> = heap.into_iter().map(|(val, _)| val).collect();
    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut sa = 0;
    for line in input.lines() {
        let ss: Vec<u64> = line.split("x").map(|x| x.parse::<u64>().unwrap()).collect();
        let areas = vec![ss[0] * ss[1], ss[0] * ss[2], ss[1] * ss[2]];
        sa += areas.iter().min().unwrap();
        sa += 2 * areas.iter().sum::<u64>();
    }
    Some(sa)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sa = 0;
    for line in input.lines() {
        let ss: Vec<u32> = line.split("x").map(|x| x.parse::<u32>().unwrap()).collect();
        let min = smallest_k(&ss, 2);
        sa += 2 * min.iter().sum::<u32>();
        sa += ss.iter().product::<u32>();
    }
    Some(sa)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(58));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(43));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(34));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(14));
    }
}
