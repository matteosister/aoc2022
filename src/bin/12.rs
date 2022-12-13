use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Debug)]
struct PathFinder {
    map: Vec<Vec<u8>>,
    distances: Vec<Vec<u16>>,
    start: (usize, usize),
    end: (usize, usize),
    width: usize,
    height: usize,
}

impl PathFinder {
    fn dijkstra(&mut self) {
        let mut search_queue = VecDeque::with_capacity(self.width * self.height);
        search_queue.push_back(self.end);

        while !search_queue.is_empty() {
            let current = search_queue.pop_front().unwrap();
            let new_dist = self.distances[current.1][current.0] + 1;

            // Check which neighbours can reach the current position.
            // If their distance needs updating, update their distance and add them for future checking
            for (dx, dy) in &[(0, -1), (-1, 0), (1, 0), (0, 1)] {
                let x = current.0 as isize + dx;
                let y = current.1 as isize + dy;

                if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
                    // Neighbour exists...
                    if self.map[current.1][current.0] <= self.map[y as usize][x as usize] + 1 {
                        // ...and can reach the current position...
                        if self.distances[y as usize][x as usize] > new_dist {
                            // ...and this is a shorter path
                            self.distances[y as usize][x as usize] = new_dist;
                            search_queue.push_back((x as usize, y as usize));
                        }
                    }
                }
            }
        }
    }

    fn from_input(input: &str) -> Self {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let map = input
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.char_indices()
                    .map(|(x, c)| match c {
                        'S' => {
                            start = (x, y);
                            b'a'
                        }
                        'E' => {
                            end = (x, y);
                            b'z'
                        }
                        n if n.is_lowercase() => n as u8,
                        _ => panic!("invalid char {c}"),
                    })
                    .collect_vec()
            })
            .collect_vec();
        let width = map[0].len();
        let height = map.len();
        let mut distances = map
            .iter()
            .map(|r| r.iter().map(|_| u16::MAX).collect_vec())
            .collect_vec();
        distances[end.1][end.0] = 0;
        PathFinder {
            map,
            distances,
            start,
            end,
            width,
            height,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut path_finder = PathFinder::from_input(input);
    path_finder.dijkstra();
    Some(path_finder.distances[path_finder.start.1][path_finder.start.0] as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut path_finder = PathFinder::from_input(input);
    path_finder.dijkstra();
    let steps = path_finder
        .map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(
                move |(x, h)| {
                    if *h == b'a' {
                        Some((x, y))
                    } else {
                        None
                    }
                },
            )
        })
        .map(|(x, y)| path_finder.distances[y][x])
        .min()
        .unwrap();

    Some(steps as u32)
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
        assert_eq!(part_two(&input), Some(29));
    }
}
