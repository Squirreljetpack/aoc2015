#![allow(unused_variables, unused_macros)]

use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use rand::{rngs::SmallRng, Rng};

advent_of_code::solution!(19);

macro_rules! debug_eprintln {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            eprintln!($($arg)*);
        }
    };
}

type Rules = HashMap<Rc<str>, Vec<Rc<str>>>;

pub fn parse(input: &str) -> (Rules, String) {
    let mut ret: HashMap<Rc<str>, Vec<Rc<str>>> = Rules::new();
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if let Some((pat, repl)) = line.split_once(" => ") {
            ret.entry(pat.into()).or_default().push(repl.into());
        } else {
            break;
        }
    }

    (ret, lines.next().unwrap().into())
}

fn split_atoms(formula: &str) -> Vec<&str> {
    let mut atoms = Vec::new();
    let mut start = 0;

    for (i, c) in formula.char_indices() {
        if c.is_uppercase() && i != 0 {
            atoms.push(&formula[start..i]);
            start = i;
        }
    }

    if start < formula.len() {
        atoms.push(&formula[start..]);
    }

    atoms
}

type RevRules<'a> = HashMap<Vec<&'a str>, Vec<&'a str>>;

pub fn parse_2(input: &'_ str) -> (RevRules<'_>, Vec<&'_ str>) {
    let mut ret = RevRules::new();
    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        if let Some((pat, repl)) = line.split_once(" => ") {
            let repl = split_atoms(repl);
            ret.entry(repl).or_default().push(pat.into());
        } else {
            break;
        }
    }

    let input = lines.next().unwrap();

    (ret, split_atoms(input))
}

// bitap would be cool but the input is non-overlapping!
pub fn part_one(input: &str) -> Option<u64> {
    let (rules, input) = parse(input);
    let mut products = HashSet::new();
    for (pat, repls) in &rules {
        for (i, _) in input.match_indices(pat.as_ref()) {
            for r in repls {
                products.insert(format!("{}{}{}", &input[..i], r, &input[i + pat.len()..]));
            }
        }
    }

    let ret = products.len();
    Some(ret as u64)
}

// It would seem we can add production rules by splitting results which have >2 elements, then apply CYK
// The initialization stage would be the trickiest since we dont have terminal sequences
// although all sequences are either xRn..Ar or => XX so maybe something can be done with that

// No production is a substring of any other but i don't see a way to exploit this: It does not guarantee that Greedy is sure to work: the first (inverse) replacement could in fact want its second section to come from the end of a different replacement starting from that second section

thread_local! {
    static RNG: std::cell::RefCell<SmallRng>  = std::cell::RefCell::new(<SmallRng as rand::SeedableRng>::seed_from_u64(12345));
}

pub fn part_two(input: &str) -> Option<u64> {
    let (rules, orig_input) = parse_2(input);
    let rules_vec: Vec<_> = rules.keys().collect();
    let all_indices: Vec<usize> = (0..rules_vec.len()).collect();

    let mut trial = 0;

    let ret = 'outer: loop {
        let mut next_input = orig_input.clone();
        let mut input = Vec::new();

        let mut count = 0;
        let mut iterations = 0;
        trial += 1;

        while input != next_input {
            input = next_input;
            next_input = Vec::new();

            let mut filtered_indices = all_indices.clone();
            let mut start = 0;

            for i in 0..input.len() {
                let c = input[i];

                filtered_indices = filtered_indices
                    .into_iter()
                    .filter(|ix| {
                        let candidate = rules_vec[*ix];
                        i - start < candidate.len() && candidate[i - start] == c
                    })
                    .collect();

                // let dbg = filtered_indices
                //     .iter()
                //     .map(|ix| rules_vec[*ix])
                //     .collect::<Vec<_>>();
                // dbg!(dbg, i);

                let mut reset = false;

                if filtered_indices.len() == 1 {
                    let candidate = rules_vec[filtered_indices[0]];
                    if candidate.len() == i - start + 1 {
                        // dbg!(&input[start..i + 1]);

                        if RNG.with_borrow_mut(|inner| inner.random_bool(0.8)) {
                            next_input.extend(rules.get(candidate).unwrap());
                            start = i + 1;
                            filtered_indices = all_indices.clone();

                            count += 1;
                            // dbg!(&next_input.len(), &next_input.last().unwrap());
                        } else {
                            reset = true;
                        }
                    }
                }

                if reset || filtered_indices.len() == 0 {
                    next_input.extend(&input[start..i]);
                    start = i;
                    filtered_indices = all_indices
                        .clone()
                        .into_iter()
                        .filter(|ix| {
                            let candidate = rules_vec[*ix];
                            candidate[0] == c
                        })
                        .collect();
                }
            }

            if start < input.len() {
                next_input.extend(&input[start..]);
            }

            // dbg!(&next_input);

            iterations += 1;
        }

        debug_eprintln!("Trial {trial} finished in {iterations} iterations");
        if input.len() == 1 {
            break 'outer count;
        }
    };

    Some(ret)
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
