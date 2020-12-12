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

    let mut ins_vec = Vec::<(char, i32)>::new();
    for line in input.lines() {
        ins_vec.push((
            line.chars().next().unwrap(),
            line[1..line.len()].parse().unwrap(),
        ))
    }
    // println!("{:?}", ins_vec);

    let mut current_pos: (i32, i32) = (0, 0);
    let dir_cycle = ['N', 'E', 'S', 'W'];
    let mut current_dir: i32 = 1; // East
    for ins in ins_vec {
        match ins.0 {
            'N' => current_pos.0 += ins.1,
            'E' => current_pos.1 += ins.1,
            'S' => current_pos.0 -= ins.1,
            'W' => current_pos.1 -= ins.1,
            'L' => current_dir = (current_dir + 4 - (ins.1 / 90)) % 4,
            'R' => current_dir = (current_dir + 4 + (ins.1 / 90)) % 4,
            'F' => match dir_cycle[current_dir as usize] {
                'N' => current_pos.0 += ins.1,
                'E' => current_pos.1 += ins.1,
                'S' => current_pos.0 -= ins.1,
                'W' => current_pos.1 -= ins.1,
                _ => panic!("Unknown direction"),
            },
            _ => panic!("Unknown action"),
        }
    }
    println!("{}", current_pos.0.abs() + current_pos.1.abs());
}
