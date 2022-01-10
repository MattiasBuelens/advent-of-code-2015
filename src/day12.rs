use json::JsonValue;
use std::convert::TryInto;

#[aoc_generator(day12)]
pub fn parse_input(input: &str) -> JsonValue {
    json::parse(input).unwrap()
}

#[aoc(day12, part1)]
pub fn part1(json: &JsonValue) -> i32 {
    match json {
        JsonValue::Number(value) => (*value).try_into().ok().unwrap(),
        JsonValue::Array(values) => values.iter().map(|x| part1(x)).sum(),
        JsonValue::Object(object) => object.iter().map(|(_, value)| part1(value)).sum(),
        _ => 0,
    }
}

#[aoc(day12, part2)]
pub fn part2(json: &JsonValue) -> i32 {
    match json {
        JsonValue::Number(value) => (*value).try_into().ok().unwrap(),
        JsonValue::Array(values) => values.iter().map(|x| part2(x)).sum(),
        JsonValue::Object(object) => {
            let contains_red = object
                .iter()
                .any(|(_, value)| value.as_str() == Some("red"));
            if contains_red {
                0
            } else {
                object.iter().map(|(_, value)| part2(value)).sum()
            }
        }
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(r#"[1,2,3]"#)), 6);
        assert_eq!(part1(&parse_input(r#"{"a":2,"b":4}"#)), 6);
        assert_eq!(part1(&parse_input(r#"[[[3]]]"#)), 3);
        assert_eq!(part1(&parse_input(r#"{"a":{"b":4},"c":-1}"#)), 3);
        assert_eq!(part1(&parse_input(r#"{"a":[-1,1]}"#)), 0);
        assert_eq!(part1(&parse_input(r#"[-1,{"a":1}]"#)), 0);
        assert_eq!(part1(&parse_input(r#"[]"#)), 0);
        assert_eq!(part1(&parse_input(r#"{}"#)), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(r#"[1,2,3]"#)), 6);
        assert_eq!(part2(&parse_input(r#"[1,{"c":"red","b":2},3]"#)), 4);
        assert_eq!(part2(&parse_input(r#"{"d":"red","e":[1,2,3,4],"f":5}"#)), 0);
        assert_eq!(part2(&parse_input(r#"[1,"red",5]"#)), 6);
    }
}
