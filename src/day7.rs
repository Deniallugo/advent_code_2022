use crate::solver::Solver;
use std::cmp::min;
use std::collections::HashMap;
use std::str::FromStr;

pub struct FirstPart;
pub struct SecondPart;

#[derive(Debug)]
enum Command {
    CdBack,
    Cd(String),
    CdPop,
    LS,
}

impl Command {
    fn parse_command(input: &str) -> Option<Command> {
        if input.contains("$") {
            if input.contains("$ cd /") {
                Some(Command::CdBack)
            } else if input.contains("$ cd ..") {
                Some(Command::CdPop)
            } else if input.contains("$ ls") {
                Some(Command::LS)
            } else {
                let (_, dir_name) = input.split_at(5);
                Some(Command::Cd(dir_name.to_string()))
            }
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
struct Dir {
    parent_name: Option<String>,
    size: usize,
    inside_dirs: Vec<String>,
}

#[derive(Debug, Default)]
struct FileSystem {
    current_dir: String,
    dirs: HashMap<String, Dir>,
}

impl FileSystem {
    fn execute_command<'a>(&mut self, command: Command, tail: Vec<&'a str>) -> Vec<&'a str> {
        let mut tail = tail;
        match command {
            Command::CdBack => {
                self.current_dir = "".to_string();
            }
            Command::Cd(dir_name) => {
                let path = format!("{}/{}", &self.current_dir, dir_name);
                if !self.dirs.contains_key(&path) {
                    let new_dir = Dir {
                        parent_name: Some(self.current_dir.clone()),
                        size: 0,
                        inside_dirs: vec![],
                    };
                    self.dirs.insert(path.clone(), new_dir.clone());
                    let current_dir = self.dirs.get_mut(&self.current_dir).unwrap();
                    current_dir.inside_dirs.push(path.clone());
                }
                self.current_dir = path;
            }
            Command::CdPop => {
                let current_dir = self.dirs.get(&self.current_dir).unwrap();
                self.current_dir = current_dir.parent_name.clone().unwrap();
            }
            Command::LS => {
                while let Some(next_string) = tail.pop() {
                    if next_string.contains("$") {
                        tail.push(next_string);
                        break;
                    } else if next_string.contains("dir") {
                        continue;
                    } else {
                        let mut data = next_string.split(" ");
                        let size = u32::from_str(data.next().unwrap()).unwrap();
                        let mut current_dir = self.current_dir.clone();
                        loop {
                            let dir = self.dirs.get_mut(&current_dir).unwrap();
                            dir.size += size as usize;
                            if let Some(parent_dir) = &dir.parent_name {
                                current_dir = parent_dir.clone();
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }
        tail
    }
    fn new() -> Self {
        let current_dir = "".to_string();
        let mut dirs = HashMap::default();
        dirs.insert(
            current_dir.clone(),
            Dir {
                parent_name: None,
                size: 0,
                inside_dirs: vec![],
            },
        );
        FileSystem { current_dir, dirs }
    }
    fn execute_all_commands(&mut self, mut tail: Vec<&str>) {
        tail.reverse();
        while tail.len() != 0 {
            let input = tail.pop().unwrap();
            let current_command = Command::parse_command(input).unwrap();
            tail = self.execute_command(current_command, tail);
        }
    }
}

impl Solver<usize> for FirstPart {
    const FILE_NAME: &'static str = "day7";

    fn solution(&self, input: String) -> usize {
        let mut file_system = FileSystem::new();
        let commands: Vec<_> = input.split("\n").collect();
        file_system.execute_all_commands(commands);
        let mut overall_size = 0;
        let max_size = 100000;
        for (_, dir) in file_system.dirs.iter() {
            if dir.size <= max_size {
                overall_size += dir.size;
            }
        }

        overall_size
    }
}

impl Solver<usize> for SecondPart {
    const FILE_NAME: &'static str = "day7";

    fn solution(&self, input: String) -> usize {
        let mut file_system = FileSystem::new();
        let commands: Vec<_> = input.split("\n").collect();
        file_system.execute_all_commands(commands);
        let total_disk_size = 70000000;
        let required_size = 30000000;
        let free_space = total_disk_size - file_system.dirs.get("").unwrap().size;
        let mut directory_size_for_remove = u32::MAX as usize;
        for (_, dir) in &file_system.dirs {
            if free_space + dir.size > required_size {
                directory_size_for_remove = min(directory_size_for_remove, dir.size);
            }
        }
        directory_size_for_remove
    }
}
