use std::error::Error;
use std::fs;

fn main() {
    part_1();
    part_2();
}

fn read_file(filename: String) -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string(filename)?;
    Ok(input)
}

#[derive(Debug)]
struct Policy1 {
    min: u32,
    max: u32,
    character: char,
    pwd: String,
}

fn part_1() {
    let input = read_file("./input.txt".to_string()).unwrap();

    let mut num_valid = 0;
    for line in input.lines() {
        let policy_vec: Vec<&str> = line.split(|c| c == '-' || c == ':' || c == ' ').collect();
        let char_vec: Vec<char> = policy_vec[2].chars().collect();
        let policy = Policy1 {
            min: policy_vec[0].parse().unwrap(),
            max: policy_vec[1].parse().unwrap(),
            character: char_vec[0],
            pwd: policy_vec[4].to_owned(),
        };
        let instances: Vec<char> = policy
            .pwd
            .chars()
            .filter(|c| c == &policy.character)
            .collect();
        let num_instances = instances.len() as u32;

        let valid = num_instances >= policy.min && num_instances <= policy.max;

        if valid {
            num_valid += 1;
        }
    }

    println!("{}", num_valid);
}

#[derive(Debug)]
struct Policy2 {
    pos1: u32,
    pos2: u32,
    character: char,
    pwd: String,
}

fn part_2() {
    let input = read_file("./input.txt".to_string()).unwrap();

    let mut num_valid = 0;
    for line in input.lines() {
        let policy_vec: Vec<&str> = line.split(|c| c == '-' || c == ':' || c == ' ').collect();
        let char_vec: Vec<char> = policy_vec[2].chars().collect();
        let policy = Policy2 {
            pos1: policy_vec[0].parse().unwrap(),
            pos2: policy_vec[1].parse().unwrap(),
            character: char_vec[0],
            pwd: policy_vec[4].to_owned(),
        };
        let char1option: Option<char> = policy.pwd.chars().nth(policy.pos1 as usize - 1);
        let char2option: Option<char> = policy.pwd.chars().nth(policy.pos2 as usize - 1);
        if char1option.is_some() && char2option.is_some() {
            let char1 = char1option.unwrap();
            let char2 = char2option.unwrap();

            let valid = (char1 == policy.character && char2 != policy.character)
                || (char1 != policy.character && char2 == policy.character);

            if valid {
                num_valid += 1;
            }
        }
    }

    println!("{}", num_valid);
}
