use std::error::Error;
use std::fs;

const BUF_SIZE: usize = 25;

fn main() {
    let invalid_num = part_1();
    part_2(invalid_num);
}

fn read_file(filename: String) -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string(filename)?;
    Ok(input)
}

fn part_1() -> u64 {
    let input = read_file("./input.txt".to_string()).unwrap();

    let mut num_vec = Vec::<u64>::new();
    for line in input.lines() {
        num_vec.push(line.parse().unwrap())
    }

    // First generation of list
    let mut possible_sums = Vec::new();
    for i in 0..BUF_SIZE {
        for j in i + 1..BUF_SIZE {
            possible_sums.push((num_vec[i] + num_vec[j], i, j));
        }
    }
    // println!("{:?}", possible_sums);

    let mut ans = 0;
    for current_pos in BUF_SIZE..num_vec.len() {
        let possible_vals = possible_sums.iter().map(|ps| ps.0).collect::<Vec<u64>>();
        // Check if valid
        if !possible_vals.contains(&num_vec[current_pos]) {
            ans = num_vec[current_pos];
            break;
        }

        // Delete stuff from list
        let new_min = current_pos - BUF_SIZE + 1;
        let mut i = 0;
        while i < possible_sums.len() {
            let ps = possible_sums[i];
            if ps.1 < new_min || ps.2 < new_min {
                possible_sums.remove(i);
            } else {
                i += 1;
            }
        }

        // Another generation of list
        for i in 0..BUF_SIZE - 1 {
            possible_sums.push((
                num_vec[new_min + i] + num_vec[current_pos],
                new_min + i,
                current_pos,
            ));
        }
        // println!("{:?}", possible_sums);

        // println!("{:?}", possible_sums);
    }
    println!("{}", ans);
    return ans;
}

fn part_2(invalid_num: u64) {
    let input = read_file("./input.txt".to_string()).unwrap();

    let mut num_vec = Vec::<u64>::new();
    for line in input.lines() {
        num_vec.push(line.parse().unwrap())
    }

    let mut ans = 0;
    for start_pos in 0..num_vec.len() {
        let mut end_pos = start_pos + 1;
        let mut contiguous_sum: u64 = num_vec[start_pos..end_pos].into_iter().sum();
        while contiguous_sum < invalid_num && end_pos < num_vec.len() {
            end_pos += 1;
            contiguous_sum = num_vec[start_pos..end_pos].into_iter().sum();
            // println!(
            //     "start: {}, end: {}, sum: {}",
            //     start_pos, end_pos, contiguous_sum
            // );
            if contiguous_sum == invalid_num {
                ans = num_vec[start_pos..end_pos].into_iter().min().unwrap()
                    + num_vec[start_pos..end_pos].into_iter().max().unwrap();
                break;
            }
        }
    }
    println!("{}", ans);
}
