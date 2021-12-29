#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> String {
    input.to_string()
}

#[aoc(day4, part1)]
pub fn part1(secret: &str) -> i32 {
    for num in 1.. {
        let mut input = String::from(secret);
        input.push_str(&num.to_string());
        let hash = md5::compute(input.as_bytes());
        if starts_with_five_zeros(&hash.into()) {
            return num;
        }
    }
    panic!("no number found");
}

fn starts_with_five_zeros(digest: &[u8; 16]) -> bool {
    match digest {
        [0, 0, x, ..] if *x < 0x10 => true,
        _ => false,
    }
}

#[aoc(day4, part2)]
pub fn part2(secret: &str) -> i32 {
    todo!()
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
