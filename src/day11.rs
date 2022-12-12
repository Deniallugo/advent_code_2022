use crate::solver::Solver;
use std::collections::VecDeque;
use std::str::FromStr;

pub struct FirstPart;
pub struct SecondPart;

enum Operation {
    Add(u128),
    Mul(u128),
    Square,
}

impl Operation {
    fn apply(&self, old: u128) -> u128 {
        match self {
            Operation::Add(x) => old + x,
            Operation::Mul(x) => old * x,
            Operation::Square => old.pow(2),
        }
    }
}

struct Monkey {
    items: VecDeque<u128>,
    test_division: u128,
    positive_monkey: u32,
    negative_monkey: u32,
    operation: Operation,
    inspected_items: u128,
}

impl From<&str> for Monkey {
    fn from(value: &str) -> Self {
        let mut value = value.split("\n");
        value.next().unwrap();
        let starting_items: VecDeque<_> = value
            .next()
            .unwrap()
            .trim_start()
            .strip_prefix("Starting items: ")
            .unwrap()
            .split(", ")
            .map(|a| u128::from(u128::from_str(a).unwrap()))
            .collect();
        let operation = value
            .next()
            .unwrap()
            .trim_start()
            .strip_prefix("Operation: ")
            .unwrap();
        let operation = if operation.contains("old * old") {
            Operation::Square
        } else if operation.contains("*") {
            let val = operation.strip_prefix("new = old * ").unwrap();
            Operation::Mul(u128::from_str(val).unwrap())
        } else if operation.contains("+") {
            let val = operation.strip_prefix("new = old + ").unwrap();
            Operation::Add(u128::from_str(val).unwrap())
        } else {
            unreachable!()
        };
        let division_value = u128::from_str(
            value
                .next()
                .unwrap()
                .trim_start()
                .strip_prefix("Test: divisible by ")
                .unwrap(),
        )
        .unwrap();
        let positive_monkey = u32::from_str(
            value
                .next()
                .unwrap()
                .trim_start()
                .strip_prefix("If true: throw to monkey ")
                .unwrap(),
        )
        .unwrap();
        let negative_monkey = u32::from_str(
            value
                .next()
                .unwrap()
                .trim_start()
                .strip_prefix("If false: throw to monkey ")
                .unwrap(),
        )
        .unwrap();
        Monkey {
            items: starting_items,
            test_division: division_value,
            positive_monkey,
            negative_monkey,
            operation,
            inspected_items: 0,
        }
    }
}

#[derive(Debug)]
pub struct ItemMovement {
    item: u128,
    monkey_num: u32,
}

impl Monkey {
    fn inspect_next_item(&mut self, worry_level_division: Option<u128>) -> Option<ItemMovement> {
        if let Some(item) = self.items.pop_front() {
            self.inspected_items += 1;
            let new_level = self.operation.apply(item);

            let new_level = if let Some(divisor) = worry_level_division {
                new_level % divisor
            } else {
                self.operation.apply(item) / 3
            };
            if new_level.clone() % self.test_division == u128::from(0u32) {
                Some(ItemMovement {
                    item: new_level,
                    monkey_num: self.positive_monkey,
                })
            } else {
                Some(ItemMovement {
                    item: new_level,
                    monkey_num: self.negative_monkey,
                })
            }
        } else {
            None
        }
    }
}

impl Solver<u128> for FirstPart {
    const FILE_NAME: &'static str = "day11";

    fn solution(&self, input: String) -> u128 {
        let mut monkeys: Vec<_> = input.split("\n\n").map(Monkey::from).collect();

        for _ in 0..20 {
            for i in 0..monkeys.len() {
                let monkey = monkeys.get_mut(i).unwrap();
                let mut movements = vec![];
                while let Some(movement) = monkey.inspect_next_item(None) {
                    movements.push(movement);
                }
                for movement in &movements {
                    let new_owner = monkeys.get_mut(movement.monkey_num as usize).unwrap();
                    new_owner.items.push_back(movement.item.clone())
                }
            }
        }
        monkeys.sort_by_key(|m| m.inspected_items);
        monkeys.reverse();
        monkeys[0].inspected_items * monkeys[1].inspected_items
    }
}

impl Solver<u128> for SecondPart {
    const FILE_NAME: &'static str = "day11";

    fn solution(&self, input: String) -> u128 {
        let mut monkeys: Vec<_> = input.split("\n\n").map(Monkey::from).collect();

        let divisor = monkeys
            .iter()
            .map(|m| m.test_division)
            .reduce(|acc, x| acc * x);

        for round in 0..10000 {
            println!("{}", round);
            for i in 0..monkeys.len() {
                let monkey = monkeys.get_mut(i).unwrap();
                let mut movements = vec![];

                while let Some(movement) = monkey.inspect_next_item(divisor) {
                    movements.push(movement);
                }
                for movement in &movements {
                    let new_owner = monkeys.get_mut(movement.monkey_num as usize).unwrap();
                    new_owner.items.push_back(movement.item.clone())
                }
            }
        }

        for (num, monkey) in monkeys.iter().enumerate() {
            println!("Monkey {} inspected {}", num, monkey.inspected_items);
        }

        monkeys.sort_by_key(|m| m.inspected_items);
        monkeys.reverse();
        monkeys[0].inspected_items * monkeys[1].inspected_items
    }
}
