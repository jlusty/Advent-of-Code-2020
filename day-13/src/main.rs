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
