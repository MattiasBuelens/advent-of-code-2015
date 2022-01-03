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

fn calibrate(replacements: &[(String, String)], molecule: &str) -> usize {
    let results = replacements
        .iter()
        .flat_map(|(input, output)| {
            molecule.match_indices(input).map(move |(i, m)| {
                let mut result = molecule.to_string();
                result.replace_range(i..(i + m.len()), output);
                result
            })
        })
        .collect::<HashSet<_>>();
    results.len()
}

#[aoc(day19, part1)]
pub fn part1((replacements, molecule): &Input) -> usize {
    calibrate(replacements, molecule)
}

fn build_molecule(replacements: &[(String, String)], molecule: &str) -> Option<usize> {
    if molecule == "e" {
        return Some(0);
    }
    // Try all possible replacements in all possible positions.
    // Work backwards: replace the output with the input.
    let mut reduced_molecules = replacements
        .iter()
        .flat_map(|(input, output)| {
            molecule.match_indices(output).map(|(i, m)| {
                let mut input_molecule = molecule.to_string();
                input_molecule.replace_range(i..(i + m.len()), input);
                input_molecule
            })
        })
        .collect::<Vec<_>>();
    // Try the *shortest* resulting molecule first.
    reduced_molecules.sort_unstable_by_key(|x| x.len());
    // Find the first molecule that can be reduced completely.
    // Theoretically speaking, there may be a longer molecule that can be reduced in fewer steps.
    // However, the replacements in the input are such that shorter molecules can *always* be
    // reduced in fewer steps than longer molecules.
    reduced_molecules
        .into_iter()
        .filter_map(|reduced| build_molecule(replacements, &reduced))
        .next()
        // Add one for the extra replacement.
        .map(|x| x + 1)
}

#[aoc(day19, part2)]
pub fn part2((replacements, molecule): &Input) -> usize {
    build_molecule(replacements, molecule).unwrap()
}

#[cfg(test)]
mod tests {
    use lazy_static::*;

    use super::*;

    lazy_static! {
        static ref REPLACEMENTS_1: &'static str = r"
H => HO
H => OH
O => HH"
            .trim();
        static ref REPLACEMENTS_2: &'static str = r"
e => H
e => O
H => HO
H => OH
O => HH"
            .trim();
    }

    #[test]
    fn test_part1() {
        let replacements = parse_replacements(&REPLACEMENTS_1);
        assert_eq!(calibrate(&replacements, "HOH"), 4);
        assert_eq!(calibrate(&replacements, "HOHOHO"), 7);
    }

    #[test]
    fn test_part2() {
        let replacements = parse_replacements(&REPLACEMENTS_2);
        assert_eq!(build_molecule(&replacements, "HOH"), Some(3));
        assert_eq!(build_molecule(&replacements, "HOHOHO"), Some(6));
    }
}
