use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Connection(Gate, String);

#[derive(Debug, Clone)]
pub enum Gate {
    Forward(Input),
    And(Input, Input),
    Or(Input, Input),
    Not(Input),
    LeftShift(Input, u8),
    RightShift(Input, u8),
}

#[derive(Debug, Clone)]
pub enum Input {
    Value(u16),
    Wire(String),
}

impl FromStr for Connection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (input, output) = s.split_once(" -> ").unwrap();
        Ok(Connection(input.parse().unwrap(), output.to_string()))
    }
}

impl FromStr for Gate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<_>>();
        Ok(match parts[..] {
            [input] => Gate::Forward(input.parse().unwrap()),
            [input1, "AND", input2] => Gate::And(input1.parse().unwrap(), input2.parse().unwrap()),
            [input1, "OR", input2] => Gate::Or(input1.parse().unwrap(), input2.parse().unwrap()),
            ["NOT", input] => Gate::Not(input.parse().unwrap()),
            [input, "LSHIFT", shift] => {
                Gate::LeftShift(input.parse().unwrap(), shift.parse().unwrap())
            }
            [input, "RSHIFT", shift] => {
                Gate::RightShift(input.parse().unwrap(), shift.parse().unwrap())
            }
            _ => panic!("unexpected gate: {}", s),
        })
    }
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if let Ok(value) = s.parse::<u16>() {
            Input::Value(value)
        } else {
            Input::Wire(s.to_string())
        })
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<Connection> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

struct Circuit {
    connections: Vec<Connection>,
    cache: HashMap<String, u16>,
}

impl Circuit {
    fn new(connections: Vec<Connection>) -> Self {
        Self {
            connections,
            cache: HashMap::default(),
        }
    }

    fn compute_wire(&mut self, wire: String) -> u16 {
        if let Some(&value) = self.cache.get(&wire) {
            return value;
        }
        let Connection(gate, _) = self
            .connections
            .iter()
            .find(|connection| connection.1 == wire)
            .unwrap()
            .clone();
        let value = match gate {
            Gate::Forward(input) => self.compute_input(input),
            Gate::And(input1, input2) => self.compute_input(input1) & self.compute_input(input2),
            Gate::Or(input1, input2) => self.compute_input(input1) | self.compute_input(input2),
            Gate::Not(input) => !self.compute_input(input),
            Gate::LeftShift(input, shift) => self.compute_input(input) << shift,
            Gate::RightShift(input, shift) => self.compute_input(input) >> shift,
        };
        self.cache.insert(wire, value);
        value
    }

    fn compute_input(&mut self, input: Input) -> u16 {
        match input {
            Input::Value(value) => value,
            Input::Wire(wire) => self.compute_wire(wire),
        }
    }
}

#[aoc(day7, part1)]
pub fn part1(input: &[Connection]) -> u16 {
    let mut circuit = Circuit::new(input.to_vec());
    circuit.compute_wire("a".to_string())
}

#[aoc(day7, part2)]
pub fn part2(input: &[Connection]) -> u16 {
    let a_value = part1(input);

    let mut connections = input.to_vec();
    // Remove the original connection to wire "b", and replace it with a new connection
    // that puts the previously computed value of "a" directly onto wire "b".
    connections.remove(
        connections
            .iter()
            .position(|connection| connection.1 == "b")
            .unwrap(),
    );
    connections.push(Connection(
        Gate::Forward(Input::Value(a_value)),
        "b".to_string(),
    ));

    // Compute with the new connections
    let mut circuit = Circuit::new(connections);
    circuit.compute_wire("a".to_string())
}

#[cfg(test)]
mod tests {
    use lazy_static::*;

    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i"
            .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        let mut circuit = Circuit::new(input);
        assert_eq!(circuit.compute_wire("d".to_string()), 72);
        assert_eq!(circuit.compute_wire("e".to_string()), 507);
        assert_eq!(circuit.compute_wire("f".to_string()), 492);
        assert_eq!(circuit.compute_wire("g".to_string()), 114);
        assert_eq!(circuit.compute_wire("h".to_string()), 65412);
        assert_eq!(circuit.compute_wire("i".to_string()), 65079);
        assert_eq!(circuit.compute_wire("x".to_string()), 123);
        assert_eq!(circuit.compute_wire("y".to_string()), 456);
    }
}
