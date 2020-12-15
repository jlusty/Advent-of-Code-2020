use std::error::Error;
use std::fs;

fn main() {
    part_1();
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

    let mut nums_spoken = vec![0];
    for i in 0..2020 {
        if i < num_vec.len() {
            nums_spoken.push(num_vec[i])
        } else {
            let mut i = nums_spoken.len() - 2;
            while i > 0 {
                if nums_spoken[i] == nums_spoken[nums_spoken.len() - 1] {
                    // println!("spoken before: {}", nums_spoken[i]);
                    nums_spoken.push(nums_spoken.len() - 1 - i);
                    break;
                }
                i -= 1;
            }
            if i == 0 {
                nums_spoken.push(0);
            }
        }
        // println!("{}", nums_spoken[nums_spoken.len() - 1]);
    }
    println!("{}", nums_spoken[nums_spoken.len() - 1]);
}
