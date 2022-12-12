use crate::solver::Solver;
use std::str::FromStr;

pub struct FirstPart;
pub struct SecondPart;

impl Solver<u32> for FirstPart {
    const FILE_NAME: &'static str = "day1";

    fn solution(&self, input: String) -> u32 {
        input
            .split("\n\n")
            .map(|res| res.split("\n").map(|v| u32::from_str(v).unwrap()).sum())
            .max()
            .unwrap()
    }
}

impl Solver<u32> for SecondPart {
    const FILE_NAME: &'static str = "day1";

    fn solution(&self, input: String) -> u32 {
        let mut solution: Vec<u32> = input
            .split("\n\n")
            .map(|res| res.split("\n").map(|v| u32::from_str(v).unwrap()).sum())
            .collect();
        solution.sort();
        solution.reverse();
        solution.truncate(3);
        solution.iter().sum()
    }
}
