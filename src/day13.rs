use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug)]
pub struct Relation(String, String, i32);

impl FromStr for Relation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Examples:
        //   Alice would gain 54 happiness units by sitting next to Bob.
        //   Alice would lose 79 happiness units by sitting next to Carol.
        let (first, s) = s.split_once(" would ").unwrap();
        let (s, last) = s
            .split_once(" happiness units by sitting next to ")
            .unwrap();
        let second = last.strip_suffix(".").unwrap();
        if let Some(gain) = s.strip_prefix("gain ") {
            let gain = gain.parse::<i32>().unwrap();
            Ok(Relation(first.to_string(), second.to_string(), gain))
        } else if let Some(loss) = s.strip_prefix("lose ") {
            let loss = loss.parse::<i32>().unwrap();
            Ok(Relation(first.to_string(), second.to_string(), -loss))
        } else {
            panic!("invalid input: {}", s)
        }
    }
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<Relation> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn compute_happiness(arrangement: &[String], relations: &[Relation]) -> i32 {
    let mut total = 0;
    for (i, person) in arrangement.iter().enumerate() {
        let left = if i == 0 {
            &arrangement[arrangement.len() - 1]
        } else {
            &arrangement[i - 1]
        };
        let right = if i == arrangement.len() - 1 {
            &arrangement[0]
        } else {
            &arrangement[i + 1]
        };
        total += get_relation_happiness(person, left, relations);
        total += get_relation_happiness(person, right, relations);
    }
    total
}

fn get_relation_happiness(person: &String, other: &String, relations: &[Relation]) -> i32 {
    relations
        .iter()
        .find(|Relation(first, second, _)| first == person && other == second)
        .unwrap()
        .2
}

#[aoc(day13, part1)]
pub fn part1(relations: &[Relation]) -> i32 {
    let people = relations
        .iter()
        .map(|Relation(first, _, _)| first.clone())
        .unique()
        .collect::<Vec<_>>();
    let people_count = people.len();
    let all_arrangements = people.into_iter().permutations(people_count);
    let best_arrangement = all_arrangements
        .max_by_key(|arrangement| compute_happiness(&arrangement[..], relations))
        .unwrap();
    compute_happiness(&best_arrangement[..], relations)
}

#[aoc(day13, part2)]
pub fn part2(relations: &[Relation]) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use lazy_static::*;

    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol."
            .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 330);
    }
}
