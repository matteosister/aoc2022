use regex::Regex;
use std::collections::{BTreeMap, VecDeque};

pub fn part_one(input: &str) -> Option<String> {
    let (mut stacks, movements) = create_stacks_and_movements(input);

    for movement in movements {
        for _ in 1..=movement.size {
            let from = stacks.get_mut(&movement.from).unwrap();
            let element = from.remove();
            std::mem::drop(from);
            let to = stacks.get_mut(&movement.to).unwrap();
            to.add(element);
        }
    }

    Some(
        stacks
            .iter()
            .map(|(_, stack)| stack.0.front().unwrap())
            .collect(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut stacks, movements) = create_stacks_and_movements(input);

    for movement in movements {
        let from = stacks.get_mut(&movement.from).unwrap();
        let elements = from.remove_n_crates(movement.size);
        std::mem::drop(from);
        let to = stacks.get_mut(&movement.to).unwrap();
        to.add_n_crates(elements);
    }

    Some(
        stacks
            .iter()
            .map(|(_, stack)| stack.0.front().unwrap())
            .collect(),
    )
}

#[derive(Debug)]
struct Stack(VecDeque<char>);

impl Stack {
    fn remove(&mut self) -> char {
        self.0.pop_front().unwrap()
    }

    fn add(&mut self, c: char) {
        self.0.push_front(c);
    }

    fn remove_n_crates(&mut self, n: u32) -> Vec<char> {
        let mut out = vec![];
        for _ in 0..n {
            out.push(self.0.pop_front().unwrap());
        }
        out.reverse();
        out
    }

    fn add_n_crates(&mut self, chars: Vec<char>) {
        for char in chars {
            self.0.push_front(char);
        }
    }
}

#[derive(Debug)]
struct Movement {
    size: u32,
    from: char,
    to: char,
}

impl Movement {
    fn from_input(s: &str) -> Self {
        let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        let caps = re.captures(s).unwrap();
        Self {
            size: caps.get(1).unwrap().as_str().parse::<u32>().unwrap(),
            from: caps.get(2).unwrap().as_str().parse::<char>().unwrap(),
            to: caps.get(3).unwrap().as_str().parse::<char>().unwrap(),
        }
    }
}

fn create_stacks_and_movements(input: &str) -> (BTreeMap<char, Stack>, Vec<Movement>) {
    let (stacks, movements) = input.split_once("\n\n").unwrap();
    let stacks = create_stacks(stacks);
    let movements = create_movements(movements);

    (stacks, movements)
}

#[derive(Debug, Clone, Copy)]
enum StackContent {
    Content(char),
    Empty,
}

impl StackContent {
    fn get_content_unchecked(&self) -> char {
        match self {
            StackContent::Content(c) => *c,
            StackContent::Empty => unimplemented!(),
        }
    }

    fn get_content(&self) -> Option<char> {
        match self {
            StackContent::Content(c) => Some(*c),
            StackContent::Empty => None,
        }
    }
}

fn create_stacks(input: &str) -> BTreeMap<char, Stack> {
    let mut stacks: VecDeque<Vec<StackContent>> = input
        .lines()
        .map(|line| {
            line.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|a| {
                    a.into_iter()
                        .collect::<String>()
                        .trim()
                        .trim_matches('[')
                        .trim_matches(']')
                        .chars()
                        .next()
                        .map(StackContent::Content)
                        .unwrap_or(StackContent::Empty)
                })
                .collect::<Vec<StackContent>>()
        })
        .collect();

    let stack_names: Vec<char> = stacks
        .pop_back()
        .unwrap()
        .iter()
        .map(|c| c.get_content_unchecked())
        .collect();

    stack_names.iter().fold(BTreeMap::new(), |mut acc, name| {
        let index = name.to_digit(10).unwrap() - 1;
        let stack_content: VecDeque<char> = stacks
            .iter()
            .map(|stack| stack[index as usize])
            .filter_map(|c| c.get_content())
            .collect();
        acc.insert(*name, Stack(stack_content));
        acc
    })
}

fn create_movements(input: &str) -> Vec<Movement> {
    input.lines().map(Movement::from_input).collect()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
