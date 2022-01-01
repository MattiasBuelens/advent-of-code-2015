use std::iter::once;
use std::str::FromStr;

use lazy_static::*;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

lazy_static! {
    static ref INPUT_RE: Regex = Regex::new(
        r"^(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)$"
    )
    .unwrap();
}

impl FromStr for Ingredient {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = INPUT_RE.captures(s).unwrap();
        Ok(Ingredient {
            name: captures[1].to_string(),
            capacity: captures[2].parse().unwrap(),
            durability: captures[3].parse().unwrap(),
            flavor: captures[4].parse().unwrap(),
            texture: captures[5].parse().unwrap(),
            calories: captures[6].parse().unwrap(),
        })
    }
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<Ingredient> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

type Selection<'a> = Vec<(&'a Ingredient, u32)>;
type SelectionRef<'a> = &'a [(&'a Ingredient, u32)];

fn select_ingredients(
    ingredients: &[Ingredient],
    teaspoons: u32,
) -> Box<dyn Iterator<Item = Selection> + '_> {
    let (first, rest) = ingredients.split_first().unwrap();
    if rest.is_empty() {
        let selection = (first, teaspoons);
        Box::new(once(vec![selection]))
    } else {
        Box::new((0..=teaspoons).flat_map(move |amount| {
            select_ingredients(rest, teaspoons - amount).map(move |mut selection| {
                let first_selection = (first, amount);
                selection.insert(0, first_selection);
                selection
            })
        }))
    }
}

fn cookie_score(selection: Selection) -> i64 {
    let mut capacity = 0i64;
    let mut durability = 0i64;
    let mut flavor = 0i64;
    let mut texture = 0i64;
    for (ingredient, amount) in selection {
        let amount = amount as i64;
        capacity += ingredient.capacity as i64 * amount;
        durability += ingredient.durability as i64 * amount;
        flavor += ingredient.flavor as i64 * amount;
        texture += ingredient.texture as i64 * amount;
    }
    capacity = capacity.max(0);
    durability = durability.max(0);
    flavor = flavor.max(0);
    texture = texture.max(0);
    capacity * durability * flavor * texture
}

#[aoc(day15, part1)]
pub fn part1(input: &[Ingredient]) -> i64 {
    select_ingredients(input, 100)
        .map(cookie_score)
        .max()
        .unwrap()
}

fn calorie_count(selection: SelectionRef) -> i64 {
    selection
        .iter()
        .map(|(ingredient, amount)| ingredient.calories as i64 * *amount as i64)
        .sum()
}

#[aoc(day15, part2)]
pub fn part2(input: &[Ingredient]) -> i64 {
    select_ingredients(input, 100)
        .filter(|selection| calorie_count(selection) == 500)
        .map(cookie_score)
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use lazy_static::*;

    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"
            .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 62842880);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 57600000);
    }
}
