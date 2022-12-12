use priority_queue::PriorityQueue;
use std::cmp::min;
use std::collections::{HashMap, HashSet};

fn find_path(maze: &mut Maze, start: &mut Node, end: &mut Node) -> Vec<Node> {
    let mut queue = PriorityQueue::new();
    start.distance = 0;
    queue.push(start.clone(), 0);

    let mut discovered_nodes = HashSet::new();

    while !queue.is_empty() {
        let (current_node, _) = queue.pop().unwrap();
        discovered_nodes.insert((current_node.x, current_node.y));

        for neighbor in maze.get_unvisited_neighbor(&current_node, &discovered_nodes) {
            let neighbor_node = maze.get_mut(&(neighbor.x, neighbor.y)).unwrap();

            let min_distance = min(
                neighbor_node.distance,
                current_node
                    .distance
                    .checked_add(1)
                    .unwrap_or(current_node.root_distance),
            );
            if min_distance != neighbor_node.distance {
                neighbor_node.distance = min_distance;
                neighbor_node.parent = Some((current_node.x, current_node.y));
                if queue.get_mut(neighbor_node).is_some() {
                    queue.change_priority(&neighbor_node, min_distance);
                }
            }

            let priority = neighbor_node.distance;
            queue.push(*neighbor_node, priority);
        }
    }

    let end_node = maze.get(&(end.x, end.y)).unwrap();
    let mut path = vec![];
    let mut previous_node = end_node;
    loop {
        let parent_coords = previous_node.parent;
        match parent_coords {
            None => break,
            Some(parent_coords) => {
                let parent_node = maze.get(&parent_coords).unwrap();
                path.push(parent_node);
                previous_node = parent_node;
            }
        }
    }
    path.into_iter().cloned().rev().collect()
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
}

impl Maze {
    fn from_input(input: &str) -> (Self, Node, Node) {
        let (nodes, start, end) = generate_nodes(input);
        (Self { nodes }, start, end)
    }

    fn get_unvisited_neighbor(
        &self,
        node: &Node,
        visited_nodes: &HashSet<(usize, usize)>,
    ) -> Vec<Node> {
        let (x, y) = (node.x, node.y);
        let top_coords = (x, y - 1);
        let right_coords = (x + 1, y);
        let bottom_coords = (x, y + 1);
        let left_coords = (x - 1, y);
        let mut out = vec![];
        for coord in [top_coords, right_coords, bottom_coords, left_coords] {
            if visited_nodes.contains(&(coord.0, coord.1)) {
                continue;
            }

            match self.nodes.iter().find(|n| n.x == coord.0 && n.y == coord.1) {
                None => {}
                Some(new_node) => {
                    if new_node.elevation <= node.elevation + 1 {
                        out.push(new_node.clone());
                    }
                }
            }
        }
        out
    }

    fn get_mut(&mut self, (x, y): &(usize, usize)) -> Option<&mut Node> {
        self.nodes.iter_mut().find(|n| n.x == *x && n.y == *y)
    }

    fn get(&self, (x, y): &(usize, usize)) -> Option<&Node> {
        self.nodes.iter().find(|n| n.x == *x && n.y == *y)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Node {
    x: usize,
    y: usize,
    elevation: usize,
    distance: usize,
    root_distance: usize,
    parent: Option<(usize, usize)>,
}

impl Node {
    fn new(x: usize, y: usize, elevation: usize) -> Self {
        Self {
            x,
            y,
            elevation,
            distance: usize::MAX,
            root_distance: usize::MAX,
            parent: None,
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
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut maze, mut start, mut end) = Maze::from_input(input);
    let path = find_path(&mut maze, &mut start, &mut end);
    Some(path.len() as u32)
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
