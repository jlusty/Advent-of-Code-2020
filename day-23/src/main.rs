use std::collections::LinkedList;
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

    let mut cup_label_vec = Vec::<u32>::new();
    for line in input.lines() {
        for c in line.chars() {
            cup_label_vec.push(c.to_digit(10).unwrap())
        }
    }

    let mut current_cup_idx = 0;
    for _ in 0..100 {
        let current_cup = cup_label_vec[current_cup_idx];
        let removed_cups = [
            cup_label_vec[(current_cup_idx + 1) % cup_label_vec.len()],
            cup_label_vec[(current_cup_idx + 2) % cup_label_vec.len()],
            cup_label_vec[(current_cup_idx + 3) % cup_label_vec.len()],
        ];
        let mut destination_cup = current_cup - 1;
        if destination_cup == 0 {
            destination_cup = 9;
        }
        while removed_cups.contains(&destination_cup) {
            destination_cup = if destination_cup == 1 {
                9 // Maximum cup label
            } else {
                destination_cup - 1
            };
        }
        // println!("-- move {} --", i + 1);
        // println!("cups: {:?}", cup_label_vec);
        // println!("current cup: {:?}", current_cup);
        // println!("pick up: {:?}", removed_cups);
        // println!("destination: {:?}", destination_cup);
        // println!("");
        let mut new_cup_label_vec = Vec::new();
        for label in cup_label_vec {
            if removed_cups.contains(&label) {
                continue;
            }
            new_cup_label_vec.push(label);
            if label == destination_cup {
                new_cup_label_vec.extend_from_slice(&removed_cups);
            }
        }
        cup_label_vec = new_cup_label_vec;
        while cup_label_vec[current_cup_idx] != current_cup {
            current_cup_idx += 1;
        }
        current_cup_idx = (current_cup_idx + 1) % cup_label_vec.len();
    }

    let mut index_of_1 = 0;
    while cup_label_vec[index_of_1] != 1 {
        index_of_1 += 1;
    }
    for i in 0..cup_label_vec.len() - 1 {
        print!(
            "{}",
            cup_label_vec[(index_of_1 + 1 + i) % cup_label_vec.len()]
        )
    }
    println!("");
}

fn part_2() {
    let input = read_file("./input.txt".to_string()).unwrap();

    let mut cup_label_list = LinkedList::<u32>::new();
    for line in input.lines() {
        for c in line.chars() {
            cup_label_list.push_back(c.to_digit(10).unwrap())
        }
    }

    for _ in 0..100 {
        println!("{:?}", cup_label_list);

        let current_cup = cup_label_list.pop_front().unwrap();
        let removed_cups = [
            cup_label_list.pop_front().unwrap(),
            cup_label_list.pop_front().unwrap(),
            cup_label_list.pop_front().unwrap(),
        ];
        cup_label_list.push_back(current_cup);

        let mut destination_cup = current_cup - 1;
        if destination_cup == 0 {
            destination_cup = 9;
        }
        while removed_cups.contains(&destination_cup) {
            destination_cup = if destination_cup == 1 {
                9 // Maximum cup label
            } else {
                destination_cup - 1
            };
        }
        // println!("-- move {} --", move_num + 1);
        // println!("cups: {:?}", cup_label_vec);
        // println!("current cup: {:?}", current_cup);
        // println!("pick up: {:?}", removed_cups);
        // println!("destination: {:?}", destination_cup);
        // println!("");

        let mut i = 0;
        for label in cup_label_list.iter() {
            if *label == destination_cup {
                i += 1;
                let mut new_list = cup_label_list.split_off(i);
                for rc_idx in 0..3 {
                    new_list.push_front(removed_cups[2 - rc_idx]);
                }
                cup_label_list.append(&mut new_list);
                break;
            }
            i += 1;
        }
    }

    let mut index_of_1 = 0;
    for cup_label in cup_label_list.iter() {
        if *cup_label == 1 {
            let mut new_start_of_list = cup_label_list.split_off(index_of_1);
            new_start_of_list.append(&mut cup_label_list);

            let mut output_ans = new_start_of_list.iter();
            output_ans.next();
            for i in output_ans {
                print!("{}", i)
            }
            println!("");
            return;
        }
        index_of_1 += 1;
    }
}
