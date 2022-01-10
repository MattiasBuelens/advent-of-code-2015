use std::iter::{empty, once};

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn split_into_groups_of_sum_and_length(
    weights: &[i32],
    sum: i32,
    group_length: usize,
) -> Box<dyn Iterator<Item = (Vec<i32>, Vec<i32>)> + '_> {
    if group_length == 0 {
        if sum == 0 {
            Box::new(once((Vec::new(), weights.to_vec())))
        } else {
            Box::new(empty())
        }
    } else if group_length == weights.len() {
        if sum == weights.iter().sum::<i32>() {
            Box::new(once((weights.to_vec(), Vec::new())))
        } else {
            Box::new(empty())
        }
    } else if group_length > weights.len() {
        Box::new(empty())
    } else {
        debug_assert!(group_length > 0);
        debug_assert!(group_length < weights.len());
        let (&first, rest) = weights.split_first().unwrap();
        if first <= sum {
            let first_in_group =
                split_into_groups_of_sum_and_length(rest, sum - first, group_length - 1).map(
                    move |(mut group, rest)| {
                        group.push(first);
                        (group, rest)
                    },
                );
            let first_in_rest = split_into_groups_of_sum_and_length(rest, sum, group_length).map(
                move |(group, mut rest)| {
                    rest.push(first);
                    (group, rest)
                },
            );
            Box::new(first_in_group.chain(first_in_rest))
        } else {
            Box::new(empty())
        }
    }
}

fn split_into_groups_of_sum(
    weights: &[i32],
    sum: i32,
) -> impl Iterator<Item = (Vec<i32>, Vec<i32>)> + '_ {
    (1..=weights.len())
        .flat_map(move |group_length| {
            split_into_groups_of_sum_and_length(weights, sum, group_length)
        })
        .map(move |(group, rest)| {
            assert_eq!(group.iter().sum::<i32>(), sum);
            (group, rest)
        })
}

fn can_split_into_two_groups_of_sum(weights: &[i32], sum: i32) -> bool {
    split_into_groups_of_sum(weights, sum).next().is_some()
}

fn quantum_entanglement(weights: &[i32]) -> i64 {
    weights.iter().map(|&x| x as i64).product::<i64>()
}

#[aoc(day24, part1)]
pub fn part1(input: &[i32]) -> i64 {
    let group_weight = input.iter().sum::<i32>() / 3;
    for group_length in 1..=input.len() {
        let best = split_into_groups_of_sum_and_length(input, group_weight, group_length)
            .filter(|(first_group, rest)| {
                // First group must have correct sum
                first_group.iter().sum::<i32>() == group_weight
                // Must be able to split other weights into two groups with same sum
               && can_split_into_two_groups_of_sum(rest, group_weight)
            })
            .min_by_key(|(first_group, _)| quantum_entanglement(first_group));
        if let Some((first_group, _)) = best {
            return quantum_entanglement(&first_group);
        }
    }
    panic!("no solution found")
}

fn can_split_into_three_groups_of_sum(weights: &[i32], sum: i32) -> bool {
    split_into_groups_of_sum(weights, sum)
        .any(|(_, rest)| can_split_into_two_groups_of_sum(&rest, sum))
}

#[aoc(day24, part2)]
pub fn part2(input: &[i32]) -> i64 {
    let group_weight = input.iter().sum::<i32>() / 4;
    for group_length in 1..=input.len() {
        let best = split_into_groups_of_sum_and_length(input, group_weight, group_length)
            .filter(|(first_group, rest)| {
                // First group must have correct sum
                first_group.iter().sum::<i32>() == group_weight
                    // Must be able to split other weights into three groups with same sum
                    && can_split_into_three_groups_of_sum(rest, group_weight)
            })
            .min_by_key(|(first_group, _)| quantum_entanglement(first_group));
        if let Some((first_group, _)) = best {
            return quantum_entanglement(&first_group);
        }
    }
    panic!("no solution found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_into_group() {
        assert_eq!(
            split_into_groups_of_sum_and_length(&[1, 2, 3], 3, 1).collect::<Vec<_>>(),
            vec![(vec![3], vec![2, 1]),]
        );
        assert_eq!(
            split_into_groups_of_sum_and_length(&[1, 2, 3], 3, 2).collect::<Vec<_>>(),
            vec![(vec![2, 1], vec![3]),]
        );
        assert_eq!(
            split_into_groups_of_sum_and_length(&[1, 2, 3], 6, 3).collect::<Vec<_>>(),
            vec![(vec![1, 2, 3], vec![])]
        );
    }

    #[test]
    fn test_part1() {
        let input = (1..=5).chain(7..=11).collect::<Vec<i32>>();
        assert_eq!(part1(&input), 99);
    }

    #[test]
    fn test_part2() {
        let input = (1..=5).chain(7..=11).collect::<Vec<i32>>();
        assert_eq!(part2(&input), 44);
    }
}
