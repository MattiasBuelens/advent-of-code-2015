use std::collections::HashMap;
use std::str::FromStr;

use lazy_static::*;

#[derive(Debug)]
pub struct Sue {
    number: u32,
    possessions: HashMap<String, u32>,
}

impl FromStr for Sue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (number, s) = s.strip_prefix("Sue ").unwrap().split_once(": ").unwrap();
        let number = number.parse().unwrap();
        let possessions = s
            .split(", ")
            .map(|s| {
                let (compound, amount) = s.split_once(": ").unwrap();
                let compound = compound.to_string();
                let amount = amount.parse().unwrap();
                (compound, amount)
            })
            .collect();
        Ok(Sue {
            number,
            possessions,
        })
    }
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Vec<Sue> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

type Sample<'a> = HashMap<&'a str, u32>;

lazy_static! {
    static ref SAMPLE: Sample<'static> = Sample::from([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]);
}

impl Sue {
    fn matches_part1(&self, sample: &Sample) -> bool {
        self.possessions
            .iter()
            .all(|(compound, amount)| sample.get(compound.as_str()) == Some(amount))
    }
}

#[aoc(day16, part1)]
pub fn part1(input: &[Sue]) -> u32 {
    let gift_sue = input.iter().find(|sue| sue.matches_part1(&SAMPLE)).unwrap();
    gift_sue.number
}

impl Sue {
    fn matches_part2(&self, sample: &Sample) -> bool {
        self.possessions.iter().all(|(compound, &amount)| {
            let sample_amount = *sample.get(compound.as_str()).unwrap();
            match compound.as_str() {
                "cats" | "trees" => amount > sample_amount,
                "pomeranians" | "goldfish" => amount < sample_amount,
                _ => amount == sample_amount,
            }
        })
    }
}

#[aoc(day16, part2)]
pub fn part2(input: &[Sue]) -> u32 {
    let gift_sue = input.iter().find(|sue| sue.matches_part2(&SAMPLE)).unwrap();
    gift_sue.number
}
