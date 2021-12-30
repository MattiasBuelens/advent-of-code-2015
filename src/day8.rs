#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

fn count_unescaped(s: &str) -> usize {
    let mut count = 0;
    let mut chars = s.chars();
    assert_eq!(chars.next().unwrap(), '\"');
    while let Some(c) = chars.next() {
        match c {
            '\\' => match chars.next().unwrap() {
                'x' => {
                    // \x00
                    chars.next().unwrap();
                    chars.next().unwrap();
                }
                '\\' | '\"' => {}
                c => panic!("unexpected escape sequence: \\{}", c),
            },
            '\"' => {
                // end of string
                assert_eq!(chars.next(), None);
                break;
            }
            _ => {} // regular character
        };
        count += 1;
    }
    count
}

#[aoc(day8, part1)]
pub fn part1(input: &[String]) -> usize {
    let count_raw = input.iter().map(|s| s.len()).sum::<usize>();
    let count_unescaped = input.iter().map(|s| count_unescaped(s)).sum::<usize>();
    count_raw - count_unescaped
}

fn count_escaped(s: &str) -> usize {
    let mut count = 0;
    count += 1; // start quote
    for c in s.chars() {
        count += match c {
            '\\' | '\"' => 2, // escape backslashes and quotes
            _ => 1,           // regular character
        };
    }
    count += 1; // end quote
    count
}

#[aoc(day8, part2)]
pub fn part2(input: &[String]) -> usize {
    let count_raw = input.iter().map(|s| s.len()).sum::<usize>();
    let count_escaped = input.iter().map(|s| count_escaped(s)).sum::<usize>();
    count_escaped - count_raw
}

#[cfg(test)]
mod tests {
    use lazy_static::*;

    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r#"
""
"abc"
"aaa\"aaa"
"\x27"
"#
        .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 12);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 19);
    }
}
