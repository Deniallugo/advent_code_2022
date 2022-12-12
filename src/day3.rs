use crate::solver::Solver;
use std::collections::hash_map::RandomState;
use std::collections::HashSet;

pub struct FirstPart;
pub struct SecondPart;

impl Solver<u32> for FirstPart {
    const FILE_NAME: &'static str = "day3";

    fn solution(&self, input: String) -> u32 {
        input
            .split("\n")
            .map(|value| {
                let (first_part, second_part) =
                    value.trim_start().trim_end().split_at(value.len() / 2);
                let first_part: HashSet<_, RandomState> =
                    HashSet::from_iter(first_part.split("").filter(|a| *a != ""));
                let second_part: HashSet<_, RandomState> =
                    HashSet::from_iter(second_part.split("").filter(|a| *a != ""));
                string_to_number(first_part.intersection(&second_part).next().unwrap())
            })
            .sum()
    }
}

fn string_to_number(value: &str) -> u32 {
    let char = value.as_bytes()[0] as u32;
    let a = "a".as_bytes()[0] as u32;
    #[allow(clippy::non_snake_case)]
    let A = "A".as_bytes()[0] as u32;
    let z = "z".as_bytes()[0] as u32;
    #[allow(clippy::non_snake_case)]
    let Z = "Z".as_bytes()[0] as u32;
    return if char >= a && char <= z {
        char - a + 1
    } else if char >= A && char <= Z {
        char - A + 27
    } else {
        unreachable!();
    };
}

impl Solver<u32> for SecondPart {
    const FILE_NAME: &'static str = "day3";

    fn solution(&self, input: String) -> u32 {
        input
            .split("\n")
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|chunk| {
                let first_part: HashSet<_, RandomState> =
                    HashSet::from_iter(chunk[0].split("").filter(|a| *a != ""));
                let second_part: HashSet<_, RandomState> =
                    HashSet::from_iter(chunk[1].split("").filter(|a| *a != ""));
                let third_part: HashSet<_, RandomState> =
                    HashSet::from_iter(chunk[2].split("").filter(|a| *a != ""));
                let first_intersection: HashSet<_> =
                    first_part.intersection(&second_part).map(|a| *a).collect();
                string_to_number(first_intersection.intersection(&third_part).next().unwrap())
            })
            .sum()
    }
}
