use crate::solver::Solver;
use std::collections::HashSet;
use std::str::FromStr;

pub struct FirstPart;
pub struct SecondPart;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rope {
    head: Point,
    tail: Point,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => unreachable!(),
        }
    }
}

impl Rope {
    fn move_head(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.head.x -= 1,
            Direction::Right => self.head.x += 1,
            Direction::Up => self.head.y += 1,
            Direction::Down => self.head.y -= 1,
        }
        self.move_tail();
        dbg!(&self);
    }

    fn move_tail(&mut self) {
        let diag = (self.tail.x - self.head.x).abs() > 1 && self.tail.y != self.head.y
            || (self.tail.y - self.head.y).abs() > 1 && self.tail.x != self.head.x;
        self.move_tail_x(1);
        let is_x = !self.move_tail_y(1);
        if diag {
            if is_x {
                self.move_tail_y(0);
            } else {
                self.move_tail_x(0);
            }
        }
    }

    fn move_tail_x(&mut self, step: i32) -> bool {
        let mut is_move = false;
        if (self.tail.x - self.head.x) > step {
            self.tail.x -= 1;
            is_move = true;
        }
        if (self.tail.x - self.head.x) < -step {
            self.tail.x += 1;
            is_move = true;
        }
        return is_move;
    }

    fn move_tail_y(&mut self, step: i32) -> bool {
        let mut is_move = false;
        if (self.tail.y - self.head.y) > step {
            self.tail.y -= 1;
            is_move = true;
        }
        if (self.tail.y - self.head.y) < -step {
            self.tail.y += 1;
            is_move = true;
        }
        is_move
    }
}

impl Solver<usize> for FirstPart {
    const FILE_NAME: &'static str = "day9";

    fn solution(&self, input: String) -> usize {
        let mut rope = Rope {
            tail: Point { x: 0, y: 0 },
            head: Point { x: 0, y: 0 },
        };
        let mut positions = HashSet::new();

        input.split("\n").for_each(|a| {
            let mut res = a.split(" ");
            let direction: Direction = res.next().unwrap().into();
            let step = u32::from_str(res.next().unwrap()).unwrap();
            dbg!((direction, step));
            for _ in 0..step {
                rope.move_head(direction);
                positions.insert(rope.tail);
            }
        });
        positions.len()
    }
}
