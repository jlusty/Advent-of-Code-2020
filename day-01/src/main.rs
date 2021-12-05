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

fn part_1() {
    let input = read_file("./input.txt".to_string()).unwrap();

    let mut num_vec = Vec::<u32>::new();
    for line in input.lines() {
        num_vec.push(line.parse().unwrap())
    }

    let mut i = 0;
    let mut j = 0;
    while !(num_vec[i] + num_vec[j] == 2020) {
        if j < num_vec.len() - 1 {
            j += 1;
        } else {
            i += 1;
            j = 0;
        }
    }

    println!("{} {}", num_vec[i], num_vec[j]);
    
    let ans = num_vec[i] * num_vec[j];
    println!("Answer is {}", ans);
}

fn part_2() {
    let input = read_file("./input.txt".to_string()).unwrap();

    let mut num_vec = Vec::<u32>::new();
    for line in input.lines() {
        num_vec.push(line.parse().unwrap())
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while !(num_vec[i] + num_vec[j] + num_vec[k] == 2020) {
        if k < num_vec.len() - 1 {
            k += 1;
        } else {
            k = 0;
            if j < num_vec.len() - 1 {
                j += 1;
            } else {
                i += 1;
                j = 0;
            }
        }
    }

    println!("{} {} {}", num_vec[i], num_vec[j], num_vec[k]);
    
    let ans = num_vec[i] * num_vec[j] * num_vec[k];
    println!("Answer is {}", ans);
}