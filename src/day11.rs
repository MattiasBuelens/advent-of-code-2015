type Password = [char; 8];

#[aoc_generator(day11)]
pub fn parse_input(input: &str) -> Password {
    input.chars().collect::<Vec<_>>().try_into().unwrap()
}

fn is_valid(password: &Password) -> bool {
    let contains_straight = {
        let mut result = false;
        for i in 2..password.len() {
            let run = [
                password[i - 2] as u8,
                password[i - 1] as u8,
                password[i] as u8,
            ];
            if run == [run[0], run[0] + 1, run[0] + 2] {
                result = true;
                break;
            }
        }
        result
    };
    let contains_iol = !password.iter().any(|&c| c == 'i' || c == 'o' || c == 'u');
    let contains_two_pairs = {
        let mut pairs = 0;
        let mut i = 1;
        while i < password.len() {
            if password[i - 1] == password[i] {
                pairs += 1;
                i += 2; // avoid matching overlapping pairs
            } else {
                i += 1;
            }
        }
        pairs >= 2
    };
    contains_straight && contains_iol && contains_two_pairs
}

fn next_password(mut password: Password) -> Password {
    let mut increment_index = password.len() - 1;
    while password[increment_index] == 'z' {
        increment_index -= 1;
    }
    password[increment_index] = ((password[increment_index] as u8) + 1) as char;
    for c in password.iter_mut().skip(increment_index + 1) {
        *c = 'a';
    }
    password
}

fn next_valid_password(mut password: Password) -> Password {
    password = next_password(password);
    while !is_valid(&password) {
        password = next_password(password)
    }
    password
}

#[aoc(day11, part1)]
pub fn part1(input: &Password) -> String {
    let password = next_valid_password(*input);
    String::from_iter(password)
}

#[aoc(day11, part2)]
pub fn part2(input: &Password) -> String {
    let password = next_valid_password(*input);
    let password = next_valid_password(password);
    String::from_iter(password)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_password() {
        assert_eq!(
            next_password(parse_input("abcdefgh")),
            parse_input("abcdefgi")
        );
        assert_eq!(
            next_password(parse_input("abcdefgz")),
            parse_input("abcdefha")
        );
        assert_eq!(
            next_password(parse_input("abcdzzzz")),
            parse_input("abceaaaa")
        );
    }

    #[test]
    fn test_is_valid() {
        assert!(!is_valid(&parse_input("hijklmmn")));
        assert!(!is_valid(&parse_input("abbceffg")));
        assert!(!is_valid(&parse_input("abbcegjk")));
        assert!(!is_valid(&parse_input("abbcegjk")));
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            next_valid_password(parse_input("abcdefgh")),
            parse_input("abcdffaa")
        );
        assert_eq!(
            next_valid_password(parse_input("ghijklmn")),
            parse_input("ghjaabcc")
        );
    }
}
