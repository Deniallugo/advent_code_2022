use crate::solver::Solver;

pub struct FirstPart;
pub struct SecondPart;

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Shape {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

impl Shape {
    fn shape_for_win(&self) -> Self {
        match self {
            Shape::Rock => Shape::Scissor,
            Shape::Paper => Shape::Rock,
            Shape::Scissor => Shape::Paper,
        }
    }

    fn shape_for_loose(&self) -> Self {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissor,
            Shape::Scissor => Shape::Rock,
        }
    }

    fn fight_result(&self, opponent_shape: Shape) -> GameResult {
        if *self == opponent_shape {
            GameResult::Draw
        } else if self.shape_for_win() == opponent_shape {
            GameResult::Loose
        } else {
            GameResult::Win
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum GameResult {
    Loose = 0,
    Draw = 3,
    Win = 6,
}

impl From<&str> for Shape {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissor,
            _ => unreachable!(),
        }
    }
}

impl From<&str> for GameResult {
    fn from(value: &str) -> Self {
        match value {
            "X" => GameResult::Loose,
            "Y" => GameResult::Draw,
            "Z" => GameResult::Win,
            _ => unreachable!(),
        }
    }
}

impl Solver<u32> for FirstPart {
    const FILE_NAME: &'static str = "day2";

    fn solution(&self, input: String) -> u32 {
        input
            .split("\n")
            .map(|res| {
                let game = res.split(" ").map(Shape::from).collect::<Vec<_>>();
                game_result(game[0], game[1])
            })
            .sum()
    }
}

impl Solver<u32> for SecondPart {
    const FILE_NAME: &'static str = "day2";

    fn solution(&self, input: String) -> u32 {
        input
            .split("\n")
            .map(|res| {
                let game = res.split(" ").collect::<Vec<_>>();
                game_result2(game[0].into(), game[1].into())
            })
            .sum()
    }
}

fn game_result(first: Shape, second: Shape) -> u32 {
    second.fight_result(first) as u32 + second as u32
}

fn game_result2(shape: Shape, result: GameResult) -> u32 {
    let shape = match result {
        GameResult::Loose => shape.shape_for_win(),
        GameResult::Draw => shape,
        GameResult::Win => shape.shape_for_loose(),
    };
    shape as u32 + result as u32
}
