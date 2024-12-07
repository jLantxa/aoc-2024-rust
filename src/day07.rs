use itertools::Itertools;
use rayon::{
    self,
    iter::{IntoParallelRefIterator, ParallelIterator},
};
use strum::EnumIter;

const INPUT_FILE: &str = "input/day07.txt";

type Equation = (i64, Vec<i64>);

#[derive(Debug, EnumIter, Clone, Copy)]
enum Op {
    Add,
    Mul,
    Concat,
}

fn main() {
    println!("Day 7");

    let equations = read_input(INPUT_FILE).expect("Could not read input");

    // Part 1
    println!(
        "[Part 1] {}",
        process_valid_equations_with_combinations(&equations, &vec![Op::Add, Op::Mul])
    );
    // Part 2
    println!(
        "[Part 2] {}",
        process_valid_equations_with_combinations(&equations, &vec![Op::Add, Op::Mul, Op::Concat])
    );
}

fn read_input(file_path: &str) -> Result<Vec<Equation>, Box<dyn std::error::Error>> {
    let text: String = std::fs::read_to_string(file_path)?;
    parse_input(&text)
}

fn parse_input(text: &str) -> Result<Vec<Equation>, Box<dyn std::error::Error>> {
    let mut equations = Vec::new();
    for line in text.lines() {
        let sides: Vec<_> = line.split(":").collect();
        let left: i64 = sides
            .first()
            .expect("Expected a left side")
            .parse()
            .expect("Expected an integer value");
        let right: Vec<i64> = sides
            .last()
            .expect("Expected a right side")
            .trim()
            .split(" ")
            .map(|str_num| str_num.parse().expect("Expected an integer element"))
            .collect();
        equations.push((left, right));
    }

    Ok(equations)
}

fn generate_op_combinations(valid_ops: &Vec<Op>, n: usize) -> Vec<Vec<Op>> {
    (0..n)
        .map(|_| valid_ops.clone()) // Repeat the slice `n` times
        .multi_cartesian_product()
        .collect()
}

fn eval_expr_with_ops(expr: &[i64], ops: &[Op]) -> Option<i64> {
    let size = expr.len();
    if size == 0 {
        return None;
    }

    let right: i64 = *expr.last().unwrap();

    if size == 1 {
        return Some(right);
    }

    let left = &expr[..(size - 1)];
    let op_right = *ops.last().expect("Expected an Op");
    let ops_left = &ops[..(ops.len() - 1)];

    match op_right {
        Op::Add => Some(right + eval_expr_with_ops(left, ops_left).unwrap()),
        Op::Mul => Some(right * eval_expr_with_ops(left, ops_left).unwrap()),
        Op::Concat => {
            let pre = eval_expr_with_ops(left, ops_left).unwrap().to_string();
            let post = right.to_string();
            Some((pre + &post).parse().unwrap())
        }
    }
}

fn process_valid_equations_with_combinations(equations: &[Equation], valid_ops: &Vec<Op>) -> i64 {
    let mut result = 0;
    for eq in equations {
        let op_combinations = generate_op_combinations(&valid_ops, eq.1.len() - 1);
        let valid_combinations = op_combinations
            .par_iter()
            .map(|ops| eval_expr_with_ops(&eq.1, &ops).unwrap())
            .filter(|res| *res == eq.0)
            .count();

        if valid_combinations > 0 {
            result += eq.0;
        }
    }

    result
}

#[test]
fn test_example() {
    const INPUT: &str = "190: 10 19\n\
     3267: 81 40 27\n\
     83: 17 5\n\
     156: 15 6\n\
     7290: 6 8 6 15\n\
     161011: 16 10 13\n\
     192: 17 8 14\n\
     21037: 9 7 18 13\n\
     292: 11 6 16 20";

    let equations = parse_input(INPUT).expect("Could not parse example");
    assert_eq!(
        process_valid_equations_with_combinations(&equations, &vec![Op::Add, Op::Mul]),
        3749
    );
    assert_eq!(
        process_valid_equations_with_combinations(&equations, &vec![Op::Add, Op::Mul, Op::Concat]),
        11387
    );
}
