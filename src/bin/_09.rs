#![allow(warnings)]

use ndarray::{Array2, ArrayView2};
use std::{cmp::Ordering, collections::{BinaryHeap, HashMap}};

advent_of_code::solution!(9);

#[derive(PartialEq, PartialOrd, Eq)]
struct Node {
    cost: u64,
    index: usize,
}

// priority queue

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.index.cmp(&self.index))
    }
}

// impl Index<[usize; 2]> allows [(index)] -> [[index]]

fn bidirectional_djikstra(graph: ArrayView2<u64>, start: usize, end: usize) -> Option<u64> {
    let nn = graph.len();

    // current best known
    let mut ds1 = vec![u64::MAX; nn];
    let mut ds2 = vec![u64::MAX; nn];

    let mut pq1 = BinaryHeap::new();
    let mut pq2 = BinaryHeap::new();

    ds1[start] = 0;
    ds2[end] = 0;

    pq1.push(Node {
        cost: 0,
        index: start,
    });
    pq2.push(Node {
        cost: 0,
        index: end,
    });

    let mut mu = u64::MAX;

    while !pq1.is_empty() || !pq2.is_empty() {
        let Node { cost, index } = pq1.pop()?;
        if cost > ds1[index] {
            continue; // a better path was/will be processed
        }
        if ds2[index] < u64::MAX {
            mu = mu.min(cost + ds2[index])
        }
        for (ni, w) in graph.row(index).iter().enumerate() {
            let nc = cost.saturating_add(*w);
            if nc < ds1[ni] {
                ds1[ni] = nc;
                pq1.push(Node {
                    cost: nc,
                    index: ni,
                })
            }
        }

        let Node { cost, index } = pq2.pop()?;
        if cost > ds2[index] {
            continue; // a better path was/will be processed
        }
        if ds1[index] < u64::MAX {
            mu = mu.min(cost + ds1[index])
        }
        for (ni, w) in graph.row(index).iter().enumerate() {
            let nc = cost.saturating_add(*w);
            if nc < ds2[ni] {
                ds2[ni] = nc;
                pq2.push(Node {
                    cost: nc,
                    index: ni,
                })
            }
        }

        // no shorter path can be found
        if mu != u64::MAX && mu <= pq1.peek()?.cost + pq2.peek()?.cost {
            return Some(mu);
        }
    }

    eprintln!("emptied");
    Some(mu)
}

fn parse_lines() {
    todo!()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut d: HashMap<String, HashMap<String, u64>> = HashMap::new();
    for line in input.lines() {
        let (l, r) = line.split_once(" = ")?;
        let (s, e) = l.split_once(" to ")?;
        d.entry(l.to_string()).or_default().insert(e.to_string(), r.parse::<u64>().ok()?);
        
         
    }
    todo!()
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
