mod utils;

use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use utils::CharBlock;

const INPUT_FILE: &str = "input/day08.txt";

type Position = (isize, isize);

fn main() {
    println!("Day 8");

    let map = CharBlock::from_file(INPUT_FILE).expect("Could not read block from file");

    // Part 1: Count the number of antinodes
    let antinodes = calculate_antinodes_for_map(&map);
    println!("[Part 1] {}", antinodes.len());
}

fn get_antena_positions(map: &CharBlock) -> HashMap<char, Vec<Position>> {
    let mut antena_positions: HashMap<char, Vec<Position>> = HashMap::new();

    for (j, row) in map.iter_rows().enumerate() {
        for (i, ch) in row.iter().enumerate() {
            if ch.is_alphanumeric() {
                antena_positions
                    .entry(*ch)
                    .or_default()
                    .push((i as isize, j as isize));
            }
        }
    }

    antena_positions
}

fn calculate_antinodes(p0: Position, p1: Position) -> Vec<Position> {
    let mut antinodes = Vec::new();

    let d01 = (p0.0 - p1.0, p0.1 - p1.1);
    let d10 = (p1.0 - p0.0, p1.1 - p0.1);
    let a01 = (p0.0 + 2 * d10.0, p0.1 + 2 * d10.1);
    let a10 = (p1.0 + 2 * d01.0, p1.1 + 2 * d01.1);

    antinodes.push(a01);
    antinodes.push(a10);

    antinodes
}

fn calculate_antinodes_for_map(map: &CharBlock) -> HashSet<Position> {
    let mut antinodes = HashSet::new();
    let antena_positions = get_antena_positions(map);

    for (_, positions) in antena_positions {
        let pairs: Vec<Vec<Position>> = positions.into_iter().combinations(2).collect();

        for pair in pairs.iter() {
            assert_eq!(pair.len(), 2, "Expected a pair of two antenas");
            let pair_antinodes = calculate_antinodes(pair[0], pair[1]);
            for antinode_position in pair_antinodes {
                if map.is_inside(antinode_position.0, antinode_position.1) {
                    antinodes.insert(antinode_position);
                }
            }
        }
    }

    antinodes
}

#[test]
fn test_example() {}
