use std::collections::HashSet;

use crate::util::Vector2D;

pub enum Direction {
    North,
    East,
    South,
    West,
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
        position += match *dir {
            Direction::North => Vector2D::new(0, -1),
            Direction::East => Vector2D::new(1, 0),
            Direction::South => Vector2D::new(0, 1),
            Direction::West => Vector2D::new(-1, 0),
        };
        visited.insert(position);
    }
    visited.len()
}

#[aoc(day3, part2)]
pub fn part2(input: &[Direction]) -> i32 {
    todo!()
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
}
