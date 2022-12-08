use std::cmp;

/// Executors

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        Wood::from_input(input)
            .into_iter()
            .filter(|tree| tree.visible_from_outside())
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Wood::from_input(input)
        .into_iter()
        .map(|tree| tree.scenic_score())
        .max()
}

/// Types / Solution

enum ColumnDirection {
    Up,
    Down,
}

enum RowDirection {
    Left,
    Right,
}

#[derive(Debug)]
struct Wood {
    position: (u32, u32),
    row_size: usize,
    col_size: usize,
    rows: Vec<Row>,
}

impl Wood {
    fn from_input(input: &str) -> Self {
        let rows: Vec<Row> = input.lines().map(Row::from_input).collect();
        Self {
            position: (0, 0),
            row_size: rows[0].trees.len(),
            col_size: rows.len(),
            rows,
        }
    }

    fn get_tree(&self, x: usize, y: usize) -> Tree {
        let row = &self.rows[y - 1];
        let size = row.get_tree(x);
        let mut top = self.get_column(x, y, ColumnDirection::Up);
        top.reverse();
        let mut left = self.get_row(x, y, RowDirection::Left);
        left.reverse();

        Tree {
            size,
            top,
            right: self.get_row(x, y, RowDirection::Right),
            bottom: self.get_column(x, y, ColumnDirection::Down),
            left,
        }
    }

    fn get_column(&self, x: usize, y: usize, direction: ColumnDirection) -> Vec<u32> {
        match direction {
            ColumnDirection::Up => self.rows[0..(y - 1)]
                .iter()
                .map(|r| r.get_tree(x))
                .collect(),
            ColumnDirection::Down => self.rows[y..].iter().map(|r| r.get_tree(x)).collect(),
        }
    }

    fn get_row(&self, x: usize, y: usize, direction: RowDirection) -> Vec<u32> {
        match direction {
            RowDirection::Right => self.rows[y - 1].trees[x..].to_vec(),
            RowDirection::Left => self.rows[y - 1].trees[0..(x - 1)].to_vec(),
        }
    }
}

impl Iterator for Wood {
    type Item = Tree;

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.position;
        if x as usize == self.row_size - 1 {
            self.position = (0, y + 1);
        } else {
            self.position = (x + 1, y);
        }
        if x as usize <= self.row_size - 1 && y as usize <= self.col_size - 1 {
            Some(self.get_tree(x as usize + 1, y as usize + 1))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Row {
    trees: Vec<u32>,
}

impl Row {
    fn from_input(row: &str) -> Self {
        Self {
            trees: row.chars().map(|c| c.to_digit(10).unwrap()).collect(),
        }
    }

    fn get_tree(&self, x: usize) -> u32 {
        self.trees[x - 1]
    }
}

#[derive(Debug)]
struct Tree {
    size: u32,
    top: Vec<u32>,
    right: Vec<u32>,
    bottom: Vec<u32>,
    left: Vec<u32>,
}

fn maximum(values: &Vec<u32>) -> u32 {
    values.iter().fold(0, |maximum, a| cmp::max(maximum, *a))
}

impl Tree {
    fn visible_from_outside(&self) -> bool {
        if self.right.is_empty()
            || self.left.is_empty()
            || self.top.is_empty()
            || self.bottom.is_empty()
        {
            return true;
        }
        self.size > maximum(&self.top)
            || self.size > maximum(&self.right)
            || self.size > maximum(&self.bottom)
            || self.size > maximum(&self.left)
    }

    fn scenic_score(&self) -> u32 {
        scenic_score(self.size, &self.top)
            * scenic_score(self.size, &self.right)
            * scenic_score(self.size, &self.bottom)
            * scenic_score(self.size, &self.left)
    }
}

fn scenic_score(reference: u32, tree_row: &Vec<u32>) -> u32 {
    let mut final_score = 0;
    for tree in tree_row {
        final_score += 1;
        if tree >= &reference {
            break;
        }
    }
    final_score
}

/// Main

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

/// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
