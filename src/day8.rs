use crate::solver::Solver;
use std::cmp::max;
use std::str::FromStr;

pub struct FirstPart;
pub struct SecondPart;

impl Solver<usize> for FirstPart {
    const FILE_NAME: &'static str = "day8";

    fn solution(&self, input: String) -> usize {
        let matrix: Vec<Vec<u32>> = input
            .split("\n")
            .map(|a| {
                a.split("")
                    .filter(|a| *a != "")
                    .map(|a| u32::from_str(a).unwrap())
                    .collect()
            })
            .collect();
        let mut total_number = 0;
        for i in 1..matrix.len() - 1 {
            let row_len = matrix.get(i).unwrap().len();
            for j in 1..row_len - 1 {
                let value = matrix[i][j];
                let mut top = true;
                let mut bottom = true;
                let mut left = true;
                let mut right = true;
                for i1 in (0..i).rev() {
                    if matrix[i1][j] >= value && top {
                        top = false;
                    }
                }
                for i1 in i + 1..matrix.len() {
                    if matrix[i1][j] >= value && bottom {
                        bottom = false;
                    }
                }

                for j1 in (0..j).rev() {
                    if matrix[i][j1] >= value && left {
                        left = false;
                    }
                }
                for j1 in j + 1..matrix.len() {
                    if matrix[i][j1] >= value && right {
                        right = false;
                    }
                }
                if top || bottom || left || right {
                    total_number += 1;
                }
            }
        }
        total_number += (matrix.len() - 1) * 4;
        total_number
    }
}

impl Solver<usize> for SecondPart {
    const FILE_NAME: &'static str = "day8";

    fn solution(&self, input: String) -> usize {
        let matrix: Vec<Vec<u32>> = input
            .split("\n")
            .map(|a| {
                a.split("")
                    .filter(|a| *a != "")
                    .map(|a| u32::from_str(a).unwrap())
                    .collect()
            })
            .collect();
        let mut max_score = 0;
        for i in 1..matrix.len() - 1 {
            let row_len = matrix.get(i).unwrap().len();
            for j in 1..row_len - 1 {
                let value = matrix[i][j];
                let mut top_count = 0;
                let mut bottom_count = 0;
                let mut left_count = 0;
                let mut right_count = 0;
                for i1 in (0..i).rev() {
                    top_count += 1;
                    if matrix[i1][j] >= value {
                        break;
                    }
                }
                for i1 in i + 1..matrix.len() {
                    bottom_count += 1;
                    if matrix[i1][j] >= value {
                        break;
                    }
                }

                for j1 in (0..j).rev() {
                    left_count += 1;
                    if matrix[i][j1] >= value {
                        break;
                    }
                }
                for j1 in j + 1..matrix.len() {
                    right_count += 1;
                    if matrix[i][j1] >= value {
                        break;
                    }
                }
                let total_score = right_count * left_count * top_count * bottom_count;
                max_score = max(max_score, total_score);
            }
        }
        max_score
    }
}
