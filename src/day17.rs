use std::cmp::Ordering;
use std::iter::{empty, once};

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn ways_to_fill(amount: i32, containers: &[i32]) -> impl Iterator<Item = Vec<i32>> + '_ {
    containers.iter().copied().enumerate().flat_map(
        move |(i, container)| -> Box<dyn Iterator<Item = _>> {
            match container.cmp(&amount) {
                Ordering::Greater => Box::new(empty()),
                Ordering::Equal => Box::new(once(vec![container])),
                Ordering::Less => Box::new(
                    ways_to_fill(amount - container, &containers[i + 1..]).map(move |mut inner| {
                        inner.insert(0, container);
                        inner
                    }),
                ),
            }
        },
    )
}

#[aoc(day17, part1)]
pub fn part1(input: &[i32]) -> usize {
    ways_to_fill(150, input).count()
}

fn ways_to_fill_with_minimum_containers(amount: i32, containers: &[i32]) -> usize {
    let minimum_ways = ways_to_fill(amount, containers)
        .map(|way| way.len())
        .min()
        .unwrap();
    ways_to_fill(amount, containers)
        .filter(|way| way.len() == minimum_ways)
        .count()
}

#[aoc(day17, part2)]
pub fn part2(input: &[i32]) -> usize {
    ways_to_fill_with_minimum_containers(150, input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: [i32; 5] = [20, 15, 10, 5, 5];

    #[test]
    fn test_part1() {
        assert_eq!(
            ways_to_fill(25, &TEST_INPUT).collect::<Vec<_>>(),
            vec![vec![20, 5], vec![20, 5], vec![15, 10], vec![15, 5, 5],]
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(ways_to_fill_with_minimum_containers(25, &TEST_INPUT), 3);
    }
}
