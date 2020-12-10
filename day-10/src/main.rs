use std::error::Error;
use std::fs;

fn main() {
    // part_1();
    part_2();
}

fn read_file(filename: String) -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string(filename)?;
    Ok(input)
}

fn part_1() {
    let input = read_file("./input.txt".to_string()).unwrap();

    let mut jolt_vec = Vec::<u32>::new();
    for line in input.lines() {
        jolt_vec.push(line.parse().unwrap())
    }

    jolt_vec.sort();
    let mut diffs = vec![0; 4];

    // First diff
    diffs[jolt_vec[0] as usize] += 1;
    for i in 1..jolt_vec.len() {
        let diff = jolt_vec[i] - jolt_vec[i - 1];
        diffs[diff as usize] += 1;
    }
    // Last diff
    diffs[3] += 1;

    let ans = diffs[1] * diffs[3];
    println!("{}", ans);
}

fn part_2() {
    let input = read_file("./input.txt".to_string()).unwrap();

    let mut jolt_vec = Vec::<u32>::new();
    for line in input.lines() {
        jolt_vec.push(line.parse().unwrap())
    }

    jolt_vec.sort();

    let mut three_apart_subsets = Vec::new();
    let mut set_start = 0;
    for i in 1..jolt_vec.len() {
        if jolt_vec[i] - jolt_vec[i - 1] == 3 {
            three_apart_subsets.push(&jolt_vec[set_start..i]);
            set_start = i;
        }
    }
    three_apart_subsets.push(&jolt_vec[set_start..jolt_vec.len()]);
    println!("{:?}", jolt_vec);
    println!("{:?}", three_apart_subsets);

    let mut num_ways: u64 = get_num_ways(&three_apart_subsets[0], three_apart_subsets[0][0]) as u64;
    println!("{}", num_ways);
    for i in 1..three_apart_subsets.len() {
        let int_num_ways = get_num_ways(&three_apart_subsets[i], 3);
        println!("{}", int_num_ways);
        num_ways *= int_num_ways as u64;
    }
    println!("{}", num_ways);
}

fn get_num_ways(jolt_vec: &[u32], distance_since_last: u32) -> u32 {
    if distance_since_last > 3 {
        // println!("Too far away: {}", jolt_vec[0]);
        return 0;
    }
    if jolt_vec.len() == 1 {
        // println!("At end: {}", jolt_vec[0]);
        return 1;
    }
    let new_dist = jolt_vec[1] - jolt_vec[0];
    if distance_since_last == 3 || distance_since_last + new_dist > 3 {
        // println!("Gap of 3: {}", jolt_vec[0]);
        return get_num_ways(&jolt_vec[1..jolt_vec.len()], new_dist);
    }
    // println!("Everything valid: {}", jolt_vec[0]);
    get_num_ways(&jolt_vec[1..jolt_vec.len()], new_dist)
        + get_num_ways(&jolt_vec[1..jolt_vec.len()], distance_since_last + new_dist)
}
