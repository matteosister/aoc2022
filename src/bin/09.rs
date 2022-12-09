use std::collections::HashSet;
use std::iter::repeat;

pub fn part_one(input: &str) -> Option<u32> {
    let movements: Vec<Movement> = input.lines().map(Movement::from_input).collect();
    let mut canvas = Canvas::new(1);
    for movement in movements {
        for _ in 0..movement.steps {
            canvas.move_head(&movement.direction);
        }
    }
    Some(canvas.tail_positions.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let movements: Vec<Movement> = input.lines().map(Movement::from_input).collect();
    let mut canvas = Canvas::new(9);
    for movement in movements {
        for _ in 0..movement.steps {
            canvas.move_head(&movement.direction);
        }
    }
    Some(canvas.tail_positions.len() as u32)
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn start() -> Self {
        Self { x: 0, y: 0 }
    }
    fn as_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
    fn left(&mut self) {
        self.x -= 1;
    }
    fn right(&mut self) {
        self.x += 1;
    }
    fn up(&mut self) {
        self.y += 1;
    }
    fn down(&mut self) {
        self.y -= 1;
    }
}

#[derive(Debug)]
struct Canvas {
    tail_positions: HashSet<Position>,
    head: Position,
    tails: Vec<Position>,
}

impl Canvas {
    fn new(num_tails: u32) -> Self {
        let mut tail_positions = HashSet::new();
        tail_positions.insert(Position::start());
        let tails = repeat(Position::start()).take(num_tails as usize).collect();
        Self {
            tail_positions,
            head: Position::start(),
            tails,
        }
    }

    fn move_head(&mut self, direction: &MovementDirection) {
        match direction {
            MovementDirection::Left => self.head.left(),
            MovementDirection::Right => self.head.right(),
            MovementDirection::Up => self.head.up(),
            MovementDirection::Down => self.head.down(),
        }
        self.maybe_move_tails();
        let last_tail = self.tails.last().unwrap();

        self.tail_positions.insert(last_tail.clone());
    }

    fn maybe_move_tails(&mut self) {
        self.tails = self
            .tails
            .iter()
            .fold((vec![], self.head.clone()), |(mut acc, previous), tail| {
                let new_tail_position = Self::calculate_positions(previous, tail);
                acc.push(new_tail_position.clone());
                (acc, new_tail_position)
            })
            .0;
    }

    fn calculate_positions(new_head: Position, tail: &Position) -> Position {
        let (head_x, head_y) = new_head.as_tuple();
        let (tail_x, tail_y) = tail.as_tuple();
        let mut new_tail = tail.clone();

        if head_y - tail_y == 2 {
            // tail needs to move up
            new_tail.up();
            if head_x > tail_x {
                new_tail.right();
            }
            if head_x < tail_x {
                new_tail.left();
            }
            return new_tail;
        }

        if head_y - tail_y == -2 {
            // tail needs to move down
            new_tail.down();
            if head_x > tail_x {
                new_tail.right();
            }
            if head_x < tail_x {
                new_tail.left();
            }
            return new_tail;
        }

        if head_x - tail_x == 2 {
            // tail needs to move right
            new_tail.right();
            if head_y > tail_y {
                new_tail.up();
            }
            if head_y < tail_y {
                new_tail.down();
            }
            return new_tail;
        }

        if head_x - tail_x == -2 {
            // tail needs to move left
            new_tail.left();
            if head_y > tail_y {
                new_tail.up();
            }
            if head_y < tail_y {
                new_tail.down();
            }
            return new_tail;
        }

        new_tail
    }
}

#[derive(Debug)]
enum MovementDirection {
    Left,
    Right,
    Up,
    Down,
}

impl MovementDirection {
    fn from_input(input: char) -> Self {
        match input {
            'L' => Self::Left,
            'R' => Self::Right,
            'U' => Self::Up,
            'D' => Self::Down,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
struct Movement {
    direction: MovementDirection,
    steps: u32,
}

impl Movement {
    fn from_input(input: &str) -> Self {
        let (direction, steps) = input.split_once(" ").unwrap();
        let direction = MovementDirection::from_input(direction.trim().chars().nth(0).unwrap());
        let steps: u32 = steps.parse().unwrap();
        Self { direction, steps }
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}
