use std::ops::RangeInclusive;

pub fn part_one(input: &str) -> Option<i32> {
    let commands: CommandList = input.lines().map(Command::from_input).collect();
    let mut curr = 20;
    let iter = std::iter::repeat_with(|| {
        let tmp = curr;
        curr += 40;
        tmp
    });

    Some(
        iter.take_while(|v| v < &commands.len())
            .map(|index| commands.get(index as u32) * index as i32)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let commands: CommandList = input.lines().map(Command::from_input).collect();

    let mut output = String::new();
    for y in 0..=5 {
        for x in 0..=39 {
            let iteration = (x + 1) + (y * 40) as u32;
            let sprite_pos = commands.get(iteration);
            let range_check = RangeInclusive::new(
                if sprite_pos == 0 { 0 } else { sprite_pos - 1 },
                sprite_pos + 1,
            );
            if range_check.contains(&(x as i32)) {
                output += "#";
            } else {
                output += ".";
            }
        }
        output += "\n";
    }
    Some(output.trim().to_string())
}

#[derive(Debug)]
struct CommandList(Vec<Command>);

impl FromIterator<Command> for CommandList {
    fn from_iter<T: IntoIterator<Item = Command>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl CommandList {
    fn get(&self, num: u32) -> i32 {
        let mut out = 1;
        let mut iteration = 1;
        for command in &self.0 {
            match command {
                Command::NoOp => {
                    iteration += 1;
                }
                Command::AddX(val) => {
                    if iteration + 2 > num {
                        break;
                    }
                    out += val;
                    iteration += 2;
                }
            }

            if iteration as u32 >= num {
                break;
            }
        }
        out as i32
    }

    fn len(&self) -> usize {
        self.0
            .iter()
            .map(|c| match c {
                Command::NoOp => 1,
                Command::AddX(_) => 2,
            })
            .sum()
    }
}

#[derive(Debug)]
enum Command {
    NoOp,
    AddX(i32),
}

impl Command {
    fn from_input(input: &str) -> Self {
        match input {
            "noop" => Self::NoOp,
            add if add.starts_with("addx ") => Self::AddX(add[5..].parse().unwrap()),
            _ => unimplemented!(),
        }
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(
            part_two(&input),
            Some(String::from(
                "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
            ))
        );
    }
}
