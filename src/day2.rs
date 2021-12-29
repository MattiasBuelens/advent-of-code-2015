use std::str::FromStr;

pub struct Present {
    l: u32,
    w: u32,
    h: u32,
}

impl FromStr for Present {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [l, w, h]: [u32; 3] = s
            .split('x')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<u32>>()
            .try_into()
            .unwrap();
        Ok(Present { l, w, h })
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Present> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

impl Present {
    fn area(&self) -> u32 {
        let mut sides = [self.l, self.w, self.h];
        sides.sort();
        let [l, w, h] = sides;
        3 * l * w + 2 * w * h + 2 * h * l
    }
}

#[aoc(day2, part1)]
pub fn part1(input: &[Present]) -> u32 {
    input.iter().map(|present| present.area()).sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &[Present]) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = input_generator("2x3x4");
        assert_eq!(part1(&input), 58);
        let input = input_generator("1x1x10");
        assert_eq!(part1(&input), 43);
    }
}
