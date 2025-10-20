#![allow(unused_variables, unused_macros)]

use itertools::iproduct;

// todo: trace with tracing

advent_of_code::solution!(18);

macro_rules! debug_eprintln {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            eprintln!($($arg)*);
        }
    };
}

type Grid = Vec<Vec<bool>>;
type UGrid = Vec<Vec<u64>>;

pub fn parse(input:&str) -> Vec<Vec<bool>> {
    let mut ret = Vec::new();
    for l in input.lines() {
        let row = l.chars().map(|f| f == '#').collect();
        ret.push(row);
    }
    ret
}

pub fn neighbors(
    m: usize,
    n: usize,
    i: usize,
    j: usize,
) -> impl Iterator<Item = (usize, usize)> {
    match (i, j) {
        (0, 0) => vec![(0, 1), (1, 0), (1, 1)],
        (0, y) if y + 1 == n => vec![(0, y - 1), (1, y - 1), (1, y)],
        (x, 0) if x + 1 == m => vec![(x - 1, 0), (x - 1, 1), (x, 1)],
        (x, y) if x + 1 == m && y + 1 == n => vec![(x - 1, y - 1), (x - 1, y), (x, y - 1)],
        (0, y) => vec![(0, y - 1), (0, y + 1), (1, y - 1), (1, y), (1, y + 1)],
        (x, 0) => vec![(x - 1, 0), (x + 1, 0), (x - 1, 1), (x, 1), (x + 1, 1)],
        (x, y) if y + 1 == n => vec![(x - 1, y), (x + 1, y), (x - 1, y - 1), (x, y - 1), (x + 1, y - 1)],
        (x, y) if x + 1 == m => vec![(x, y - 1), (x, y + 1), (x - 1, y - 1), (x - 1, y), (x - 1, y + 1)],
        (x, y) => vec![
            (x - 1, y - 1), (x - 1, y), (x - 1, y + 1),
            (x, y - 1),                 (x, y + 1),
            (x + 1, y - 1), (x + 1, y), (x + 1, y + 1),
        ],
    }
    .into_iter()
}

pub fn clear(scratch: &mut UGrid) {
    for row in scratch.iter_mut() {
        for v in row.iter_mut() {
            *v = 0;
        }
    }
}

pub fn count(grid: &Grid) -> usize {
    grid.iter().flatten().filter(|&&b| b).count()
}

pub fn transition(gridv: bool, scratchv: u64) -> bool {
    match (gridv, scratchv) {
        (true, 2) | (true, 3) => true,
        (true, _) => false,
        (false, 3) => true,
        (false , _) => false,
    }
}

pub fn step(grid: &mut Grid, scratch: &mut UGrid) {
    let m = grid.len();
    let n = grid[0].len();
    clear(scratch);
    for (i, j) in iproduct!(0..m, 0..n) {
        if grid[i][j] {
            for (x, y) in neighbors(m, n, i, j) {
                scratch[x][y] += 1;
            }
        }
    }
    for (i, j) in iproduct!(0..m, 0..n) {
        grid[i][j] = transition(grid[i][j], scratch[i][j])
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut grid = parse(&input);
    let m = grid.len();
    let n = grid[0].len();
    let mut scratch = vec![vec![0; n]; m];
    for _ in 0..100 {
        step(&mut grid, &mut scratch)
    }
    Some(count(&grid) as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = parse(&input);
    let m = grid.len();
    let n = grid[0].len();
    let mut scratch = vec![vec![0; n]; m];
    for _ in 0..100 {
        step(&mut grid, &mut scratch);
        grid[0][0] = true;
        grid[0][n-1] = true;
        grid[m-1][0] = true;
        grid[m-1][n-1] = true;
    }
    Some(count(&grid) as u64)
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