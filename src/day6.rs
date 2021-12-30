use std::str::FromStr;

use crate::util::Vector2D;

#[derive(Debug, Copy, Clone)]
pub struct Instruction(Command, Rectangle);

#[derive(Debug, Copy, Clone)]
pub enum Command {
    On,
    Off,
    Toggle,
}

#[derive(Debug, Copy, Clone)]
pub struct Rectangle(Vector2D, Vector2D);

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if let Some(s) = s.strip_prefix("turn on ") {
            Instruction(Command::On, s.parse()?)
        } else if let Some(s) = s.strip_prefix("turn off ") {
            Instruction(Command::Off, s.parse()?)
        } else if let Some(s) = s.strip_prefix("toggle ") {
            Instruction(Command::Toggle, s.parse()?)
        } else {
            panic!("unknown instruction: {}", s);
        })
    }
}

impl FromStr for Rectangle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (top_left, bottom_right) = s.split_once(" through ").unwrap();
        let (x1, y1) = top_left.split_once(',').unwrap();
        let (x2, y2) = bottom_right.split_once(',').unwrap();
        Ok(Rectangle(
            Vector2D::new(x1.parse().unwrap(), y1.parse().unwrap()),
            Vector2D::new(x2.parse().unwrap(), y2.parse().unwrap()),
        ))
    }
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day6, part1)]
pub fn part1(input: &[Instruction]) -> usize {
    let mut grid = vec![[false; 1000]; 1000];
    for &Instruction(command, Rectangle(tl, br)) in input {
        for y in tl.y()..=br.y() {
            for x in tl.x()..=br.x() {
                let light = &mut grid[y as usize][x as usize];
                *light = match command {
                    Command::On => true,
                    Command::Off => false,
                    Command::Toggle => !(*light),
                };
            }
        }
    }
    grid.into_iter()
        .map(|row| row.into_iter().filter(|&x| x).count())
        .sum()
}

#[aoc(day6, part2)]
pub fn part2(input: &[Instruction]) -> u32 {
    let mut grid = vec![[0u32; 1000]; 1000];
    for &Instruction(command, Rectangle(tl, br)) in input {
        for y in tl.y()..=br.y() {
            for x in tl.x()..=br.x() {
                let light = &mut grid[y as usize][x as usize];
                *light = match command {
                    Command::On => *light + 1,
                    Command::Off => light.saturating_sub(1),
                    Command::Toggle => *light + 2,
                };
            }
        }
    }
    grid.into_iter()
        .map(|row| row.into_iter().sum::<u32>())
        .sum()
}
