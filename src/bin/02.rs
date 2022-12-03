pub fn part_one(input: &str) -> Option<u32> {
    Some(
        create_matches(input, Match::from_match_str_day1)
            .iter()
            .map(|m| m.points())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        create_matches(input, Match::from_match_str_day2)
            .iter()
            .map(|m| m.points())
            .sum(),
    )
}

#[derive(Debug, PartialEq, Clone)]
enum Choice {
    Rock,
    Paper,
    Scissor,
}

impl Choice {
    fn from_char(c: char) -> Self {
        match c {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissor,
            _ => unimplemented!(),
        }
    }

    fn loosing(&self) -> Self {
        match self {
            Choice::Rock => Choice::Scissor,
            Choice::Paper => Choice::Rock,
            Choice::Scissor => Choice::Paper,
        }
    }

    fn winning(&self) -> Self {
        match self {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissor,
            Choice::Scissor => Choice::Rock,
        }
    }

    fn value(&self) -> u32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissor => 3,
        }
    }
}

#[derive(Debug)]
struct Match {
    p1: Choice,
    p2: Choice,
}

impl Match {
    fn from_match_str_day1(match_str: &str) -> Self {
        Self {
            p1: Choice::from_char(match_str.chars().nth(0).unwrap()),
            p2: Choice::from_char(match_str.chars().nth(2).unwrap()),
        }
    }

    pub fn from_match_str_day2(match_str: &str) -> Self {
        let p1 = Choice::from_char(match_str.chars().nth(0).unwrap());
        match match_str.chars().nth(2).unwrap() {
            'X' => Match::loose(p1),
            'Y' => Match::draw(p1),
            'Z' => Match::win(p1),
            _ => unimplemented!(),
        }
    }

    fn loose(opponent: Choice) -> Self {
        let p2 = opponent.loosing();
        Self { p1: opponent, p2 }
    }

    fn win(opponent: Choice) -> Self {
        let p2 = opponent.winning();
        Self { p1: opponent, p2 }
    }

    fn draw(opponent: Choice) -> Self {
        Self {
            p1: opponent.clone(),
            p2: opponent,
        }
    }

    fn points(&self) -> u32 {
        match (&self.p1, &self.p2) {
            (Choice::Scissor, Choice::Rock) => 6 + Choice::value(&Choice::Rock),
            (Choice::Rock, Choice::Paper) => 6 + Choice::value(&Choice::Paper),
            (Choice::Paper, Choice::Scissor) => 6 + Choice::value(&Choice::Scissor),
            (a, b) if a == b => 3 + b.value(),
            (_, b) => b.value(),
        }
    }
}

fn create_matches(input: &str, match_creator: impl Fn(&str) -> Match) -> Vec<Match> {
    input.lines().map(match_creator).collect()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
