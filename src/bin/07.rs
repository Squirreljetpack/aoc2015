#![allow(unused)]
use std::{cell::RefCell, collections::HashMap};

// use itertools::Itertools;

advent_of_code::solution!(7);

macro_rules! split3 {
    ($s:expr) => {{ // double scope to contain iter
        let mut iter = $s.split_whitespace();
        let l = iter.next().expect("missing");
        if let Some(m) = iter.next() {
            if let Some(r) = iter.next() {
                (l, m, r)
            } else {
                (m, l, "")
            }
        } else {
            (l, "", "")
        }
    }};
}

#[derive(Debug, Clone)]
enum Source {
    Label(String),
    Value(u64)
}

impl From<&str> for Source {
    fn from(s: &str) -> Self {
        if let Ok(num) = s.parse::<u64>() {
            Source::Value(num)
        } else {
            Source::Label(s.to_string())
        }
    }
}

impl Source {
    fn eval(&self, map: &RefCell<HashMap<String, Op>>) -> Option<u64> {
        match self {
            Self::Label(s) => {
                let op = {
                    let borrowed = map.borrow();
                    borrowed.get(s).cloned()? // !important
                };
                let (result, computed) = op.eval(map)?;
                if ! computed {
                    if let Some(v) = map.borrow_mut().get_mut(s) { // update
                        *v = Op::_VAL(result);
                        // *v = Op::SET(Source::Value(result)); // 4x speedup replacing this!
                    }
                }
                Some(result)
            },
            Self::Value(v) => Some(v.clone()),
        }
    }

    fn eval_cache(&self, map: &HashMap<String, Op>, cache: &mut HashMap<String, u64>) -> Option<u64> {
        match self {
            Self::Label(s) => {
                if let Some(cached_value) = cache.get(s) {
                    return Some(*cached_value);
                }
                let op = map.get(s)?;
                let result = op.eval_cache(map, cache)?;
                cache.insert(s.clone(), result);
                Some(result)
            },
            Self::Value(v) => Some(*v),
        }
    }
}

#[derive(Debug, Clone)]
enum Op {
    AND(Source, Source),
    LSHIFT(Source, Source),
    NOT(Source),
    OR(Source, Source),
    RSHIFT(Source, Source),
    SET(Source),
    _VAL(u64)
}

impl Op {
    fn eval(&self, map: &RefCell<HashMap<String, Op>>) -> Option<(u64, bool)> {
        let mut computed = false;
        let result = 
            match self {
                Self::AND(l, r) => l.eval(map)? & r.eval(map)?,
                Self::OR(l, r) => l.eval(map)? | r.eval(map)?,
                Self::LSHIFT(l, r) => l.eval(map)? << r.eval(map)?,
                Self::RSHIFT(l, r) => l.eval(map)? >> r.eval(map)?,
                Self::NOT(l) => ! l.eval(map)?,
                Self::SET(l) => l.eval(map)?,
                Self::_VAL(l) => {
                    computed = true;
                    *l
                },
            };
        Some((result, computed))
    }

    fn eval_cache(&self, map: &HashMap<String, Op>, cache: &mut HashMap<String, u64>) -> Option<u64> {
        let result =
            match self {
                Self::AND(l, r) => l.eval_cache(map, cache)? & r.eval_cache(map, cache)?,
                Self::OR(l, r) => l.eval_cache(map, cache)? | r.eval_cache(map, cache)?,
                Self::LSHIFT(l, r) => l.eval_cache(map, cache)? << r.eval_cache(map, cache)?,
                Self::RSHIFT(l, r) => l.eval_cache(map, cache)? >> r.eval_cache(map, cache)?,
                Self::NOT(l) => !l.eval_cache(map, cache)?,
                Self::SET(l) => l.eval_cache(map, cache)?,
                Self::_VAL(l) => {
                    *l
                },
            };
        Some(result)
    }
}

fn parse_line(line: &str)  -> Option<(String, Op)> {
    let (l, target) = line.split_once(" -> ")?;
    let (l, _op, r) = split3!(l);
    let op = match _op {
        "AND"    => Op::AND(Source::from(l), Source::from(r)),
        "OR"     => Op::OR(Source::from(l), Source::from(r)),
        "LSHIFT" => Op::LSHIFT(Source::from(l), Source::from(r)),
        "RSHIFT" => Op::RSHIFT(Source::from(l), Source::from(r)),
        "NOT"    => Op::NOT(Source::from(l)),
        ""       => {
            if let Ok(u) = l.parse::<u64>() {
                Op::_VAL(u)
            } else {
                Op::SET(Source::from(l))
            }
        }
        _        => {
            eprintln!("Invalid line {line}");
            return None
        },
    };

    Some((target.to_string(), op))
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut d: HashMap<String, Op> = HashMap::new();

    for line in input.lines() {
        let (k, v) = parse_line(line)?;
        // d.entry(k).or_default().push(v); // mutate (not needed because each wire defined once)
        d.insert(k, v);
    }

    // eprintln!("{:#?}", d);

    Source::Label("a".into()).eval(&RefCell::new(d))
}

pub fn part_two(input: &str) -> Option<u64> {
    let val_a = part_one(input)?;

    let mut d: HashMap<String, Op> = HashMap::new();
    for line in input.lines() {
        let (k, v) = parse_line(line)?;
        d.insert(k, v);
    }

    d.insert("b".to_string(), Op::_VAL(val_a));

    let mut cache = HashMap::new();
    Source::Label("a".to_string()).eval_cache(&d, &mut cache)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        {
            let result = part_one(&advent_of_code::template::read_file_part(
                "examples", DAY, 1,
            ));
            assert_eq!(result, Some(72));
        }
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
