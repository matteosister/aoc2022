use std::ops::RangeInclusive;
use std::str::FromStr;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        create_assigned_section_list(input)
            .filter(|(section1, section2)| section1.overlap(&section2))
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        create_assigned_section_list(input)
            .filter(|(section1, section2)| section1.partial_overlap(&section2))
            .count() as u32,
    )
}

fn create_assigned_section_list<'a>(
    input: &'a str,
) -> impl Iterator<Item = (AssignedSection, AssignedSection)> + 'a {
    input
        .lines()
        .map(|line| line.split_at(line.chars().position(|c| c == ',').unwrap()))
        .map(|(elf1, elf2)| {
            (
                AssignedSection::from_str(elf1).expect("impossible to parse section"),
                AssignedSection::from_str(&elf2[1..]).expect("impossible to parse section"),
            )
        })
}

#[derive(Debug)]
struct AssignedSection(RangeInclusive<u32>);

impl AssignedSection {
    fn new(start: u32, end: u32) -> Self {
        Self(RangeInclusive::new(start, end))
    }

    fn overlap(&self, other: &Self) -> bool {
        (self.0.start() <= other.0.start() && self.0.end() >= other.0.end())
            || (other.0.start() <= self.0.start() && other.0.end() >= self.0.end())
    }

    fn partial_overlap(&self, other: &Self) -> bool {
        self.0.start() <= other.0.end() && self.0.end() >= other.0.start()
    }
}

impl FromStr for AssignedSection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_at(s.chars().position(|c| c == '-').unwrap());
        let start = start.parse::<u32>().unwrap();
        let end = end[1..].parse::<u32>().unwrap();
        Ok(Self(RangeInclusive::new(start, end)))
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlap_test() {
        assert!(AssignedSection::overlap(
            &AssignedSection::new(2, 4),
            &AssignedSection::new(1, 5)
        ));

        assert!(AssignedSection::overlap(
            &AssignedSection::new(1, 20),
            &AssignedSection::new(2, 5)
        ));

        assert_eq!(
            false,
            AssignedSection::overlap(&AssignedSection::new(1, 20), &AssignedSection::new(15, 22))
        );
    }

    #[test]
    fn partial_overlap_test() {
        assert!(AssignedSection::partial_overlap(
            &AssignedSection::new(2, 4),
            &AssignedSection::new(3, 5)
        ));

        assert!(AssignedSection::partial_overlap(
            &AssignedSection::new(10, 20),
            &AssignedSection::new(2, 10)
        ));

        assert_eq!(
            false,
            AssignedSection::partial_overlap(
                &AssignedSection::new(1, 5),
                &AssignedSection::new(6, 22)
            )
        );
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
