#![allow(unused_variables, unused_macros)]

use std::u64;

advent_of_code::solution!(22);

macro_rules! debug_eprintln {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            eprintln!($($arg)*);
        }
    };
}

fn get_num(s: &str) -> impl Iterator<Item = u16> + '_ {
    s.split_whitespace().filter_map(|w| w.parse().ok())
}

pub fn parse(s: &str) -> State {
    let mut lines = s.lines();
    let b_health = get_num(lines.next().unwrap()).next().unwrap();
    unsafe { B_DMG = get_num(lines.next().unwrap()).next().unwrap() };

    State {
        b_health,
        health: unsafe { P_HEALTH },
        mana: unsafe { P_MANA },
        ..Default::default()
    }
}

static mut B_DMG: u16 = 0;
static mut P_HEALTH: u16 = 50;
static mut P_MANA: u16 = 500;
static MISSILE: u16 = 4;
static DRAIN: u16 = 2;
static POISON: u16 = 3;
static SHIELD: u16 = 7;
static RECHARGE: u16 = 101;

#[derive(Debug, Default, Clone)]
pub struct State {
    b_health: u16,
    health: u16,
    mana: u16,
    poison_t: u16,
    recharge_t: u16,
    shield_t: u16,
    mana_total: u64,
    turn: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Spell {
    SHIELD,
    POISON,
    RECHARGE,
    MISSILE,
    DRAIN,
}

const SPELL_COSTS: [u16; 5] = [113, 173, 229, 53, 73];

impl Spell {
    pub const fn cost(self) -> u16 {
        SPELL_COSTS[self as usize]
    }
}

impl State {
    pub fn spells(&self) -> Vec<Spell> {
        let mut ret = Vec::new();

        if self.mana >= 53 {
            ret.push(Spell::MISSILE);
        }

        if self.mana >= 73 {
            ret.push(Spell::DRAIN);
        }

        if self.mana >= 113 {
            if self.shield_t <= 1 {
                ret.push(Spell::SHIELD);
            }
        }
        // tick happens before

        if self.mana >= 173 {
            if self.poison_t <= 1 {
                ret.push(Spell::POISON);
            }
        }

        if 229 + 173 >= self.mana && self.mana >= 229 {
            if self.recharge_t <= 1 {
                ret.push(Spell::RECHARGE);
            }
        }

        ret.sort();
        ret
    }

    pub fn spell(&mut self, s: Spell) {
        match s {
            Spell::SHIELD => self.shield_t = 6,
            Spell::POISON => self.poison_t = 6,
            Spell::RECHARGE => self.recharge_t = 5,
            Spell::DRAIN => {
                self.health += DRAIN;
                dec(&mut self.b_health, DRAIN);
            }
            Spell::MISSILE => {
                dec(&mut self.b_health, MISSILE);
            }
        }

        let cost = s.cost();
        self.mana -= cost;
        self.mana_total += cost as u64;
    }

    pub fn tick(&mut self) {
        if self.poison_t > 0 {
            dec(&mut self.b_health, POISON);
        }
        if self.recharge_t > 0 {
            self.mana += RECHARGE;
        }

        dec(&mut self.poison_t, 1);
        dec(&mut self.recharge_t, 1);
        dec(&mut self.shield_t, 1);
    }

    pub fn b_turn(&mut self) -> Result<(), u64> {
        self.tick();

        if self.b_health <= 0 {
            return Err(self.mana_total);
        }

        let dmg = unsafe {
            let mut dmg = B_DMG;
            if self.shield_t > 0 {
                dmg -= SHIELD;
            }
            dmg
        };

        if self.health > dmg {
            self.health -= dmg;
            Ok(())
        } else {
            Err(0)
        }
    }

    pub fn p_turn(&mut self, s: Spell) -> Result<(), u64> {
        self.tick();
        if self.b_health <= 0 {
            return Err(self.mana_total);
        }

        self.spell(s);
        Ok(())
    }

    pub fn process(mut self, s: Spell) -> Result<Self, u64> {
        self.p_turn(s)?;
        self.b_turn()?;
        self.turn += 1;
        Ok(self)
    }
}

fn dec(t: &mut u16, n: u16) {
    *t = t.saturating_sub(n);
}

fn dfs<const HARD: bool>(start: State) -> u64 {
    let mut min = u64::MAX;
    let mut iterations = 0;
    // dequeue for
    let mut states = vec![(Spell::DRAIN, start)];

    // Some kind of lower bound of remaining mana needed would greatly improve pruning
    while let Some((s, mut curr)) = states.pop() {
        iterations += 1;
        if HARD {
            if curr.health > 1 {
                dec(&mut curr.health, 1);
            } else {
                continue;
            }
        };
        if curr.turn > 0 {
            debug_eprintln!("{}{s:?}", " ".repeat(curr.turn - 1));
        }

        // debug_eprintln!("---------\n{curr:?} {min}\n---------");

        let spells = curr.spells();
        for s in spells {
            let next = curr.clone().process(s.clone());

            match next {
                Ok(state) if state.mana_total < min => {
                    // debug_eprintln!("{state:?}");
                    states.push((s, state));
                }
                Err(c) if c != 0 => {
                    min = min.min(c);
                    debug_eprintln!(
                        "{}{s:?}\n{}{c}",
                        " ".repeat(curr.turn + 1),
                        " ".repeat(curr.turn + 1)
                    );
                }
                _ => {
                    // debug_eprintln!("{}pruned", " ".repeat(curr.turn));
                    continue;
                }
            }
        }
    }

    debug_eprintln!("{}", iterations);

    min
}

pub fn part_one(input: &str) -> Option<u64> {
    let start = parse(input);
    let ret = dfs::<false>(start);

    Some(ret)
}

pub fn part_two(input: &str) -> Option<u64> {
    let start = parse(input);
    let ret = dfs::<true>(start);

    Some(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        unsafe { P_HEALTH = 10 };
        unsafe { P_MANA = 250 };
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(226));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
