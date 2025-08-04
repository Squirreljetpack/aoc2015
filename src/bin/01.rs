advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let result = input.bytes().fold(0, |acc, c| {
        match c {
            b'(' => acc + 1,
            b')' => acc - 1,
            _ => acc,
        }
    });
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut floor = 0;
    for (i, c) in input.bytes().enumerate() {
        match c {
            b'(' => floor += 1,
            b')' => floor -= 1,
            _ => {}
        }
        if floor == -1 {
            return Some((i + 1) as u32);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(0));
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(0));
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 3));
        assert_eq!(result, Some(3));
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 4));
        assert_eq!(result, Some(3));
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 5));
        assert_eq!(result, Some(3));
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 6));
        assert_eq!(result, Some(-1));
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 7));
        assert_eq!(result, Some(-1));
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 8));
        assert_eq!(result, Some(-3));
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 9));
        assert_eq!(result, Some(-3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 10));
        assert_eq!(result, Some(1));
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 11));
        assert_eq!(result, Some(5));
    }
}
