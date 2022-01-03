use std::cmp::Ordering;

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn ways_to_fill(amount: i32, containers: &[i32]) -> u64 {
    containers
        .iter()
        .enumerate()
        .map(|(i, &container)| match container.cmp(&amount) {
            Ordering::Greater => 0,
            Ordering::Equal => 1,
            Ordering::Less => ways_to_fill(amount - container, &containers[i + 1..]),
        })
        .sum()
}

#[aoc(day17, part1)]
pub fn part1(input: &[i32]) -> u64 {
    ways_to_fill(150, &input)
}

#[aoc(day17, part2)]
pub fn part2(input: &[i32]) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: [i32; 5] = [20, 15, 10, 5, 5];

    #[test]
    fn test_part1() {
        assert_eq!(ways_to_fill(25, &TEST_INPUT), 4);
    }
}
