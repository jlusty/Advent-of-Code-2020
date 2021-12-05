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

    let mut ins_vec = Vec::<(String, i32)>::new();
    for line in input.lines() {
        let mut ins_iter = line.split(' ');
        let op = ins_iter.next().unwrap();
        let arg: i32 = ins_iter.next().unwrap().parse().unwrap();
        ins_vec.push((op.to_string(), arg));
    }
    // println!("{:?}", ins_vec);

    let mut num_evals = vec![0; ins_vec.len()];
    let mut acc = 0;
    let mut i = 0;
    loop {
        let ins = &ins_vec[i];
        num_evals[i] += 1;
        if num_evals[i] > 1 {
            break;
        }
        match ins.0.as_ref() {
            "acc" => {
                acc += ins.1;
                i += 1;
            }
            "jmp" => {
                i = (i as i32 + ins.1) as usize;
            }
            "nop" => {
                i += 1;
            }
            _ => panic!("Unkown opcode"),
        }
    }
    println!("{}", acc);
}

fn part_2() {
    let input = read_file("./input.txt".to_string()).unwrap();

    let mut ins_vec = Vec::<(String, i32)>::new();
    for line in input.lines() {
        let mut ins_iter = line.split(' ');
        let op = ins_iter.next().unwrap();
        let arg: i32 = ins_iter.next().unwrap().parse().unwrap();
        ins_vec.push((op.to_string(), arg));
    }
    // println!("{:?}", ins_vec);

    let master_ins_vec = ins_vec.clone();
    let mut acc = 0;
    for i in 0..ins_vec.len() {
        let mut ins_vec = master_ins_vec.clone();
        if ins_vec[i].0 != "jmp" {
            continue;
        }
        ins_vec[i].0 = "nop".to_string();

        let mut num_evals = vec![0; ins_vec.len()];
        acc = 0;
        let mut i = 0;
        let mut found_ans = false;
        loop {
            if i >= ins_vec.len() {
                found_ans = true;
                break;
            }
            let ins = &ins_vec[i];
            num_evals[i] += 1;
            if num_evals[i] > 1 {
                break;
            }
            match ins.0.as_ref() {
                "acc" => {
                    acc += ins.1;
                    i += 1;
                }
                "jmp" => {
                    i = (i as i32 + ins.1) as usize;
                }
                "nop" => {
                    i += 1;
                }
                _ => panic!("Unkown opcode"),
            }
        }
        if found_ans {
            break;
        }
    }
    println!("{}", acc);
}
