use std::error::Error;
use std::fs;

fn main() {
    part_2();
}

fn read_file(filename: String) -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string(filename)?;
    Ok(input)
}

fn part_1() {
    let input = read_file("./input.txt".to_string()).unwrap();

    let mut expr_vec = Vec::<String>::new();
    for line in input.lines() {
        expr_vec.push(line.to_string())
    }

    let mut sum_of_totals: u64 = 0;
    for line in expr_vec {
        let expr = line.chars().filter(|c| *c != ' ').collect::<Vec<char>>();
        // println!("{:?}", expr);

        let mut bracket_depth = vec![0u64];
        let mut bracket_op = Vec::new();
        let mut current_op = '+';
        for op in expr {
            match op {
                '(' => {
                    bracket_op.push(current_op);
                    bracket_depth.push(0);
                    current_op = '+';
                }
                ')' => {
                    let last_idx = bracket_depth.len() - 1;
                    match bracket_op[bracket_op.len() - 1] {
                        '+' => bracket_depth[last_idx - 1] += bracket_depth[last_idx],
                        '*' => bracket_depth[last_idx - 1] *= bracket_depth[last_idx],
                        _ => panic!("Unknown operator"),
                    }
                    bracket_depth.pop();
                    bracket_op.pop();
                }
                '+' => current_op = '+',
                '*' => current_op = '*',
                num => {
                    let last_idx = bracket_depth.len() - 1;
                    match current_op {
                        '+' => bracket_depth[last_idx] += num.to_digit(10).unwrap() as u64,
                        '*' => bracket_depth[last_idx] *= num.to_digit(10).unwrap() as u64,
                        _ => panic!("Unknown operator"),
                    }
                }
            }
        }
        sum_of_totals += bracket_depth[0];
    }
    println!("{}", sum_of_totals);
}

fn part_2() {
    let input = read_file("./input.txt".to_string()).unwrap();

    let mut expr_vec = Vec::<String>::new();
    for line in input.lines() {
        expr_vec.push(format!(
            "({})",
            line.to_string()
                .replace("(", "((")
                .replace(")", "))")
                .replace("*", ")*(")
        ))
    }

    let mut sum_of_totals: u64 = 0;
    for line in expr_vec {
        // println!("{}", line);
        let expr = line.chars().filter(|c| *c != ' ').collect::<Vec<char>>();

        let mut bracket_depth = vec![0u64];
        let mut bracket_op = Vec::new();
        let mut current_op = '+';
        for op in expr {
            match op {
                '(' => {
                    bracket_op.push(current_op);
                    bracket_depth.push(0);
                    current_op = '+';
                }
                ')' => {
                    let last_idx = bracket_depth.len() - 1;
                    match bracket_op[bracket_op.len() - 1] {
                        '+' => bracket_depth[last_idx - 1] += bracket_depth[last_idx],
                        '*' => bracket_depth[last_idx - 1] *= bracket_depth[last_idx],
                        _ => panic!("Unknown operator"),
                    }
                    bracket_depth.pop();
                    bracket_op.pop();
                }
                '+' => current_op = '+',
                '*' => current_op = '*',
                num => {
                    let last_idx = bracket_depth.len() - 1;
                    match current_op {
                        '+' => bracket_depth[last_idx] += num.to_digit(10).unwrap() as u64,
                        '*' => bracket_depth[last_idx] *= num.to_digit(10).unwrap() as u64,
                        _ => panic!("Unknown operator"),
                    }
                }
            }
        }
        // println!("{}", bracket_depth[0]);
        sum_of_totals += bracket_depth[0];
    }
    println!("{}", sum_of_totals);
}
