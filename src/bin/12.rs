#![allow(unused_variables)]

advent_of_code::solution!(12);


use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};
use std::fmt;

struct SumNumbers(i64);

impl<'de> Deserialize<'de> for SumNumbers {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SumVisitor;

        impl<'de> Visitor<'de> for SumVisitor {
            type Value = i64;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any JSON structure containing numbers")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(0)
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(v)
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(v as i64)
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut total = 0;
                while let Some(elem) = seq.next_element_seed(SumSeed)? {
                    total += elem;
                }
                Ok(total)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut total = 0;
                while let Some((_k, v)) = map.next_entry_seed(SumSeed, SumSeed)? {
                    total += v;
                }
                Ok(total)
            }
        }

        struct SumSeed;

        impl<'de> de::DeserializeSeed<'de> for SumSeed {
            type Value = i64;

            fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_any(SumVisitor)
            }
        }

        let sum = deserializer.deserialize_any(SumVisitor)?;
        Ok(SumNumbers(sum))
    }
}

struct SumNumbersNoRed(i64);
impl<'de> Deserialize<'de> for SumNumbersNoRed {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SumVisitor;

        impl<'de> Visitor<'de> for SumVisitor {
            type Value = i64;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any JSON structure containing numbers")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if v == "red" {
                    return Err(de::Error::custom("red"))
                }
                Ok(0)
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(v)
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(v as i64)
            }
            
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut total = 0;

                loop {
                    match seq.next_element_seed(SumSeed) {
                        Ok(Some(val)) => total += val,
                        Ok(None) => break,
                        Err(e) => {
                            if e.to_string().starts_with("red") {
                                continue;
                            } else {
                                return Err(e);
                            }
                        }
                    }
                }

                Ok(total)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut total = 0;
                let mut red = false;

                loop {
                    match map.next_entry_seed(SumSeed, SumSeed) {
                        Ok(Some((_k, v))) => {
                            total += v;
                        }
                        Ok(None) => break,
                        Err(e) => {
                            if e.to_string().starts_with("red") {
                                red = true;
                                continue;
                            } else {
                                return Err(e);
                            }
                        }
                    }
                }
                if red {
                    Ok(0)
                } else {
                    Ok(total)
                }
            }
        }

        struct SumSeed;

        impl<'de> de::DeserializeSeed<'de> for SumSeed {
            type Value = i64;

            fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_any(SumVisitor)
            }
        }

        let sum = deserializer.deserialize_any(SumVisitor)?;
        Ok(SumNumbersNoRed(sum))
    }
}


pub fn part_one(input: &str) -> Option<u64> {
    let result: SumNumbers = serde_json::from_str(input).unwrap();
    Some(result.0 as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let result: SumNumbersNoRed = serde_json::from_str(input).unwrap();
    Some(result.0 as u64)
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
