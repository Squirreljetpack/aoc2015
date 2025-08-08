advent_of_code::solution!(3);

use bitvec::prelude::*;
use std::collections::VecDeque;

// struct BitVec128 {
//     bits: u128,
// }

// impl BitVec128 {
//     fn new() -> Self {
//         Self { bits: 0 }
//     }

//     fn to_pos(index: i32) -> Result<u32, String> {
//         if index < -64 || index > 63 {
//             return Err(format!("Index {index} out of bounds (-64 to 63)"));
//         }
//         Ok((index + 64) as u32)
//     }

//     fn set(&mut self, index: i32) -> Result<(), String> {
//         let i = Self::to_pos(index)?;
//         self.bits |= 1u128 << i;
//         Ok(())
//     }

// fn unset(&mut self, index: i32) -> Result<(), String> {
//     let i = Self::to_pos(index)?;
//     self.bits &= !(1u128 << i);
//     Ok(())
// }

// fn toggle(&mut self, index: i32) -> Result<(), String> {
//     let i = Self::to_pos(index)?;
//     self.bits ^= 1u128 << i;
//     Ok(())
// }

// fn is_set(&self, index: i32) -> Result<bool, String> {
//     let i = Self::to_pos(index)?;
//     Ok((self.bits & (1u128 << i)) != 0)
// }

//     fn count(&self) -> u32 {
//         self.bits.count_ones()
//     }
// }

pub fn part_one(input: &str) -> Option<u64> {
    let (mut x, mut y, mut z): (u32, u32, bool) = (0, 128, false);
    let mut d = VecDeque::new();
    let mut bv = bitvec![0; 256];
    bv.set(y as usize, true);
    d.push_back(bv);
    for i in input.bytes() {
        // eprintln!("{}, {x}, {y}", i as char);
        match i {
            b'>' => x += 1,
            b'^' => y += 1,
            b'<' => {
                if x == 0 {
                    z = true
                } else {
                    x -= 1
                }
            }
            b'v' => y -= 1,
            _ => panic!(),
        }
        step(x, y, z, &mut d);
        z = false;
    }
    let result = d.iter().map(|bv| bv.count_ones() as u64).sum();
    Some(result)
}

fn step(x: u32, y: u32, z: bool, d: &mut VecDeque<BitVec>) {
    if x >= d.len().try_into().unwrap() {
        let mut bv = bitvec![0; 256];
        bv.set(y as usize, true);
        d.push_back(bv);
    } else if z {
        let mut bv = bitvec![0; 256];
        bv.set(y as usize, true);
        d.push_front(bv);
    } else {
        d[x.try_into().unwrap()].set(y as usize, true);
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    eprintln!("-----");
    let (mut x, mut y, mut z): (u32, u32, bool) = (0, 128, false);
    let (mut a, mut b, mut c): (u32, u32, bool) = (0, 128, false);
    let mut d = VecDeque::new();
    let mut bv = bitvec![0; 256];
    bv.set(y as usize, true);
    d.push_back(bv);
    let mut even = true;
    for i in input.bytes() {
        if even {
            // eprintln!("{}, {x}, {y}", i as char);
            match i {
                b'>' => x += 1,
                b'^' => y += 1,
                b'<' => {
                    if x == 0 {
                        z = true;
                        a += 1
                    } else {
                        x -= 1
                    }
                }
                b'v' => y -= 1,
                _ => panic!(),
            }
            step(x, y, z, &mut d);
            z = false;
        } else {
            // eprintln!("{}, {a}, {b}", i as char);
            match i {
                b'>' => a += 1,
                b'^' => b += 1,
                b'<' => {
                    if a == 0 {
                        c = true;
                        x += 1;
                    } else {
                        a -= 1
                    }
                }
                b'v' => b -= 1,
                _ => panic!(),
            }
            step(a, b, c, &mut d);
            c = false;
        }

        even = !even;
    }
    let result = d.iter().map(|bv| bv.count_ones() as u64).sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(4));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(3));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(3));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 6,
        ));
        assert_eq!(result, Some(11));
    }
}
