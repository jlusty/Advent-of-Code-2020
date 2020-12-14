use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn main() {
    part_2();
}

fn read_file(filename: String) -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string(filename)?;
    Ok(input)
}

#[derive(Debug)]
struct Mask {
    zeroing_mask: u64,
    value_mask: u64,
}

#[derive(Debug)]
struct MemIns {
    address: u64,
    value: u64,
}

#[derive(Debug)]
enum Instruction {
    Mask(Mask),
    MemIns(MemIns),
}

fn part_1() {
    let input = read_file("./input.txt".to_string()).unwrap();

    let mut lines_vec = Vec::<Instruction>::new();
    let mut max_index = 0;
    for line in input.lines() {
        if &line[0..3] == "mas" {
            let bitmask = line[7..43].to_string();
            // println!("{}", bitmask);
            let mut zeroing_mask = 0;
            let mut value_mask = 0;
            let mut i = 0;
            for c in bitmask.chars() {
                match c {
                    'X' => {}
                    '0' => {
                        zeroing_mask += 2u64.pow(35 - i);
                    }
                    '1' => {
                        zeroing_mask += 2u64.pow(35 - i);
                        value_mask += 2u64.pow(35 - i);
                    }
                    _ => panic!("Unknown character in mask definition"),
                }
                i += 1;
            }
            lines_vec.push(Instruction::Mask(Mask {
                zeroing_mask,
                value_mask,
            }))
        } else if &line[0..3] == "mem" {
            let mut line_parts = line.split(' ');
            let address = line_parts.next().unwrap();
            let address_val = address[4..address.len() - 1].parse().unwrap();
            line_parts.next();
            let value = line_parts.next().unwrap();
            lines_vec.push(Instruction::MemIns(MemIns {
                address: address_val,
                value: value.parse().unwrap(),
            }));
            if address_val > max_index {
                max_index = address_val;
            };
        }
    }

    // println!("{:?}", lines_vec);
    let mut address_space: Vec<u64> = vec![0; 1 + max_index as usize];
    let mut current_mask: Mask = Mask {
        zeroing_mask: 0,
        value_mask: 0,
    };
    for ins in lines_vec {
        match ins {
            Instruction::Mask(mask) => {
                current_mask = mask;
            }
            Instruction::MemIns(mem_ins) => {
                // println!("{:b}", mem_ins.value);
                // println!("{:b}", current_mask.zeroing_mask);
                let value_to_remove = mem_ins.value & current_mask.zeroing_mask;
                // println!("{:b}", value_to_remove);
                address_space[mem_ins.address as usize] =
                    mem_ins.value - value_to_remove + current_mask.value_mask;
            }
        }
    }

    let mut running_total = 0;
    for i in address_space {
        running_total += i;
    }
    println!("{}", running_total);
}

#[derive(Debug)]
enum Instruction2 {
    Mask(Mask2),
    MemIns(MemIns),
}

#[derive(Debug)]
struct Mask2 {
    position_mask: u64,
    floating_mask: u64,
    overwriting_mask: u64,
}

fn part_2() {
    let input = read_file("./input.txt".to_string()).unwrap();

    let mut lines_vec = Vec::<Instruction2>::new();
    let mut max_index = 0;
    for line in input.lines() {
        if &line[0..3] == "mas" {
            let bitmask = line[7..43].to_string();
            // println!("{}", bitmask);
            let mut position_mask = 0;
            let mut floating_mask = 0;
            let mut overwriting_mask = 0;
            let mut i = 0;
            for c in bitmask.chars() {
                match c {
                    '0' => {}
                    '1' => {
                        position_mask += 2u64.pow(35 - i);
                        overwriting_mask += 2u64.pow(35 - i);
                    }
                    'X' => {
                        position_mask += 2u64.pow(35 - i);
                        floating_mask += 2u64.pow(35 - i);
                    }
                    _ => panic!("Unknown character in mask definition"),
                }
                i += 1;
            }
            lines_vec.push(Instruction2::Mask(Mask2 {
                position_mask,
                floating_mask,
                overwriting_mask,
            }))
        } else if &line[0..3] == "mem" {
            let mut line_parts = line.split(' ');
            let address = line_parts.next().unwrap();
            let address_val = address[4..address.len() - 1].parse().unwrap();
            line_parts.next();
            let value = line_parts.next().unwrap();
            lines_vec.push(Instruction2::MemIns(MemIns {
                address: address_val,
                value: value.parse().unwrap(),
            }));
            if address_val > max_index {
                max_index = address_val;
            };
        }
    }

    // println!("{:?}", lines_vec);
    let mut address_space: HashMap<u64, u64> = HashMap::new();
    let mut current_mask: Mask2 = Mask2 {
        position_mask: 0,
        floating_mask: 0,
        overwriting_mask: 0,
    };
    for ins in lines_vec {
        match ins {
            Instruction2::Mask(mask) => {
                current_mask = mask;
            }
            Instruction2::MemIns(mem_ins) => {
                // println!("{:b}", mem_ins.value);
                // println!("{:b}", current_mask.zeroing_mask);
                let value_to_remove = mem_ins.address & current_mask.position_mask;
                // println!("{:b}", value_to_remove);
                let mut base_writer_vec = Vec::new();
                base_writer_vec
                    .push(mem_ins.address - value_to_remove + current_mask.overwriting_mask);
                for i in 0..36 {
                    if current_mask.floating_mask & 2u64.pow(i) != 0 {
                        for bw in base_writer_vec.clone() {
                            base_writer_vec.push(bw + 2u64.pow(i));
                        }
                    }
                }
                for bw in base_writer_vec {
                    // let value_to_write =
                    //     address_space[mem_ins.address as usize] - value_to_remove + bw;
                    // println!(
                    //     "address {} written, was {}, now {:b}",
                    //     value_to_write, address_space[value_to_write as usize], value_to_write
                    // );
                    address_space.insert(bw, mem_ins.value);
                }
                println!("");
                // address_space[mem_ins.address as usize] = mem_ins.value;
            }
        }
    }

    let mut running_total = 0;
    for i in address_space.values() {
        running_total += i;
    }
    println!("{}", running_total);
}
