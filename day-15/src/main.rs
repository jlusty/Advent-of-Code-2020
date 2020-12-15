use std::collections::HashMap;
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

    let mut num_vec = Vec::<usize>::new();
    for line in input.lines() {
        num_vec = line
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect::<Vec<usize>>();
    }
    // println!("{:?}", num_vec);

    let mut nums_spoken = HashMap::<usize, usize>::new();
    let mut last_spoken = 0;
    let mut last_spoken_to_insert = last_spoken;
    for i in 0..2020 {
        if i < num_vec.len() {
            last_spoken = num_vec[i];
        } else {
            last_spoken = *nums_spoken.get(&last_spoken).unwrap_or(&0);
            if last_spoken != 0 {
                last_spoken = i - last_spoken;
            }
        }
        // println!("{}", nums_spoken[nums_spoken.len() - 1]);
        nums_spoken.insert(last_spoken_to_insert, i);
        last_spoken_to_insert = last_spoken;
    }
    println!("{}", last_spoken);
}

fn part_2() {
    let input = read_file("./input.txt".to_string()).unwrap();

    let mut num_vec = Vec::<usize>::new();
    for line in input.lines() {
        num_vec = line
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect::<Vec<usize>>();
    }
    // println!("{:?}", num_vec);

    let mut nums_spoken = HashMap::<usize, usize>::new();
    let mut last_spoken = 0;
    let mut last_spoken_to_insert = last_spoken;
    for i in 0..30000000 {
        if i < num_vec.len() {
            last_spoken = num_vec[i];
        } else {
            last_spoken = *nums_spoken.get(&last_spoken).unwrap_or(&0);
            if last_spoken != 0 {
                last_spoken = i - last_spoken;
            }
        }
        // println!("{}", nums_spoken[nums_spoken.len() - 1]);
        nums_spoken.insert(last_spoken_to_insert, i);
        last_spoken_to_insert = last_spoken;
        if i % 1000000 == 0 {
            println!("{}", i)
        }
    }
    println!("{}", last_spoken);
}
