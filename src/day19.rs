use std::collections::HashSet;

pub type Replacements = Vec<(String, String)>;
pub type Input = (Replacements, String);

fn parse_replacements(input: &str) -> Replacements {
    input
        .lines()
        .map(|line| {
            let (input, output) = line.split_once(" => ").unwrap();
            (input.to_string(), output.to_string())
        })
        .collect()
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Input {
    let (replacements, molecule) = input.split_once("\n\n").unwrap();
    let replacements = parse_replacements(replacements);
    let molecule = molecule.to_string();
    (replacements, molecule)
}

#[aoc(day19, part1)]
pub fn part1((replacements, molecule): &Input) -> usize {
    let results = replacements
        .iter()
        .flat_map(|(input, output)| {
            molecule.match_indices(input).map(move |(i, m)| {
                let mut result = molecule.to_string();
                result.replace_range(i..(i + m.len()), &output);
                result
            })
        })
        .collect::<HashSet<_>>();
    results.len()
}

#[aoc(day19, part2)]
pub fn part2((replacements, molecule): &Input) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use lazy_static::*;

    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
H => HO
H => OH
O => HH"
            .trim();
    }

    #[test]
    fn test_part1() {
        let replacements = parse_replacements(&TEST_INPUT);
        assert_eq!(part1(&(replacements.clone(), "HOH".to_string())), 4);
        assert_eq!(part1(&(replacements, "HOHOHO".to_string())), 7);
    }
}
