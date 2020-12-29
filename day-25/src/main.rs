use std::error::Error;
use std::fs;

fn main() {
    part_1();
}

fn read_file(filename: String) -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string(filename)?;
    Ok(input)
}

const DIVIDER: u64 = 20201227;

fn part_1() {
    let input = read_file("./input.txt".to_string()).unwrap();

    let mut num_vec = Vec::<u64>::new();
    for line in input.lines() {
        num_vec.push(line.parse().unwrap())
    }

    let card_pub_key = num_vec[0];
    let door_pub_key = num_vec[1];

    let subject_num: u64 = 7;

    let card_loop_size = get_loop_size(subject_num, card_pub_key);
    // let door_loop_size = get_loop_size(subject_num, door_pub_key);

    let encryption_key = get_transformed_sub(door_pub_key, card_loop_size);
    println!("{}", encryption_key);
}

fn get_loop_size(subject_num: u64, pub_key: u64) -> u64 {
    let mut new_subject_num = subject_num;
    let mut loop_size: u64 = 1;
    while new_subject_num != pub_key {
        new_subject_num = (new_subject_num * subject_num) % DIVIDER;
        loop_size += 1;
    }

    return loop_size;
}

fn get_transformed_sub(subject_num: u64, loop_size: u64) -> u64 {
    let mut transformed_sub: u64 = 1;
    for _ in 0..loop_size {
        transformed_sub = (transformed_sub * subject_num) % DIVIDER
    }
    return transformed_sub;
}
