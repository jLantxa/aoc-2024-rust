mod utils;

use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use utils::Grid;

const INPUT_FILE: &str = "input/day08.txt";

type Position = (isize, isize);

fn main() {
    println!("Day 8");

    let map = Grid::from_file(INPUT_FILE).expect("Could not read block from file");

    // Part 1: Count the number of antinodes
    let non_resonant_antinodes = calculate_antinodes_for_map(&map, false);
    println!("[Part 1] {}", non_resonant_antinodes.len());

    // Part 2: Include resonant nodes
    let resonant_nodes = calculate_antinodes_for_map(&map, true);
    println!("[Part 2] {}", resonant_nodes.len());
}

fn get_antena_positions(map: &Grid) -> HashMap<char, Vec<Position>> {
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

fn calculate_antinodes(
    map: &Grid,
    p0: Position,
    p1: Position,
    include_resonant: bool,
) -> Vec<Position> {
    let mut antinodes = Vec::new();
    let d10 = (p0.0 - p1.0, p0.1 - p1.1);
    let d01 = (p1.0 - p0.0, p1.1 - p0.1);

    let mut k01 = if include_resonant { 1 } else { 2 };
    let mut k10 = k01;
    loop {
        let a01 = (p0.0 + k01 * d01.0, p0.1 + k01 * d01.1);
        if !map.is_inside(a01.0, a01.1) || (!include_resonant && k01 > 2) {
            break;
        }
        antinodes.push(a01);
        k01 += 1;
    }
    loop {
        let a10 = (p1.0 + k10 * d10.0, p1.1 + k10 * d10.1);
        if !map.is_inside(a10.0, a10.1) || (!include_resonant && k10 > 2) {
            break;
        }
        antinodes.push(a10);
        k10 += 1;
    }

    antinodes
}

fn calculate_antinodes_for_map(map: &Grid, include_resonant: bool) -> HashSet<Position> {
    let mut antinodes = HashSet::new();
    let antena_positions = get_antena_positions(map);

    for (_, positions) in antena_positions {
        let pairs: Vec<Vec<Position>> = positions.into_iter().combinations(2).collect();

        for pair in pairs.iter() {
            assert_eq!(pair.len(), 2, "Expected a pair of two antenas");
            let pair_antinodes = calculate_antinodes(map, pair[0], pair[1], include_resonant);
            for antinode_position in pair_antinodes {
                antinodes.insert(antinode_position);
            }
        }
    }

    antinodes
}

#[test]
fn test_example() {}
