#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> String {
    input.to_string()
}

fn look_and_say(digits: &str) -> String {
    let mut digits = digits.chars();
    let mut current = digits.next().unwrap();
    let mut count = 1u32;
    let mut output = String::new();

    for c in digits {
        if current == c {
            count += 1;
        } else {
            output.push_str(&count.to_string());
            output.push(current);
            count = 1;
            current = c;
        }
    }
    output.push_str(&count.to_string());
    output.push(current);

    output
}

#[aoc(day10, part1)]
pub fn part1(input: &str) -> usize {
    let mut s = input.to_string();
    for _ in 0..40 {
        s = look_and_say(&s);
    }
    s.len()
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> usize {
    let mut s = input.to_string();
    for _ in 0..50 {
        s = look_and_say(&s);
    }
    s.len()
}

#[cfg(test)]
mod tests {
    use lazy_static::*;

    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"".trim();
    }

    #[test]
    fn test_part1() {
        let _input = input_generator(&TEST_INPUT);
        assert_eq!(look_and_say("1"), "11");
        assert_eq!(look_and_say("11"), "21");
        assert_eq!(look_and_say("21"), "1211");
        assert_eq!(look_and_say("1211"), "111221");
        assert_eq!(look_and_say("111221"), "312211");
    }
}
