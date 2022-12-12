use crate::solver::Solver;
use std::collections::HashSet;
use std::str::FromStr;

pub struct FirstPart;
pub struct SecondPart;

impl Solver<u32> for FirstPart {
    const FILE_NAME: &'static str = "day4";

    fn solution(&self, input: String) -> u32 {
        input
            .split("\n")
            .map(|a| {
                let (first_range_str, second_range_str) = split_by_two(a, ",");
                let (first_range_start, first_range_finish) =
                    split_by_two_to_uint(first_range_str, "-");
                let (second_range_start, second_range_finish) =
                    split_by_two_to_uint(second_range_str, "-");
                if first_range_start >= second_range_start
                    && first_range_finish <= second_range_finish
                    || second_range_start >= first_range_start
                        && second_range_finish <= first_range_finish
                {
                    1
                } else {
                    0
                }
            })
            .sum()
    }
}

fn split_by_two<'a, 'b>(string: &'a str, separator: &'b str) -> (&'a str, &'a str) {
    let mut string = string.split(separator);
    let first = string.next().unwrap();
    let second = string.next().unwrap();
    (first, second)
}

fn split_by_two_to_uint(string: &str, separator: &str) -> (u32, u32) {
    let (first, second) = split_by_two(string, separator);
    (
        u32::from_str(first).unwrap(),
        u32::from_str(second).unwrap(),
    )
}
impl Solver<u32> for SecondPart {
    const FILE_NAME: &'static str = "day4";

    fn solution(&self, input: String) -> u32 {
        let mut insertions = 0;
        input.split("\n").for_each(|a| {
            let (first_range_str, second_range_str) = split_by_two(a, ",");
            let (first_range_start, first_range_finish) =
                split_by_two_to_uint(first_range_str, "-");
            let (second_range_start, second_range_finish) =
                split_by_two_to_uint(second_range_str, "-");
            let first_hash_set: HashSet<_> = (first_range_start..first_range_finish + 1).collect();
            let second_hash_set: HashSet<_> =
                (second_range_start..second_range_finish + 1).collect();
            if first_hash_set
                .intersection(&second_hash_set)
                .next()
                .is_some()
            {
                insertions += 1;
            }
        });
        insertions
    }
}
