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

fn part_2() {
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

    // Display
    // for y in -10..10 {
    //     for x in -10..10 {
    //         if hexagon_map.get(&(x, y)).is_some() && hexagon_map.get(&(x, y)).unwrap() % 2 == 1 {
    //             print!("b");
    //         } else {
    //             print!(" ");
    //         }
    //     }
    //     println!("");
    // }
    // let mut num_black_tiles = 0;
    // for hex in hexagon_map.values() {
    //     if hex % 2 == 1 {
    //         num_black_tiles += 1;
    //     }
    // }
    // println!("{:?}", hexagon_map);
    // println!("Day 0: {}", num_black_tiles);

    for day in 0..100 {
        let mut new_hexagon_map = hexagon_map.clone();
        let x_range = (
            hexagon_map.keys().map(|(x, _)| x).min().unwrap() - 2,
            hexagon_map.keys().map(|(x, _)| x).max().unwrap() + 2,
        );
        let y_range = (
            hexagon_map.keys().map(|(_, y)| y).min().unwrap() - 2,
            hexagon_map.keys().map(|(_, y)| y).max().unwrap() + 2,
        );
        for y in y_range.0..y_range.1 {
            for x in x_range.0..x_range.1 {
                let hex = hexagon_map.get(&(x, y));

                let adjacent_tiles = [
                    hexagon_map.get(&(x + 2, y)),
                    hexagon_map.get(&(x + 1, y - 1)),
                    hexagon_map.get(&(x - 1, y - 1)),
                    hexagon_map.get(&(x - 2, y)),
                    hexagon_map.get(&(x - 1, y + 1)),
                    hexagon_map.get(&(x + 1, y + 1)),
                ];
                let mut num_adjacent_black = 0;
                for tile in adjacent_tiles.iter() {
                    if tile.is_some() && tile.unwrap() % 2 == 1 {
                        num_adjacent_black += 1;
                    }
                }
                if hex.is_some() && hex.unwrap() % 2 == 1 {
                    // Current tile is black
                    if num_adjacent_black == 0 || num_adjacent_black > 2 {
                        new_hexagon_map.insert((x, y), 0);
                    }
                } else {
                    // Current tile is white
                    if num_adjacent_black == 2 {
                        new_hexagon_map.insert((x, y), 1);
                    }
                }
            }
        }
        hexagon_map = new_hexagon_map.clone();
        let mut num_black_tiles = 0;
        for hex in hexagon_map.values() {
            if hex % 2 == 1 {
                num_black_tiles += 1;
            }
        }
        println!("Day {}: {}", day + 1, num_black_tiles);
    }
}
