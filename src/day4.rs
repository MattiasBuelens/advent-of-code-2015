#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> String {
    input.to_string()
}

fn solve(secret: &str, success: impl Fn(&[u8; 16]) -> bool) -> i32 {
    for num in 1.. {
        let mut input = String::from(secret);
        input.push_str(&num.to_string());
        let hash = md5::compute(input.as_bytes());
        if success(&hash.into()) {
            return num;
        }
    }
    panic!("no number found");
}

#[aoc(day4, part1)]
pub fn part1(secret: &str) -> i32 {
    solve(secret, starts_with_five_zeros)
}

fn starts_with_five_zeros(digest: &[u8; 16]) -> bool {
    matches!(digest, [0, 0, x, ..] if *x < 0x10)
}

#[aoc(day4, part2)]
pub fn part2(secret: &str) -> i32 {
    solve(secret, starts_with_six_zeros)
}

fn starts_with_six_zeros(digest: &[u8; 16]) -> bool {
    matches!(digest, [0, 0, 0, ..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("abcdef"), 609043);
        assert_eq!(part1("pqrstuv"), 1048970);
    }
}
