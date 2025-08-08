advent_of_code::solution!(5);
use itertools::Itertools;

const VOWELS: [char; 5] = ['a','e','i','o','u'];
const FORBIDDEN: [(char, char); 4] = [('a', 'b'), ('c', 'd'), ('p','q'), ('x', 'y')];

pub fn part_one(input: &str) -> Option<u64> {
    let result = input.lines()
        .filter(|x| x.chars().filter(|c| VOWELS.contains(c)).count() >= 3)
        .filter(|x| x.chars().tuple_windows().any(|(a, b)| a == b))
        .filter(|x| ! x.chars().tuple_windows().any(|t| FORBIDDEN.contains(&t)))
        .count();

    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let result = input.lines()
        .filter(|x| has_pair(x))
        .filter(|x| x.chars().tuple_windows().any(|(a, _, c)| a == c))
        .count();

    Some(result as u64)
}

fn has_pair(s: &str) -> bool {
    s.chars().tuple_windows().enumerate().any(|(i, p) : (usize, (char, char))| {
        s.chars().skip(i + 2).tuple_windows().any(|p2| p == p2)
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(1));
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(1));
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 3));
        assert_eq!(result, Some(0));
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 4));
        assert_eq!(result, Some(0));
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 5));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 6));
        assert_eq!(result, Some(1));
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 7));
        assert_eq!(result, Some(1));
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 8));
        assert_eq!(result, Some(0));
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 9));
        assert_eq!(result, Some(0));
    }
}
