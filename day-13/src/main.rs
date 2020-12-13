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

    let mut line_vec = Vec::<&str>::new();
    for line in input.lines() {
        line_vec.push(line)
    }

    let earliest_depart: i32 = line_vec[0].parse().unwrap();
    let id_vec: Vec<i32> = line_vec[1]
        .split(",")
        .filter(|id| id != &"x")
        .map(|id| id.parse().unwrap())
        .collect::<Vec<i32>>();

    let mut earliest_bus = 0;
    let mut minutes_to_wait = 0;
    for id in id_vec {
        if -(earliest_depart % id) + id < minutes_to_wait || earliest_bus == 0 {
            earliest_bus = id;
            minutes_to_wait = -(earliest_depart % id) + id;
        }
        // println!("{}", -(earliest_depart % id) + id);
    }
    println!("{}", earliest_bus * minutes_to_wait);
}

fn part_2() {
    let input = read_file("./input.txt".to_string()).unwrap();

    let mut line_vec = Vec::<&str>::new();
    for line in input.lines() {
        line_vec.push(line)
    }

    let id_vec: Vec<Option<i64>> = line_vec[1]
        .split(",")
        .map(|id| {
            if id == "x" {
                None
            } else {
                Some(id.parse().unwrap())
            }
        })
        .collect::<Vec<Option<i64>>>();

    let mut modulo_arr = Vec::new();
    for i in 0..id_vec.len() {
        let id_opt = id_vec[i];
        match id_opt {
            Some(id) => modulo_arr.push((id, i as i64)),
            None => {}
        }
    }
    modulo_arr.sort_by_key(|a| a.0);
    // println!("{:?}", modulo_arr);

    // println!("{:?}", modulo_arr[0]);
    let mut checking_for: i64 = (modulo_arr[0].0 as i64 - modulo_arr[0].1 as i64) % modulo_arr[0].0;
    if modulo_arr[0].1 == 0 {
        checking_for = 0;
    }
    while checking_for < 0 {
        checking_for += modulo_arr[0].0;
    }
    let mut current_num = checking_for;
    // println!(
    //     "{} is {} mod {}",
    //     current_num, checking_for, modulo_arr[0].0
    // );
    let mut current_adder = modulo_arr[0].0;
    // println!("Will now add {}", current_adder);
    for i in 1..modulo_arr.len() {
        let m = modulo_arr[i];
        let mut checking_for = (m.0 - m.1) % m.0;
        if m.1 == 0 {
            checking_for = 0;
        }
        while checking_for < 0 {
            checking_for += m.0;
        }
        // println!("Checking for {} mod {}", checking_for, m.0);
        while current_num % m.0 != checking_for {
            current_num += current_adder;
            // println!("{}, {}", current_num, current_adder);
        }
        // println!("{} is {} mod {}", current_num, checking_for, m.0);
        current_adder *= m.0;
        // println!("Will now add {}", current_adder);
    }
    println!("{}", current_num);
}
