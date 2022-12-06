use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    find_pattern(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    find_pattern(input, 14)
}

fn find_pattern(input: &str, size: usize) -> Option<u32> {
    let mut code = vec![];
    let mut final_index = 0;

    for (index, char) in input.chars().enumerate() {
        code.push(char);
        if code.len() == size {
            if is_composed_by_unique_elements(&code) {
                final_index = index + 1;
                break;
            }
            code = code[1..].to_vec();
        }
    }

    Some(final_index as u32)
}

fn is_composed_by_unique_elements(code: &Vec<char>) -> bool {
    let unique_chars: HashSet<&char> = code.iter().collect();
    unique_chars.len() == code.len()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        let lines: Vec<&str> = input.lines().collect();
        assert_eq!(part_one(&lines[0]), Some(7));
        assert_eq!(part_one(&lines[1]), Some(5));
        assert_eq!(part_one(&lines[2]), Some(6));
        assert_eq!(part_one(&lines[3]), Some(10));
        assert_eq!(part_one(&lines[4]), Some(11));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        let lines: Vec<&str> = input.lines().collect();
        assert_eq!(part_two(&lines[0]), Some(19));
        assert_eq!(part_two(&lines[1]), Some(23));
        assert_eq!(part_two(&lines[2]), Some(23));
        assert_eq!(part_two(&lines[3]), Some(29));
        assert_eq!(part_two(&lines[4]), Some(26));
    }
}
