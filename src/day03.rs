use regex::Regex;

const INPUT_FILE: &str = "input/day03.txt";

enum Operation {
    Mul(i32, i32),
    Enable,
    Disable,
}

fn main() {
    println!("Day 3");

    let memory = read_input(INPUT_FILE).expect("Failed to read input file");

    // Part 1
    let value = process_str(&memory, false);
    println!("[Part 1] Value: {}", value);

    // Part 2
    let value = process_str(&memory, true);
    println!("[Part 2] Value: {}", value);
}

fn read_input(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(std::fs::read_to_string(file_path)?)
}

fn process_str(memory: &str, with_enable_ops: bool) -> i32 {
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();

    let mut ops = Vec::new();

    for caps in mul_re.captures_iter(memory) {
        if let (Some(x), Some(y)) = (caps.get(1), caps.get(2)) {
            if let (Ok(x), Ok(y)) = (x.as_str().parse::<i32>(), y.as_str().parse::<i32>()) {
                ops.push((caps.get(0).unwrap().start(), Operation::Mul(x, y)));
            }
        }
    }

    if with_enable_ops {
        for caps in do_re.captures_iter(memory) {
            ops.push((caps.get(0).unwrap().start(), Operation::Enable));
        }

        for caps in dont_re.captures_iter(memory) {
            ops.push((caps.get(0).unwrap().start(), Operation::Disable));
        }

        ops.sort_by_key(|(start, _)| *start);
    }

    let ops = ops.into_iter().map(|(_, op)| op).collect();

    process_ops(&ops)
}

fn process_ops(ops: &Vec<Operation>) -> i32 {
    let mut enabled = true;
    let mut value: i32 = 0;

    for op in ops {
        match op {
            Operation::Enable => enabled = true,
            Operation::Disable => enabled = false,
            Operation::Mul(a, b) => {
                if enabled {
                    value += a * b
                }
            }
        }
    }

    value
}

#[test]
fn test_example() {
    const MEMORY_0: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    assert_eq!(process_str(MEMORY_0, false), 161);

    const MEMORY_1: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert_eq!(process_str(MEMORY_1, true), 48);
}
