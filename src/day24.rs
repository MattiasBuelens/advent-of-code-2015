use std::iter::{empty, once};

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn split_into_groups_of_length(
    weights: &[i32],
    group_length: usize,
) -> Box<dyn Iterator<Item = (Vec<i32>, Vec<i32>)> + '_> {
    if group_length == 0 {
        Box::new(once((Vec::new(), weights.to_vec())))
    } else if group_length == weights.len() {
        Box::new(once((weights.to_vec(), Vec::new())))
    } else if group_length > weights.len() {
        Box::new(empty())
    } else {
        debug_assert!(group_length > 0);
        debug_assert!(group_length < weights.len());
        let (first, rest) = weights.split_first().unwrap();
        let first_in_group =
            split_into_groups_of_length(rest, group_length - 1).map(|(mut group, rest)| {
                group.push(*first);
                (group, rest)
            });
        let first_in_rest =
            split_into_groups_of_length(rest, group_length).map(|(group, mut rest)| {
                rest.push(*first);
                (group, rest)
            });
        Box::new(first_in_group.chain(first_in_rest))
    }
}

fn split_into_groups_of_sum(
    weights: &[i32],
    sum: i32,
) -> impl Iterator<Item = (Vec<i32>, Vec<i32>)> + '_ {
    (1..=weights.len())
        .flat_map(|group_length| split_into_groups_of_length(weights, group_length))
        .filter(move |(group, _)| group.iter().sum::<i32>() == sum)
}

fn quantum_entanglement(weights: &[i32]) -> i64 {
    weights.iter().map(|&x| x as i64).product::<i64>()
}

#[aoc(day24, part1)]
pub fn part1(input: &[i32]) -> i64 {
    let group_weight = input.iter().sum::<i32>() / 3;
    for group_length in 1..=input.len() {
        let best = split_into_groups_of_length(input, group_length)
            .filter(|(first_group, rest)| {
                // First group must have correct sum
                first_group.iter().sum::<i32>() == group_weight
                // Must be able to split other weights into second and third group
                && split_into_groups_of_sum(rest, group_weight).next().is_some()
            })
            .min_by_key(|(first_group, _)| quantum_entanglement(first_group));
        if let Some((first_group, _)) = best {
            return quantum_entanglement(&first_group);
        }
    }
    panic!("no solution found")
}

#[aoc(day24, part2)]
pub fn part2(input: &[i32]) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_into_group() {
        assert_eq!(
            split_into_groups_of_length(&[1, 2, 3], 1).collect::<Vec<_>>(),
            vec![
                (vec![1], vec![2, 3]),
                (vec![2], vec![3, 1]),
                (vec![3], vec![2, 1]),
            ]
        );
        assert_eq!(
            split_into_groups_of_length(&[1, 2, 3], 2).collect::<Vec<_>>(),
            vec![
                (vec![2, 1], vec![3]),
                (vec![3, 1], vec![2]),
                (vec![2, 3], vec![1]),
            ]
        );
        assert_eq!(
            split_into_groups_of_length(&[1, 2, 3], 3).collect::<Vec<_>>(),
            vec![(vec![1, 2, 3], vec![])]
        );
    }

    #[test]
    fn test_part1() {
        let input = (1..=5).chain(7..=11).collect::<Vec<i32>>();
        assert_eq!(part1(&input), 99);
    }
}
