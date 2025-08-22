use ndarray::{Array2, ArrayView2};
use std::
    collections::HashMap
;
advent_of_code::solution!(9);

fn hki(graph: ArrayView2<u64>, initial: u64, remainder: u64, invert: bool) -> u64 {
    let mut best = 0;
    let mut rc = remainder;

    while rc != 0 {
        let next = rc.trailing_zeros() as u64;
        rc &= !(1 << next);

        let cost = graph[(initial as usize, next as usize)]
            + hki(graph, next, remainder & !(1 << next), invert);

        best = if best == 0 {
            cost
        } else {
            if invert {
                best.max(cost)
            } else {
                best.min(cost)
            }
            
        }
    }

    best
}

fn hk(graph: ArrayView2<u64>, invert: bool) -> u64 {
    let nc = graph.nrows();
    dbg!(graph);
    let remainder= (1u64 << nc) - 1;

    let iter = (0..nc).map(|initial| hki(graph, initial as u64, remainder ^ (1 << initial), invert));

    if invert {
        iter.max().expect("Empty graph")
    } else {
        iter.min().expect("Empty graph")
    }
}

pub fn parse(input: &str) -> Option<ndarray::ArrayBase<ndarray::OwnedRepr<u64>, ndarray::Dim<[usize; 2]>>> {
    let mut d: HashMap<u64, HashMap<u64, u64>> = HashMap::new();
    let mut count: u64 = 0;
    let mut atoi: HashMap<String, u64> = HashMap::new();

    for line in input.lines() {
        let (l, r) = line.split_once(" = ")?;
        let (s, e) = l.split_once(" to ")?;
        let s_i = *atoi.entry(s.into()).or_insert_with(|| {
            count += 1;
            count - 1
        });
        let e_i = *atoi.entry(e.into()).or_insert_with(|| {
            count += 1;
            count - 1
        });
        d.entry(s_i)
            .or_default()
            .insert(e_i, r.parse::<u64>().ok()?);
    }

    let mut graph = Array2::<u64>::zeros((count as usize, count as usize));
    for (s, r) in d.into_iter() {
        for (e, c) in r.into_iter() {
            graph[[s as usize, e as usize]] = c;
            graph[[e as usize, s as usize]] = c;
        }
    }
    Some(graph)
}

pub fn part_one(input: &str) -> Option<u64> {
    let graph = parse(input)?;
    let result = hk(graph.view(), false);
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let graph = parse(input)?;
    let result = hk(graph.view(), true);
    Some(result)
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
