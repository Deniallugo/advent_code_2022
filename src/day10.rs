use crate::solver::Solver;
use std::str::FromStr;

pub struct FirstPart;
pub struct SecondPart;

#[derive(Debug, Copy, Clone)]
pub enum Command {
    NOOP,
    ADDX(i32),
}

impl From<&str> for Command {
    fn from(value: &str) -> Self {
        if value.contains("noop") {
            Command::NOOP
        } else if value.contains("addx") {
            let (_, a) = value.split_at(5);
            Command::ADDX(i32::from_str(a).unwrap())
        } else {
            unreachable!()
        }
    }
}

#[derive(Debug)]
struct CPU {
    x: i32,
    current_cycle: i32,
}

impl CPU {
    fn execute_command(&mut self, command: Command) {
        match command {
            Command::NOOP => self.current_cycle += 1,
            Command::ADDX(x) => {
                // dbg!((self.x, command, self.current_cycle));
                self.x += x;
                self.current_cycle += 1;
            }
        }
    }
}

impl Solver<i32> for FirstPart {
    const FILE_NAME: &'static str = "day10";

    fn solution(&self, input: String) -> i32 {
        let mut stack = vec![];
        input.split("\n").for_each(|a| {
            let command = Command::from(a);
            if let Command::ADDX(_) = &command {
                stack.push(Command::NOOP);
            }
            stack.push(command);
        });
        let mut cpu = CPU {
            x: 1,
            current_cycle: 1,
        };
        let mut next_cycle_to_measure = 20;
        let step = 40;
        let mut sum = 0;
        for command in stack.iter() {
            if cpu.current_cycle == next_cycle_to_measure {
                sum += cpu.current_cycle * cpu.x;
                next_cycle_to_measure += step;
            }
            cpu.execute_command(*command);
        }

        sum
    }
}

impl Solver<String> for SecondPart {
    const FILE_NAME: &'static str = "day10";

    fn solution(&self, input: String) -> String {
        let mut stack = vec![];
        input.split("\n").for_each(|a| {
            let command = Command::from(a);
            if let Command::ADDX(_) = &command {
                stack.push(Command::NOOP);
            }
            stack.push(command);
        });
        let mut cpu = CPU {
            x: 1,
            current_cycle: 1,
        };
        let mut next_row_start = 40;
        let step = 40;
        let mut result = "".to_string();
        for command in stack.iter() {
            let current_pixel = cpu.current_cycle - next_row_start + step - 1;
            if current_pixel == cpu.x || current_pixel == cpu.x - 1 || current_pixel == cpu.x + 1 {
                result.push('#');
            } else {
                result.push('.');
            }
            if cpu.current_cycle == next_row_start {
                result.push('\n');
                next_row_start += step;
            }
            cpu.execute_command(*command);
        }

        result
    }
}
