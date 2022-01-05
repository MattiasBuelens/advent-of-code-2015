use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(isize),
    JumpIfEven(Register, isize),
    JumpIfOne(Register, isize),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Register {
    A,
    B,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instruction, s) = s.split_once(' ').unwrap();
        Ok(match instruction {
            "hlf" => Instruction::Half(s.parse().unwrap()),
            "tpl" => Instruction::Triple(s.parse().unwrap()),
            "inc" => Instruction::Increment(s.parse().unwrap()),
            "jmp" => Instruction::Jump(s.parse().unwrap()),
            "jie" => {
                let (register, offset) = s.split_once(", ").unwrap();
                Instruction::JumpIfEven(register.parse().unwrap(), offset.parse().unwrap())
            }
            "jio" => {
                let (register, offset) = s.split_once(", ").unwrap();
                Instruction::JumpIfOne(register.parse().unwrap(), offset.parse().unwrap())
            }
            _ => panic!("unknown instruction: {}", instruction),
        })
    }
}

impl FromStr for Register {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "a" => Register::A,
            "b" => Register::B,
            _ => panic!("unknown register: {}", s),
        })
    }
}

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

struct Computer {
    program: Vec<Instruction>,
    pc: usize,
    registers: [i32; 2],
}

impl Computer {
    fn new(program: Vec<Instruction>, registers: [i32; 2]) -> Self {
        Self {
            program,
            pc: 0,
            registers,
        }
    }

    fn run(&mut self) {
        while self.step() {}
    }

    fn step(&mut self) -> bool {
        match self.program[self.pc] {
            Instruction::Half(register) => {
                let register = self.register_mut(register);
                *register /= 2;
                self.pc += 1;
            }
            Instruction::Triple(register) => {
                let register = self.register_mut(register);
                *register *= 3;
                self.pc += 1;
            }
            Instruction::Increment(register) => {
                let register = self.register_mut(register);
                *register += 1;
                self.pc += 1;
            }
            Instruction::Jump(offset) => {
                self.jump(offset);
            }
            Instruction::JumpIfEven(register, offset) => {
                if *self.register(register) % 2 == 0 {
                    self.jump(offset);
                } else {
                    self.pc += 1;
                }
            }
            Instruction::JumpIfOne(register, offset) => {
                if *self.register(register) == 1 {
                    self.jump(offset);
                } else {
                    self.pc += 1;
                }
            }
        }
        self.pc < self.program.len()
    }

    fn register(&self, register: Register) -> &i32 {
        match register {
            Register::A => &self.registers[0],
            Register::B => &self.registers[1],
        }
    }

    fn register_mut(&mut self, register: Register) -> &mut i32 {
        match register {
            Register::A => &mut self.registers[0],
            Register::B => &mut self.registers[1],
        }
    }

    fn jump(&mut self, offset: isize) {
        self.pc = (self.pc as isize + offset) as usize;
    }
}

#[aoc(day23, part1)]
pub fn part1(input: &[Instruction]) -> i32 {
    let mut computer = Computer::new(input.to_vec(), [0, 0]);
    computer.run();
    *computer.register(Register::B)
}

#[aoc(day23, part2)]
pub fn part2(input: &[Instruction]) -> i32 {
    let mut computer = Computer::new(input.to_vec(), [1, 0]);
    computer.run();
    *computer.register(Register::B)
}

#[cfg(test)]
mod tests {
    use lazy_static::*;

    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
inc a
jio a, +2
tpl a
inc a"
            .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        let mut computer = Computer::new(input, [0; 2]);
        computer.run();
        assert_eq!(*computer.register(Register::A), 2);
    }
}
