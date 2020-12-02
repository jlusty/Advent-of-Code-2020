use std::error::Error;
use std::fs;

fn main() {
    part_1();
}

fn read_file(filename: String) -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string(filename)?;
    Ok(input)
}

#[derive(Debug)]
struct Policy {
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
        let policy = Policy {
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
