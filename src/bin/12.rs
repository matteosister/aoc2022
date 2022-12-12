use priority_queue::PriorityQueue;
use std::collections::{HashMap, HashSet};
use std::ops::Index;

fn find_path(maze: &mut Maze, start: &mut Node, end: &mut Node) {
    let mut queue = PriorityQueue::new();
    start.root_distance = 0;
    queue.push(start, 0);
    let mut discovered_nodes = HashSet::new();

    while !queue.is_empty() {
        let (current_node, _) = queue.pop().unwrap();
        discovered_nodes.insert((current_node.x, current_node.y));
        for neighbor in
            maze.get_unvisited_neighbor((current_node.x, current_node.y), discovered_nodes)
        {
        }
        break;
    }
}

fn generate_nodes(input: &str) -> (Vec<Node>, Node, Node) {
    let chars: HashMap<char, usize> = ('a'..='z')
        .into_iter()
        .enumerate()
        .map(|(size, char)| (char, size + 1))
        .collect();
    let mut start = None;
    let mut end = None;

    (
        input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, char)| {
                        match char {
                            'S' => start = Some(Node::new(x + 1, y + 1, 1)),
                            'E' => end = Some(Node::new(x + 1, y + 1, 26)),
                            _ => {}
                        };
                        Node::create(x + 1, y + 1, char, &chars)
                    })
                    .collect::<Vec<Node>>()
            })
            .collect(),
        start.expect("unable to find start node"),
        end.expect("unable to find end node"),
    )
}

#[derive(Debug)]
struct Maze {
    nodes: Vec<Node>,
    rows: usize,
    cols: usize,
}

impl Maze {
    fn from_input(input: &str) -> (Self, Node, Node) {
        let rows = input.lines().count();
        let cols = input.lines().nth(0).unwrap().chars().count();
        let (nodes, start, end) = generate_nodes(input);
        (Self { nodes, rows, cols }, start, end)
    }

    fn get_unvisited_neighbor(
        &self,
        (x, y): (usize, usize),
        visited_nodes: HashSet<(usize, usize)>,
    ) -> Vec<Node> {
        let top_coords = (x, y - 1);
        let right_coords = (x + 1, y);
        let bottom_coords = (x, y + 1);
        let left_coords = (x - 1, y);
        let mut out = vec![];
        for coord in [top_coords, right_coords, bottom_coords, left_coords] {
            if visited_nodes.contains(&(coord.0, coord.1)) {
                continue;
            }
            dbg!(coord);
            match self.nodes.iter().find(|n| n.x == coord.0 && n.y == coord.1) {
                None => {}
                Some(node) => {
                    out.push(node.clone());
                }
            }
        }
        out
    }

    fn get_mut(&mut self, (x, y): &(usize, usize)) -> Option<&mut Node> {
        self.nodes.iter_mut().find(|n| n.x == *x && n.y == *y)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Node {
    x: usize,
    y: usize,
    elevation: usize,
    distance: usize,
    root_distance: usize,
    manhattan_distance: usize,
}

impl Node {
    fn new(x: usize, y: usize, elevation: usize) -> Self {
        Self {
            x,
            y,
            elevation,
            distance: usize::MAX,
            root_distance: usize::MAX,
            manhattan_distance: usize::MAX,
        }
    }

    fn create(x: usize, y: usize, elevation: char, elevation_map: &HashMap<char, usize>) -> Self {
        let elevation = match elevation {
            'S' => 1,
            'E' => 26,
            char => elevation_map[&char],
        };

        Self::new(x, y, elevation)
    }

    fn set_manhattan_distance(&mut self, end: &Node) {
        self.manhattan_distance = (end.x.abs_diff(self.x) + end.y.abs_diff(self.y)) * 2
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut maze, mut start, mut end) = Maze::from_input(input);
    for mut node in maze.nodes.iter_mut() {
        node.set_manhattan_distance(&end);
    }
    find_path(&mut maze, &mut start, &mut end);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}
