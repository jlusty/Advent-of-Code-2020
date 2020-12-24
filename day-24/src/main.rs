use std::collections::HashMap;
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

    let mut dir_vec = Vec::<Vec<String>>::new();
    for line in input.lines() {
        let mut letter_buf = ' ';
        let mut one_line_dir_vec = Vec::new();
        for c in line.chars() {
            if c == 'n' || c == 's' {
                letter_buf = c;
            } else {
                one_line_dir_vec.push(format!("{}{}", letter_buf, c).trim().to_string());
                letter_buf = ' ';
            }
        }
        dir_vec.push(one_line_dir_vec);
    }

    let mut hexagon_map = HashMap::<(i32, i32), u32>::new();
    for dir_line in dir_vec {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        for dir in dir_line {
            match &dir[..] {
                "e" => x += 2,
                "se" => {
                    y -= 1;
                    x += 1;
                }
                "sw" => {
                    y -= 1;
                    x -= 1;
                }
                "w" => x -= 2,
                "nw" => {
                    y += 1;
                    x -= 1
                }
                "ne" => {
                    y += 1;
                    x += 1;
                }
                _ => panic!("Unknown direction"),
            }
        }
        hexagon_map
            .entry((x, y))
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    let mut num_black_tiles = 0;
    for hex in hexagon_map.values() {
        if hex % 2 == 1 {
            num_black_tiles += 1;
        }
    }
    println!("{}", num_black_tiles);
}
