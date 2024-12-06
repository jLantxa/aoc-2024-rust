use std::collections::HashSet;

const INPUT_FILE: &str = "input/day06.txt";

type Direction = (i32, i32);

const CELL_OBSTACLE: char = '#';
const CELL_GUARD_UP: char = '^';
const CELL_GUARD_DOWN: char = 'v';
const CELL_GUARD_LEFT: char = '<';
const CELL_GUARD_RIGHT: char = '>';

const UP: Direction = (0, -1);
const RIGHT: Direction = (1, 0);
const DOWN: Direction = (0, 1);
const LEFT: Direction = (-1, 0);

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

    fn at(&self, i: i32, j: i32) -> char {
        self.block[j as usize][i as usize]
    }

    fn dimensions(&self) -> (i32, i32) {
        (self.block[0].len() as i32, self.block.len() as i32)
    }

    fn is_inside(&self, i: i32, j: i32) -> bool {
        let dimensions = self.dimensions();
        (i >= 0 && i < dimensions.0) && (j >= 0 && j < dimensions.1)
    }

    fn get_visited_positions(&self) -> HashSet<(i32, i32)> {
        let mut visited_positions = HashSet::new();
        let (mut pos, mut dir) = self.find_guard().expect("Expected a guard");

        loop {
            visited_positions.insert((pos.0, pos.1));

            let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
            if !self.is_inside(next_pos.0, next_pos.1) {
                break;
            } else if self.at(next_pos.0, next_pos.1) == CELL_OBSTACLE {
                dir = turn_right(&dir);
            }

            pos.0 += dir.0;
            pos.1 += dir.1;
        }

        visited_positions
    }

    fn find_guard(&self) -> Option<((i32, i32), Direction)> {
        for (j, row) in self.block.iter().enumerate() {
            for (i, _) in row.iter().enumerate() {
                let ch = self.at(i as i32, j as i32);
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
}

fn main() {
    println!("Day 6");

    let map: Map = Map::from_file(INPUT_FILE).expect("Could not read the input");

    // Part 1
    let visited_positions = map.get_visited_positions();
    println!("[Part 1] {}", visited_positions.len());
}

fn turn_right(dir: &Direction) -> Direction {
    (-dir.1, dir.0)
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

    let visited_positions = map.get_visited_positions();
    assert_eq!(visited_positions.len(), 41);
}
