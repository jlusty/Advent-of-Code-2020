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
