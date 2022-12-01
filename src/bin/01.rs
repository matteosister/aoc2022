pub fn part_one(input: &str) -> Option<u32> {
    split_in_chunks(input).max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut elfs: Vec<u32> = split_in_chunks(input).collect();
    elfs.sort();
    elfs.reverse();
    Some(elfs[0..=2].iter().sum())
}

fn split_in_chunks<'a>(input: &'a str) -> impl Iterator<Item = u32> + 'a {
    input.split("\n\n").map(|elf_contents| {
        elf_contents
            .trim()
            .split("\n")
            .map(|cal| cal.parse::<u32>().unwrap())
            .sum::<u32>()
    })
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
