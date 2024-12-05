use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_FILE: &str = "input/day05.txt";

type OrderingRule = (u8, u8);
type Update = Vec<u8>;

fn main() {
    println!("Day 5");

    let input = read_input(INPUT_FILE).expect("Could not read input file");

    let rules = &input.0;
    let updates = &input.1;

    // Part 1: Do a first pass, process the correct updates and filter incorrect updates
    let (part_1_value, mut second_updates) = process_updates(updates, rules);
    println!("[Part 1] {}", part_1_value);

    // Part 2: Fix incorrect updates and do a second processing
    second_updates
        .iter_mut()
        .for_each(|u| sort_update(u, rules));

    let (part_2_value, _) = process_updates(&second_updates, rules);
    println!("[Part 2] {}", part_2_value);
}

fn compare(lhs: u8, rhs: u8, rules: &Vec<OrderingRule>) -> Ordering {
    // This assumes that there exists a rule for every pair of numbers
    for rule in rules {
        if rule.0 == lhs && rule.1 == rhs {
            return Ordering::Less;
        } else if rule.0 == rhs && rule.1 == lhs {
            return Ordering::Greater;
        }
    }

    // Need some default if no rule exists, which will never happen
    Ordering::Greater
}

fn check_update(update: &Update, rules: &Vec<OrderingRule>) -> Option<u8> {
    let is_sorted = update.is_sorted_by(|lhs, rhs| compare(*lhs, *rhs, rules) == Ordering::Less);
    if is_sorted {
        let median_index = update.len() / 2;
        Some(update[median_index])
    } else {
        None
    }
}

fn sort_update(update: &mut Update, rules: &Vec<OrderingRule>) {
    update.sort_unstable_by(|lhs, rhs| compare(*lhs, *rhs, rules));
}

fn process_updates(updates: &Vec<Update>, rules: &Vec<OrderingRule>) -> (u32, Vec<Update>) {
    let mut result_correct_updates: u32 = 0;
    let mut incorrect_updates = Vec::new();
    for update in updates {
        let check_result = check_update(&update, rules);

        match check_result {
            Some(n) => result_correct_updates += n as u32,
            None => incorrect_updates.push(update.to_vec()),
        }
    }

    (result_correct_updates, incorrect_updates)
}

fn read_input(
    file_path: &str,
) -> Result<(Vec<OrderingRule>, Vec<Update>), Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut rules = Vec::new();
    let mut updates = Vec::new();
    let mut lines = reader.lines().into_iter();

    // Process first block with the update rules
    loop {
        let line = lines.next().expect("Expected an update rule")?;
        if line.trim() == "" {
            break;
        }

        let elements: Vec<&str> = line.split("|").collect();
        rules.push((
            elements[0].parse().expect("Expected an integer"),
            elements[1].parse().expect("Expected an integer"),
        ));
    }

    // Process second block with the updates
    for line in lines {
        let line_unwrapped = line?;
        let elements: Vec<&str> = line_unwrapped.split(",").collect();
        updates.push(
            elements
                .iter()
                .map(|s| s.parse().expect("Could not parse value into integer"))
                .collect(),
        );
    }

    Ok((rules, updates))
}
