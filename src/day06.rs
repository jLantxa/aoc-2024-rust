use std::collections::HashSet;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

const INPUT_FILE: &str = "input/day06.txt";

const CELL_OBSTACLE: char = '#';
const CELL_GUARD_UP: char = '^';
const CELL_GUARD_DOWN: char = 'v';
const CELL_GUARD_LEFT: char = '<';
const CELL_GUARD_RIGHT: char = '>';

const UP: Direction = (0, -1);
const RIGHT: Direction = (1, 0);
const DOWN: Direction = (0, 1);
const LEFT: Direction = (-1, 0);

type Position = (i32, i32);
type Direction = (i32, i32);

fn turn_right(dir: &Direction) -> Direction {
    (-dir.1, dir.0)
}

#[derive(Debug, Clone)]
struct Map {
    block: Vec<Vec<char>>,
}

impl Map {
    fn from_string(text: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut block = Vec::with_capacity(140);

        for line in text.lines() {
            let mut char_line: Vec<char> = Vec::with_capacity(140);
            for ch in line.chars() {
                char_line.push(ch);
            }

            block.push(char_line);
        }

        Ok(Self { block })
    }

    fn from_file(file_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Self::from_string(&std::fs::read_to_string(file_name)?)
    }

    fn at(&self, pos: Position) -> char {
        self.block[pos.1 as usize][pos.0 as usize]
    }

    fn dimensions(&self) -> (i32, i32) {
        (self.block[0].len() as i32, self.block.len() as i32)
    }

    fn is_inside(&self, position: Position) -> bool {
        let dimensions = self.dimensions();
        (position.0 >= 0 && position.0 < dimensions.0)
            && (position.1 >= 0 && position.1 < dimensions.1)
    }

    // Returns a HashSet with all the positions visited by the guard.
    fn get_visited_positions(&self) -> HashSet<Position> {
        let mut visited_positions = HashSet::new();
        let (mut pos, mut dir) = self.find_guard().expect("Expected a guard");

        loop {
            visited_positions.insert((pos.0, pos.1));

            let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
            if !self.is_inside(next_pos) {
                break;
            } else if self.at(next_pos) == CELL_OBSTACLE {
                dir = turn_right(&dir);
            } else {
                pos.0 += dir.0;
                pos.1 += dir.1;
            }
        }

        visited_positions
    }

    // Find the guard and its direction in the map.
    fn find_guard(&self) -> Option<(Position, Direction)> {
        for (j, row) in self.block.iter().enumerate() {
            for (i, _) in row.iter().enumerate() {
                let ch = self.at((i as i32, j as i32));
                let pos = (i as i32, j as i32);
                match ch {
                    CELL_GUARD_UP => return Some((pos, UP)),
                    CELL_GUARD_RIGHT => return Some((pos, RIGHT)),
                    CELL_GUARD_DOWN => return Some((pos, DOWN)),
                    CELL_GUARD_LEFT => return Some((pos, LEFT)),
                    _ => (),
                }
            }
        }

        None
    }

    // Calculate how many different obstacles in the map make the guard loop.
    fn calculate_posible_obstacles(&self) -> (usize, usize) {
        let visited_positions = self.get_visited_positions();
        let num_posible_obstacles = visited_positions
            .par_iter() // Parallel iterator
            .filter(|&position| self.check_obstacle(*position))
            .count();

        (visited_positions.len(), num_posible_obstacles)
    }

    // Check if an obstacle produces a loop
    fn check_obstacle(&self, obstacle_pos: (i32, i32)) -> bool {
        let (mut pos, mut dir) = self.find_guard().expect("Expected a guard");
        if pos == obstacle_pos {
            return false;
        }

        let mut past_positions: HashSet<(Position, Direction)> = HashSet::new();
        loop {
            if past_positions.contains(&(pos, dir)) {
                return true;
            }

            past_positions.insert((pos, dir));

            let next_position = (pos.0 + dir.0, pos.1 + dir.1);
            if !self.is_inside(next_position) {
                break;
            } else if (next_position == obstacle_pos) || (self.at(next_position) == CELL_OBSTACLE) {
                dir = turn_right(&dir);
            } else {
                pos.0 += dir.0;
                pos.1 += dir.1;
            }
        }

        false
    }
}

fn main() {
    println!("Day 6");

    let map: Map = Map::from_file(INPUT_FILE).expect("Could not read the input");
    let (num_visited_positions, num_posible_obstacles) = map.calculate_posible_obstacles();

    println!("[Part 1] {}", num_visited_positions);
    println!("[Part 2] {}", num_posible_obstacles);
}

#[test]
fn test_example() {
    const MAP: &str = "....#.....\n\
                       .........#\n\
                       ..........\n\
                       ..#.......\n\
                       .......#..\n\
                       ..........\n\
                       .#..^.....\n\
                       ........#.\n\
                       #.........\n\
                       ......#...\n\
                      ";
    let map = Map::from_string(MAP).expect("Could not read map");

    let (visited_positions, num_obstacles) = map.calculate_posible_obstacles();
    assert_eq!(visited_positions, 41);
    assert_eq!(num_obstacles, 6);
}
