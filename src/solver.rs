use std::fmt::Debug;
use std::fs;
use std::path::Path;

pub trait Solver<F: Debug> {
    const FILE_NAME: &'static str;
    fn input(&self) -> String {
        let path = format!("./{}", Self::FILE_NAME);
        let path = Path::new(&path);
        let result = fs::read(path).unwrap();
        String::from_utf8(result).unwrap()
    }

    fn solution(&self, input: String) -> F;

    fn result(&self) -> F {
        self.solution(self.input())
    }

    fn print_result(&self) {
        println!("{:?}", self.result());
    }
}
