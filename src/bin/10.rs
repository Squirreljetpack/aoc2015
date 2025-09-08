

advent_of_code::solution!(10);

// also, itertools.group_by
pub fn lookandsay(digits: Vec<u8>) -> Vec<u8> {
    let mut it = digits.into_iter();
    let mut cn = it.next().unwrap();
    let mut cl = 1;
    let mut res = Vec::new();

    for n in it {
        if n == cn {
            cl += 1
        } else {
            res.push(cl);
            res.push(cn);
            cn = n;
            cl = 1;
        }
    }
    res.push(cl);
    res.push(cn);
    res
}

pub fn part_one(input: &str) -> Option<u64> {
    dbg!(input);

    let mut digits: Vec<u8> = input.trim()
        .chars()
        .map(|c| c.to_digit(10).map(|d| d as u8))
        .collect::<Option<Vec<u8>>>()?;

    for _ in 0..40 {
        digits = lookandsay(digits);
    }

    Some(digits.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut digits: Vec<u8> = input.trim()
        .chars()
        .map(|c| c.to_digit(10).map(|d| d as u8))
        .collect::<Option<Vec<u8>>>()?;

    for _ in 0..50 {
        digits = lookandsay(digits);
    }

    Some(digits.len() as u64)
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
