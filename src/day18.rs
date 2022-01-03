use crate::util::Vector2D;

#[derive(Debug, Clone)]
pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Vec<bool>>,
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Grid {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let cells = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();
    Grid {
        width,
        height,
        cells,
    }
}

impl Grid {
    fn step(&self) -> Self {
        let mut next = self.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = &mut next.cells[y][x];
                let neighbours = Vector2D::new(x as i32, y as i32)
                    .neighbours_diagonal()
                    .filter(|pos| {
                        (0..self.width as i32).contains(&pos.x())
                            && (0..self.height as i32).contains(&pos.y())
                            && self.cells[pos.y() as usize][pos.x() as usize]
                    })
                    .count();
                *cell = match (*cell, neighbours) {
                    // A light which is on stays on when 2 or 3 neighbors are on,
                    // and turns off otherwise.
                    (true, 2 | 3) => true,
                    (true, _) => false,
                    // A light which is off turns on if exactly 3 neighbors are on,
                    // and stays off otherwise.
                    (false, 3) => true,
                    (false, _) => false,
                };
            }
        }
        next
    }

    fn count_on(&self) -> usize {
        self.cells
            .iter()
            .map(|row| row.iter().filter(|cell| **cell).count())
            .sum()
    }
}

fn simulate(mut grid: Grid, steps: usize) -> Grid {
    for _ in 0..steps {
        grid = grid.step();
    }
    grid
}

#[aoc(day18, part1)]
pub fn part1(input: &Grid) -> usize {
    simulate(input.clone(), 100).count_on()
}

#[aoc(day18, part2)]
pub fn part2(input: &Grid) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use lazy_static::*;

    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
.#.#.#
...##.
#....#
..#...
#.#..#
####.."
            .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(simulate(input, 4).count_on(), 4);
    }
}
