advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let mut i = 0;
    loop {
        i += 1;
        let key = format!("{}{}", input.trim(), i);
        let digest = md5::compute(key);
        // 4 bits per hex
        if digest[0] == 0 && digest[1] == 0 && (digest[2] >> 4 == 0) {
            return Some(i as u64);
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut i = 0;
    loop {
        i += 1;
        let key = format!("{}{}", input.trim(), i);
        let digest = md5::compute(key);
        if digest[0] == 0 && digest[1] == 0 && digest[2] == 0 {
            return Some(i as u64);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(609043));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(1048970));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
