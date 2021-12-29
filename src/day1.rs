#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> String {
    input.to_string()
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let up = input.chars().filter(|&c| c == '(').count() as i32;
    let down = input.chars().filter(|&c| c == ')').count() as i32;
    up - down
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let mut position = 0;
    for (i, c) in input.chars().enumerate() {
        position += match c {
            '(' => 1,
            ')' => -1,
            c => panic!("unknown input character: {}", c),
        };
        if position < 0 {
            return i + 1;
        }
    }
    panic!("did not enter basement")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("(())"), 0);
        assert_eq!(part1("((("), 3);
        assert_eq!(part1("(()(()("), 3);
        assert_eq!(part1("))((((("), 3);
        assert_eq!(part1("())"), -1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(")"), 1);
        assert_eq!(part2("()())"), 5);
    }
}
