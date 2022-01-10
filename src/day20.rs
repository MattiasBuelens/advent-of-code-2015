#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> u32 {
    input.parse().unwrap()
}

#[aoc(day20, part1)]
pub fn part1(&target_sum: &u32) -> u32 {
    // The highest possible house number with this sum is when the house number is a prime: p.
    // (For the worst-case scenario, we'll assume the house number is actually prime.)
    // The number of presents delivered equals:
    // sum = 10 + 10 * p
    // => house <= ceil(sum / 10)
    let end = div_ceil(target_sum, 10);
    for house in 1..=end {
        let sum = 10 * divisors(house).sum::<u32>();
        if sum >= target_sum {
            return house;
        }
    }
    panic!("no house found with sum {}", target_sum);
}

fn div_ceil(dividend: u32, divisor: u32) -> u32 {
    (dividend / divisor) + (if dividend % divisor == 0 { 0 } else { 1 })
}

fn divisors(n: u32) -> impl Iterator<Item = u32> {
    let max = (n as f64).sqrt().floor() as u32;
    (1..=max)
        .flat_map(move |div| {
            IntoIterator::into_iter(if div * div == n {
                [Some(div), None]
            } else if n % div == 0 {
                [Some(div), Some(n / div)]
            } else {
                [None, None]
            })
        })
        .flatten()
}

#[aoc(day20, part2)]
pub fn part2(&target_sum: &u32) -> u32 {
    for house in 1.. {
        let sum = divisors(house)
            .filter(|divisor| {
                // Each Elf will stop after delivering presents to 50 houses.
                // If there are more than 50 houses before this one, then this house gets nothing.
                house / divisor <= 50
            })
            .sum::<u32>()
            * 11;
        if sum >= target_sum {
            return house;
        }
    }
    panic!("no house found with sum {}", target_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&10), 1);
        assert_eq!(part1(&11), 2);
        assert_eq!(part1(&30), 2);
        assert_eq!(part1(&31), 3);
        assert_eq!(part1(&40), 3);
        assert_eq!(part1(&41), 4);
        assert_eq!(part1(&60), 4);
        assert_eq!(part1(&70), 4);
        assert_eq!(part1(&71), 6);
    }
}
