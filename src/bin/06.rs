advent_of_code::solution!(6);
// use std::simd::{Simd};
// cargo rustc --release -- --emit=asm -C target-cpu=native

fn parse_coords(s: &str) -> Option<((u32, u32), (u32, u32))> {
    let (a, b) = s.split_once(" through ")?;
    let (x1, y1) = a.split_once(',')?;
    let (x2, y2) = b.split_once(',')?;

    Some((
        (x1.parse().ok()?, y1.parse().ok()?),
        (x2.parse().ok()?, y2.parse().ok()?),
    ))
}


fn toggle_slice(slice: &mut [u8]) {
    for x in slice {
        *x ^= 1;
    }
}

fn set_slice(slice: &mut [u8], value: u8) {
    for x in slice {
        *x = value;
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut grid = [[0u8; 1000]; 1000];
    for line in input.lines() {
        if let Some(line) = line.strip_prefix("turn on ") {
            let ((xl, yl), (xh, yh)) = parse_coords(line).unwrap();
                for y in yl..=yh {
                    let slice = &mut grid[y as usize][xl as usize..=(xh as usize)];
                    set_slice(slice, 1);
                }
        } else if let Some(line) = line.strip_prefix("turn off ") {
            let ((xl, yl), (xh, yh)) = parse_coords(line).unwrap();
                for y in yl..=yh {
                    let slice = &mut grid[y as usize][xl as usize..=(xh as usize)];
                    set_slice(slice, 0);
                }

        } else if let Some(line) = line.strip_prefix("toggle ") {
            let ((xl, yl), (xh, yh)) = parse_coords(line).unwrap();
                for y in yl..=yh {
                    let slice = &mut grid[y as usize][xl as usize..=(xh as usize)];
                    toggle_slice(slice);
                }
        } else {
            panic!();
        }
    }

    let count = grid.iter().flatten().map(|&x| x as u64).sum();
    Some(count)
}

fn on_slice(slice: &mut [u16]) {
    for x in slice {
        *x += 1;
    }
}

fn off_slice(slice: &mut [u16]) {
    for x in slice {

        *x = std::cmp::max(*x, 1) - 1;
    }
}

fn toggle_slice2(slice: &mut [u16]) {
    for x in slice {
        *x += 2;
    }
}


pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = [[0u16; 1000]; 1000];
    for line in input.lines() {
        if let Some(line) = line.strip_prefix("turn on ") {
            let ((xl, yl), (xh, yh)) = parse_coords(line).unwrap();
                for y in yl..=yh {
                    let slice = &mut grid[y as usize][xl as usize..=(xh as usize)];
                    on_slice(slice);
                }
        } else if let Some(line) = line.strip_prefix("turn off ") {
            let ((xl, yl), (xh, yh)) = parse_coords(line).unwrap();
                for y in yl..=yh {
                    let slice = &mut grid[y as usize][xl as usize..=(xh as usize)];
                    off_slice(slice);
                }

        } else if let Some(line) = line.strip_prefix("toggle ") {
            let ((xl, yl), (xh, yh)) = parse_coords(line).unwrap();
                for y in yl..=yh {
                    let slice = &mut grid[y as usize][xl as usize..=(xh as usize)];
                    toggle_slice2(slice);
                }
        } else {
            panic!();
        }
    }

    let count = grid.iter().flatten().map(|&x| x as u64).sum();
    Some(count)
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
            assert_eq!(result, Some(1_000_000));
        }
        {
            let result = part_one(&advent_of_code::template::read_file_part(
                "examples", DAY, 2,
            ));
            assert_eq!(result, Some(1000));
        }
        {
            let result = part_one(&advent_of_code::template::read_file_part(
                "examples", DAY, 3,
            ));
            assert_eq!(result, Some(1_000_000 - 4));
        }
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
