#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

fn is_nice(s: &str) -> bool {
    let three_vowels = s.chars().filter(|&c| "aeiou".contains(c)).count() >= 3;
    let consecutive = s
        .chars()
        .zip(s.chars().skip(1))
        .any(|(left, right)| left == right);
    let forbidden = ["ab", "cd", "pq", "xy"]
        .into_iter()
        .any(|pat| s.contains(pat));
    three_vowels && consecutive && !forbidden
}

#[aoc(day5, part1)]
pub fn part1(input: &[String]) -> usize {
    input.iter().filter(|s| is_nice(s)).count()
}

#[aoc(day5, part2)]
pub fn part2(input: &[String]) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(is_nice("ugknbfddgicrmopn"));
        assert!(is_nice("aaa"));
        assert!(!is_nice("jchzalrnumimnmhp"));
        assert!(!is_nice("haegwjzuvuyypxyu"));
        assert!(!is_nice("dvszwmarrgswjxmb"));
    }
}
