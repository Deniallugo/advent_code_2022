use crate::solver::Solver;
use std::collections::hash_map::RandomState;
use std::collections::HashSet;

pub struct FirstPart;
pub struct SecondPart;

impl Solver<usize> for FirstPart {
    const FILE_NAME: &'static str = "day6";

    fn solution(&self, input: String) -> usize {
        solution(input, 4)
    }
}

impl Solver<usize> for SecondPart {
    const FILE_NAME: &'static str = "day6";

    fn solution(&self, input: String) -> usize {
        solution(input, 14)
    }
}

fn solution(input: String, step: usize) -> usize {
    let input = input.as_bytes();
    let mut final_character = step;
    while final_character < input.len() {
        let hashset: HashSet<_, RandomState> =
            HashSet::from_iter(input[final_character - step..final_character].iter());
        if hashset.len() == step {
            return final_character;
        }
        final_character += 1;
    }
    panic!("not found");
}
