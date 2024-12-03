use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_FILE: &str = "input/day01.txt";

fn main() {
    println!("Day 1");

    // Read lists from file
    let (list1, list2) = read_input(INPUT_FILE).expect("Failed to read input file");

    assert_eq!(list1.len(), list2.len(), "Lists must have the same length");

    // Part 1: Total distance
    let dist = calculate_total_distance(&list1, &list2);
    println!("[Part 1] {}", dist);

    // Part 2: Similarity index
    let similarity = calculate_similarity_index(&list1, &list2);
    println!("[Part 2] {}", similarity);
}

fn read_input(file_path: &str) -> Result<(Vec<u32>, Vec<u32>), Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut list1 = Vec::with_capacity(1000); // Allocate capacity for efficiency
    let mut list2 = Vec::with_capacity(1000);

    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split_whitespace();

        let first: u32 = parts
            .next()
            .ok_or("Missing first ID")?
            .parse()
            .map_err(|_| "Cannot convert first ID to u32")?;
        let second: u32 = parts
            .next()
            .ok_or("Missing second ID")?
            .parse()
            .map_err(|_| "Cannot convert second ID to u32")?;

        list1.push(first);
        list2.push(second);
    }

    Ok((list1, list2))
}

fn calculate_total_distance(list1: &[u32], list2: &[u32]) -> u32 {
    let mut sorted1 = list1.to_vec();
    let mut sorted2 = list2.to_vec();

    sorted1.sort_unstable();
    sorted2.sort_unstable();

    sorted1
        .iter()
        .zip(sorted2.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

fn calculate_similarity_index(list1: &[u32], list2: &[u32]) -> u32 {
    let count1 = count_elements(list1);
    let count2 = count_elements(list2);

    count1
        .iter()
        .map(|(&id, &count1)| {
            let count2 = count2.get(&id).copied().unwrap_or(0);
            id * (count1 * count2) as u32
        })
        .sum()
}

fn count_elements(list: &[u32]) -> HashMap<u32, usize> {
    let mut counts = HashMap::new();
    for &id in list {
        *counts.entry(id).or_insert(0) += 1;
    }
    counts
}
