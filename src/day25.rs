type Input = (u32, u32);

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> Input {
    let s = input
        .strip_prefix(
            "To continue, please consult the code grid in the manual.  Enter the code at row ",
        )
        .unwrap();
    let (row, s) = s.split_once(", column ").unwrap();
    let column = s.strip_suffix('.').unwrap();
    (row.parse().unwrap(), column.parse().unwrap())
}

fn code_position(row: u32, column: u32) -> u32 {
    // Codes on same diagonal have same sum of row and column numbers
    let diagonal = row + column;
    if diagonal == 0 {
        return column;
    }
    // Total number of codes on previous diagonals
    // = 1 + 2 + 3 + .. + diagonal
    let codes_on_previous_diagonals = diagonal * (diagonal + 1) / 2;
    codes_on_previous_diagonals + column
}

#[aoc(day25, part1)]
pub fn part1(&(row, column): &Input) -> u64 {
    let position = code_position(row - 1, column - 1);
    let mut code = 20151125;
    for _ in 0..position {
        code = (code * 252533) % 33554393;
    }
    code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_position() {
        assert_eq!(code_position(0, 0), 0);
        assert_eq!(code_position(1, 0), 1);
        assert_eq!(code_position(0, 1), 2);
        assert_eq!(code_position(2, 0), 3);
        assert_eq!(code_position(0, 2), 5);
        assert_eq!(code_position(5, 0), 15);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&(1, 1)), 20151125);
        assert_eq!(part1(&(6, 1)), 33071741);
        assert_eq!(part1(&(6, 6)), 27995004);
    }
}
