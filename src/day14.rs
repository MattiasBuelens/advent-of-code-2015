use std::str::FromStr;

use lazy_static::*;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Reindeer {
    name: String,
    fly_speed: u32,
    fly_duration: u32,
    rest_duration: u32,
}

lazy_static! {
    static ref INPUT_RE: Regex = Regex::new(
        r"^(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds\.$"
    )
    .unwrap();
}

impl FromStr for Reindeer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = INPUT_RE.captures(s).unwrap();
        Ok(Reindeer {
            name: captures[1].to_string(),
            fly_speed: captures[2].parse().unwrap(),
            fly_duration: captures[3].parse().unwrap(),
            rest_duration: captures[4].parse().unwrap(),
        })
    }
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Reindeer> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[derive(Debug)]
struct ReindeerState {
    reindeer: Reindeer,
    distance: u32,
    is_resting: bool,
    remaining: u32,
    points: u32,
}

impl ReindeerState {
    fn new(reindeer: Reindeer) -> Self {
        Self {
            distance: 0,
            is_resting: false,
            remaining: reindeer.fly_duration,
            reindeer,
            points: 0,
        }
    }

    fn step(&mut self) {
        if !self.is_resting {
            self.distance += self.reindeer.fly_speed;
        }
        self.remaining -= 1;
        if self.remaining == 0 {
            self.is_resting = !self.is_resting;
            self.remaining = if self.is_resting {
                self.reindeer.rest_duration
            } else {
                self.reindeer.fly_duration
            };
        }
    }

    fn award_point(&mut self) {
        self.points += 1;
    }
}

fn simulate(reindeer: &[Reindeer], seconds: usize) -> Vec<ReindeerState> {
    let mut states = reindeer
        .iter()
        .map(|reindeer| ReindeerState::new(reindeer.clone()))
        .collect::<Vec<_>>();
    for _ in 0..seconds {
        for state in states.iter_mut() {
            state.step();
        }
        // At the end of each second, he awards one point to the reindeer currently in the lead.
        // (If there are multiple reindeer tied for the lead, they each get one point.)
        let leader_distance = states.iter().map(|state| state.distance).max().unwrap();
        for state in states.iter_mut() {
            if state.distance == leader_distance {
                state.award_point();
            }
        }
    }
    states
}

#[aoc(day14, part1)]
pub fn part1(input: &[Reindeer]) -> u32 {
    simulate(input, 2503)
        .into_iter()
        .map(|state| state.distance)
        .max()
        .unwrap()
}

#[aoc(day14, part2)]
pub fn part2(input: &[Reindeer]) -> u32 {
    simulate(input, 2503)
        .into_iter()
        .map(|state| state.points)
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use lazy_static::*;

    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
"
        .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        let mut states = simulate(&input, 1000);
        states.sort_by_key(|state| state.distance);
        let [loser, winner]: [ReindeerState; 2] = states.try_into().unwrap();
        assert_eq!(winner.reindeer.name, "Comet");
        assert_eq!(winner.distance, 1120);
        assert_eq!(loser.reindeer.name, "Dancer");
        assert_eq!(loser.distance, 1056);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        let mut states = simulate(&input, 1000);
        states.sort_by_key(|state| state.points);
        let [loser, winner]: [ReindeerState; 2] = states.try_into().unwrap();
        assert_eq!(winner.reindeer.name, "Dancer");
        assert_eq!(winner.points, 689);
        assert_eq!(loser.reindeer.name, "Comet");
        assert_eq!(loser.points, 312);
    }
}
