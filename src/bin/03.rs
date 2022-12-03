use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| line.split_at(line.len() / 2))
            .flat_map(|(content1, content2)| find_same_content(content1, content2))
            .map(char_to_value)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let groups = input.lines().collect::<Vec<&str>>();
    let sum = groups
        .chunks(3)
        .map(|group| {
            let mut found_char = None;
            for char in group[0].chars() {
                if group[1].contains(char) && group[2].contains(char) {
                    found_char = Some(char);
                }
            }
            found_char.unwrap()
        })
        .map(char_to_value)
        .sum();

    Some(sum)
}

fn find_same_content(content1: &str, content2: &str) -> Vec<char> {
    content1
        .chars()
        .fold(HashSet::new(), |mut matches, char1| {
            if content2.contains(char1) {
                matches.insert(char1);
            }
            matches
        })
        .into_iter()
        .collect()
}

fn char_to_value(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 96
    } else {
        c as u32 - 38
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_to_value_works() {
        assert_eq!(char_to_value('a'), 1);
        assert_eq!(char_to_value('p'), 16);
        assert_eq!(char_to_value('P'), 42);
        assert_eq!(char_to_value('Z'), 52);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
