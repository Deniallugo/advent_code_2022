use crate::solver::Solver;
use std::str::FromStr;

pub struct FirstPart;
pub struct SecondPart;

#[derive(Debug, Clone, Copy)]
struct Movement {
    from: usize,
    to: usize,
    quantity: u32,
}

impl From<&str> for Movement {
    fn from(value: &str) -> Self {
        let (_, value) = value.split_at(5);
        let mut split = value.split(" from ");
        let quantity = u32::from_str(split.next().unwrap()).unwrap();
        let value = split.next().unwrap();
        let mut split = value.split(" to ");
        let from = u32::from_str(split.next().unwrap()).unwrap() as usize;
        let to = u32::from_str(split.next().unwrap()).unwrap() as usize;
        Movement { from, to, quantity }
    }
}

#[derive(Debug)]
pub struct Stack(pub(crate) Vec<Vec<char>>);

impl From<&str> for Stack {
    fn from(value: &str) -> Self {
        let mut values: Vec<_> = value.split("\n").map(|a| a.as_bytes()).collect();
        values.reverse();
        let first_value = 1;
        let next_values = 4;
        let mut stack = vec![];
        let mut values = values.iter();
        values.next();
        for value in values {
            let mut current_position = first_value;
            let mut stack_number = 0;
            while current_position < value.len() {
                let current_char = value[current_position] as char;
                current_position += next_values;
                stack_number += 1;
                if stack.len() < stack_number {
                    stack.push(vec![]);
                }
                if current_char != ' ' {
                    stack[stack_number - 1].push(current_char);
                }
            }
        }
        Self(stack)
    }
}

impl Stack {
    fn apply_movement_separately(&mut self, movement: &Movement) {
        for _ in 0..movement.quantity {
            let value = self.0.get_mut(movement.from - 1).unwrap().pop().unwrap();
            self.0.get_mut(movement.to - 1).unwrap().push(value);
        }
    }

    fn apply_movement_together(&mut self, movement: &Movement) {
        let mut results = vec![];
        let from = self.0.get_mut(movement.from - 1).unwrap();
        for _ in 0..movement.quantity {
            results.push(from.pop().unwrap());
        }
        results.reverse();

        let to = self.0.get_mut(movement.to - 1).unwrap();
        for result in results {
            to.push(result);
        }
    }

    fn get_top_values(&self) -> Vec<char> {
        self.0
            .iter()
            .map(|stack| *stack.last().unwrap_or(&' '))
            .collect()
    }
}

impl Solver<String> for FirstPart {
    const FILE_NAME: &'static str = "day5";

    fn solution(&self, input: String) -> String {
        let mut data = input.split("\n\n");
        let mut stack = Stack::from(data.next().unwrap());
        let movements: Vec<_> = data
            .next()
            .unwrap()
            .split("\n")
            .map(Movement::from)
            .collect();
        for movement in movements {
            stack.apply_movement_separately(&movement);
        }
        stack.get_top_values().iter().collect()
    }
}

impl Solver<String> for SecondPart {
    const FILE_NAME: &'static str = "day5";

    fn solution(&self, input: String) -> String {
        let mut data = input.split("\n\n");
        let mut stack = Stack::from(data.next().unwrap());
        let movements: Vec<_> = data
            .next()
            .unwrap()
            .split("\n")
            .map(Movement::from)
            .collect();
        for movement in movements {
            stack.apply_movement_together(&movement);
        }
        stack.get_top_values().iter().collect()
    }
}
