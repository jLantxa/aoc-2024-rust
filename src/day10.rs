use std::collections::HashSet;
use std::collections::VecDeque;
use std::iter::FromIterator;

const INPUT_FILE: &str = "input/day10.txt";

type Map = Vec<Vec<u8>>;
const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn main() {
    // No more using the grid
    println!("Day 10");

    let map = parse_input(INPUT_FILE).expect("Could not read input");
    let (total_score, total_rating) = calculate_trails(&map);
    println!("[Part 1] {}", total_score);
    println!("[Part 2] {}", total_rating);
}

fn parse_map(input: &str) -> Map {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn parse_input(file_path: &str) -> Result<Map, Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(file_path)?;
    Ok(parse_map(&input))
}

fn inside_map(i: isize, j: isize, map: &Map) -> bool {
    let width = map[0].len() as isize;
    let height = map.len() as isize;

    (i >= 0) && (i < width) && (j >= 0) && (j < height)
}

fn bfs_trailhead(map: &Map, start: (isize, isize)) -> Vec<(isize, isize)> {
    let mut queue = VecDeque::new();

    queue.push_back((start, 0));

    let mut endpoints = Vec::new();

    while let Some(((i, j), height)) = queue.pop_front() {
        for (di, dj) in DIRECTIONS {
            let next = (i as isize + di, j as isize + dj);

            if !inside_map(next.0, next.1, map) {
                continue;
            }

            let next_height = map[next.0 as usize][next.1 as usize];
            if next_height == height + 1 {
                if next_height == 9 {
                    endpoints.push(next);
                } else {
                    queue.push_back((next, next_height));
                }
            }
        }
    }

    endpoints
}

fn calculate_trails(map: &Map) -> (usize, usize) {
    let mut total_score = 0;
    let mut total_rating = 0;

    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == 0 {
                let endpoints = bfs_trailhead(map, (x as isize, y as isize));

                // All different endpoints (repeated, from different paths)
                total_rating += endpoints.len();

                // All unique endpoints (reachable destinations)
                total_score += HashSet::<(isize, isize)>::from_iter(endpoints).len();
            }
        }
    }

    (total_score, total_rating)
}

#[test]
fn test_example() {
    const INPUT: &str = "89010123\n\
                         78121874\n\
                         87430965\n\
                         96549874\n\
                         45678903\n\
                         32019012\n\
                         01329801\n\
                         10456732";

    let map = parse_map(INPUT);
    let (total_score, total_rating) = calculate_trails(&map);
    assert_eq!(total_score, 36);
    assert_eq!(total_rating, 81);
}
