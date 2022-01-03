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
    fn step(&self, part2: bool) -> Self {
        let mut next = self.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = &mut next.cells[y][x];
                if part2 && self.is_stuck(x, y) {
                    assert!(*cell);
                    continue;
                }
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

    fn is_stuck(&self, x: usize, y: usize) -> bool {
        [0, self.width - 1].contains(&x) && [0, self.height - 1].contains(&y)
    }

    fn make_stuck(&mut self) {
        for y in [0, self.width - 1] {
            for x in [0, self.height - 1] {
                self.cells[y][x] = true;
            }
        }
    }
}

fn simulate(mut grid: Grid, steps: usize, part2: bool) -> Grid {
    for _ in 0..steps {
        grid = grid.step(part2);
    }
    grid
}

#[aoc(day18, part1)]
pub fn part1(input: &Grid) -> usize {
    simulate(input.clone(), 100, false).count_on()
}

#[aoc(day18, part2)]
pub fn part2(input: &Grid) -> usize {
    let mut grid = input.clone();
    grid.make_stuck();
    simulate(grid, 100, true).count_on()
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
        let grid = input_generator(&TEST_INPUT);
        assert_eq!(simulate(grid, 4, false).count_on(), 4);
    }

    #[test]
    fn test_part2() {
        let mut grid = input_generator(&TEST_INPUT);
        grid.make_stuck();
        assert_eq!(simulate(grid, 5, true).count_on(), 17);
    }
}
