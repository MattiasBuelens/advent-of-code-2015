#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

fn is_nice_part1(s: &str) -> bool {
    // It contains at least three vowels (aeiou only)
    let three_vowels = s.chars().filter(|&c| "aeiou".contains(c)).count() >= 3;
    // It contains at least one letter that appears twice in a row
    let consecutive = s
        .chars()
        .zip(s.chars().skip(1))
        .any(|(left, right)| left == right);
    // It does not contain the strings ab, cd, pq, or xy
    let forbidden = ["ab", "cd", "pq", "xy"]
        .into_iter()
        .any(|pat| s.contains(pat));
    three_vowels && consecutive && !forbidden
}

#[aoc(day5, part1)]
pub fn part1(input: &[String]) -> usize {
    input.iter().filter(|s| is_nice_part1(s)).count()
}

fn is_nice_part2(s: &str) -> bool {
    // It contains a pair of any two letters that appears at least twice in the string
    // without overlapping
    let pair_twice = s
        .chars()
        .enumerate()
        .any(|(i, _)| i + 2 < s.len() && s[i + 2..].contains(&s[i..i + 2]));
    // It contains at least one letter which repeats with exactly one letter between them
    let repeat_with_between = s
        .chars()
        .zip(s.chars().skip(2))
        .any(|(left, right)| left == right);
    pair_twice && repeat_with_between
}

#[aoc(day5, part2)]
pub fn part2(input: &[String]) -> usize {
    input.iter().filter(|s| is_nice_part2(s)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(is_nice_part1("ugknbfddgicrmopn"));
        assert!(is_nice_part1("aaa"));
        assert!(!is_nice_part1("jchzalrnumimnmhp"));
        assert!(!is_nice_part1("haegwjzuvuyypxyu"));
        assert!(!is_nice_part1("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_part2() {
        assert!(is_nice_part2("qjhvhtzxzqqjkmpb"));
        assert!(is_nice_part2("xxyxx"));
        assert!(!is_nice_part2("uurcxstgmygtbstg"));
        assert!(!is_nice_part2("ieodomkazucvgmuy"));
    }
}
