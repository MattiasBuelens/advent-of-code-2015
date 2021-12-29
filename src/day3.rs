use std::collections::HashSet;

use crate::util::Vector2D;

pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn step(&self) -> Vector2D {
        match *self {
            Direction::North => Vector2D::new(0, -1),
            Direction::East => Vector2D::new(1, 0),
            Direction::South => Vector2D::new(0, 1),
            Direction::West => Vector2D::new(-1, 0),
        }
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Direction> {
    input
        .chars()
        .map(|c| match c {
            '^' => Direction::North,
            '>' => Direction::East,
            'v' => Direction::South,
            '<' => Direction::West,
            c => panic!("unexpected input: {}", c),
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &[Direction]) -> usize {
    let mut visited = HashSet::<Vector2D>::new();
    let mut position = Vector2D::zero();
    visited.insert(position);
    for dir in input {
        position += dir.step();
        visited.insert(position);
    }
    visited.len()
}

#[aoc(day3, part2)]
pub fn part2(input: &[Direction]) -> usize {
    let mut visited = HashSet::<Vector2D>::new();
    let mut santa = Vector2D::zero();
    let mut robo_santa = Vector2D::zero();
    visited.insert(santa);
    let mut directions = input.into_iter();
    loop {
        if let Some(dir) = directions.next() {
            santa += dir.step();
            visited.insert(santa);
        } else {
            break;
        }
        if let Some(dir) = directions.next() {
            robo_santa += dir.step();
            visited.insert(robo_santa);
        } else {
            break;
        }
    }
    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = input_generator(">");
        assert_eq!(part1(&input), 2);
        let input = input_generator("^>v<");
        assert_eq!(part1(&input), 4);
        let input = input_generator("^v^v^v^v^v");
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn test_part2() {
        let input = input_generator("^v");
        assert_eq!(part2(&input), 3);
        let input = input_generator("^>v<");
        assert_eq!(part2(&input), 3);
        let input = input_generator("^v^v^v^v^v");
        assert_eq!(part2(&input), 11);
    }
}
