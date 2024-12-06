const INPUT_FILE: &str = "input/day06.txt";

type Direction = (i32, i32);

const CELL_EMPTY: char = '.';
const CELL_OBSTACLE: char = '#';
const CELL_GUARD_UP: char = '^';
const CELL_GUARD_DOWN: char = 'v';
const CELL_GUARD_LEFT: char = '<';
const CELL_GUARD_RIGHT: char = '>';
const CELL_GUARD_VISITED: char = 'X';

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

    fn set(&mut self, i: i32, j: i32, value: char) {
        self.block[j as usize][i as usize] = value;
    }

    fn rows(&self) -> std::slice::Iter<'_, Vec<char>> {
        self.block.iter()
    }

    fn rows_mut(&mut self) -> std::slice::IterMut<'_, Vec<char>> {
        self.block.iter_mut()
    }

    fn dimensions(&self) -> (i32, i32) {
        (self.block[0].len() as i32, self.block.len() as i32)
    }

    fn is_inside(&self, i: i32, j: i32) -> bool {
        let dimensions = self.dimensions();
        (i >= 0 && i < dimensions.0) && (j >= 0 && j < dimensions.1)
    }
}

fn main() {
    println!("Day 6");

    let map: Map = Map::from_file(INPUT_FILE).expect("Could not read the input");

    // Part 1
    let num_guard_positions = calculate_guard_positions(&map);
    println!("[Part 1] {}", num_guard_positions);
}

fn turn_right(dir: &Direction) -> Direction {
    (-dir.1, dir.0)
}

fn turn_left(dir: &Direction) -> Direction {
    (dir.1, -dir.0)
}

fn annotate_guard_positions(map: &Map) -> Map {
    let mut annotated_map = map.clone();
    let (mut pos, mut dir) = find_guard(map).expect("Expected a guard");

    loop {
        annotated_map.set(pos.0, pos.1, CELL_GUARD_VISITED);

        let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
        if !annotated_map.is_inside(next_pos.0, next_pos.1) {
            break;
        } else if annotated_map.at(next_pos.0, next_pos.1) == CELL_OBSTACLE {
            dir = turn_right(&dir);
        }

        pos.0 += dir.0;
        pos.1 += dir.1;
    }

    annotated_map
}

fn find_guard(map: &Map) -> Option<((i32, i32), Direction)> {
    for (j, row) in map.rows().enumerate() {
        for (i, col) in row.iter().enumerate() {
            let ch = map.at(i as i32, j as i32);
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

fn count_guard_positions(annotated_map: &Map) -> u32 {
    let mut count = 0;

    for row in annotated_map.rows() {
        for cell in row {
            if *cell == CELL_GUARD_VISITED {
                count += 1;
            }
        }
    }

    count
}

fn calculate_guard_positions(map: &Map) -> u32 {
    let annotated_map = annotate_guard_positions(map);
    count_guard_positions(&annotated_map)
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
    assert_eq!(calculate_guard_positions(&map), 41);
}
