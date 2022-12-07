use std::collections::HashMap;
use std::ops::Add;

pub fn part_one(input: &str) -> Option<u32> {
    let output: Vec<TerminalOutput> = input.lines().map(TerminalOutput::from_input).collect();
    let fs = Fs::from_terminal_outputs(output);

    let sum_size: u32 =
        fs.0.keys()
            .map(|folder| fs.traverse(folder, |files| files.iter().map(|f| f.1).sum::<u32>()))
            .filter(|size| size <= &100_000)
            .sum();

    Some(sum_size)
}

pub fn part_two(input: &str) -> Option<u32> {
    let output: Vec<TerminalOutput> = input.lines().map(TerminalOutput::from_input).collect();
    let fs = Fs::from_terminal_outputs(output);

    const TOTAL: u32 = 70000000;
    const NEEDED: u32 = 30000000;

    let fs_size: u32 = fs.traverse("/", |files| files.iter().map(|f| f.1).sum::<u32>());
    let unused = TOTAL - fs_size;
    let to_be_freed = NEEDED - unused;

    let mut possible_folders: Vec<u32> =
        fs.0.keys()
            .map(|folder| fs.traverse(folder, |files| files.iter().map(|f| f.1).sum::<u32>()))
            .filter(|size| size >= &to_be_freed)
            .collect();
    possible_folders.sort_unstable();
    Some(possible_folders[0])
}

//////////////////////
// Tree definitions //
//////////////////////
#[derive(Debug, Clone)]
struct TreeFile(String, u32);

#[derive(Debug, Clone)]
struct TreeFolder {
    files: Vec<TreeFile>,
}

impl TreeFolder {
    fn new() -> Self {
        TreeFolder { files: vec![] }
    }

    fn add_file(&mut self, name: String, size: u32) {
        self.files.push(TreeFile(name, size));
    }
}

#[derive(Debug)]
struct Fs(HashMap<String, TreeFolder>);

#[derive(Debug)]
struct Path(Vec<String>);

impl Path {
    fn root() -> Self {
        Self(vec![])
    }

    fn add(&mut self, folder_name: &str) {
        self.0.push(folder_name.to_string());
    }

    fn parent(&mut self) {
        self.0.pop();
    }

    fn as_string(&self) -> String {
        self.0
            .iter()
            .fold(String::new(), |acc, path_piece| acc.add(path_piece))
    }
}

impl Fs {
    fn from_terminal_outputs(terminal_outputs: Vec<TerminalOutput>) -> Self {
        let mut fs = HashMap::new();
        let mut actual_path = Path::root();
        let mut actual_folder = TreeFolder::new();
        for terminal_output in &terminal_outputs[1..] {
            match terminal_output {
                TerminalOutput::Command(command) => match command {
                    TerminalCommand::ChDir(ch_dir_command) => {
                        fs.insert(actual_path.as_string(), actual_folder.clone());

                        match ch_dir_command {
                            ChDirCommand::ChDir(name) => {
                                actual_path.add(name);
                            }
                            ChDirCommand::ChDirBack => {
                                actual_path.parent();
                            }
                            ChDirCommand::ChDirTop => actual_path = Path::root(),
                        }
                        actual_folder = fs
                            .get(&actual_path.as_string())
                            .cloned()
                            .unwrap_or(TreeFolder::new());
                    }
                    TerminalCommand::Ls => {}
                },
                TerminalOutput::File(name, size) => actual_folder.add_file(name.to_string(), *size),
                TerminalOutput::Dir(_) => {}
            }
        }
        fs.insert(actual_path.as_string(), actual_folder.clone());

        Self(fs)
    }
    fn traverse<V>(&self, start_name: &str, func: impl Fn(Vec<&TreeFile>) -> V) -> V {
        let files = self
            .0
            .iter()
            .filter_map(|(key, tree_folder)| {
                if key.starts_with(start_name) {
                    Some(&tree_folder.files)
                } else {
                    None
                }
            })
            .flatten()
            .into_iter()
            .collect();
        func(files)
    }
}

////////////////////////////////
// Terminal parse definitions //
////////////////////////////////
#[derive(Debug)]
enum TerminalOutput {
    Command(TerminalCommand),
    File(String, u32),
    Dir(String),
}

impl TerminalOutput {
    fn from_input(line: &str) -> Self {
        if line.starts_with("$") {
            // this is a command
            TerminalOutput::Command(TerminalCommand::from_input(&line[2..]))
        } else {
            // this is a file size
            if line.starts_with("dir") {
                TerminalOutput::Dir(line[4..].to_string())
            } else {
                let (size, name) = line.split_at(line.find(" ").unwrap());
                TerminalOutput::File(name[1..].to_string(), size.parse().unwrap())
            }
        }
    }
}

#[derive(Debug)]
enum TerminalCommand {
    ChDir(ChDirCommand),
    Ls,
}

impl TerminalCommand {
    pub fn from_input(command: &str) -> Self {
        if command == "ls" {
            Self::Ls
        } else {
            if command == "cd .." {
                Self::ChDir(ChDirCommand::ChDirBack)
            } else if command == "cd /" {
                Self::ChDir(ChDirCommand::ChDirTop)
            } else {
                Self::ChDir(ChDirCommand::ChDir(command[3..].to_string()))
            }
        }
    }
}

#[derive(Debug)]
enum ChDirCommand {
    ChDir(String),
    ChDirBack,
    ChDirTop,
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
