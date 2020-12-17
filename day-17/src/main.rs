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

    let mut planes = Vec::<Vec<Vec<char>>>::new();
    planes.push(Vec::<Vec<char>>::new());
    for line in input.lines() {
        planes[0].push(line.chars().collect());
    }
    for z in 0..planes.len() {
        println!("Plane z={}", z);
        for y in 0..planes[0].len() {
            for x in 0..planes[0][0].len() {
                print!("{}", planes[z][y][x]);
            }
            println!("");
        }
    }

    let mut next_state;
    for _ in 0..6 {
        next_state = planes.clone();
        for z in 0..planes.len() {
            for y in 0..planes[0].len() {
                next_state[z][y].insert(0, '.');
                next_state[z][y].push('.');
            }
            next_state[z].insert(0, vec!['.'; planes[0][0].len() + 2]);
            next_state[z].push(vec!['.'; planes[0][0].len() + 2]);
        }
        next_state.insert(
            0,
            vec![vec!['.'; planes[0].len() + 2]; planes[0][0].len() + 2],
        );
        next_state.push(vec![vec!['.'; planes[0].len() + 2]; planes[0][0].len() + 2]);
        planes = next_state.clone();

        // println!("Padded edge");
        // for z in 0..planes.len() {
        //     println!("Plane z={}", z);
        //     for y in 0..planes[0].len() {
        //         for x in 0..planes[0][0].len() {
        //             print!("{}", planes[z][y][x]);
        //         }
        //         println!("");
        //     }
        // }

        for z in 0..planes.len() {
            for y in 0..planes[0].len() {
                for x in 0..planes[0][0].len() {
                    let mut num_active_neighbours = 0;
                    for z_pos in &[-1i32, 0, 1] {
                        for y_pos in &[-1i32, 0, 1] {
                            for x_pos in &[-1i32, 0, 1] {
                                if *z_pos == 0 && *y_pos == 0 && *x_pos == 0
                                    || (z as i32 + z_pos) as usize == planes.len()
                                    || (z as i32 + z_pos) == -1
                                    || (y as i32 + y_pos) as usize == planes[0].len()
                                    || (y as i32 + y_pos) == -1
                                    || (x as i32 + x_pos) as usize == planes[0][0].len()
                                    || (x as i32 + x_pos) == -1
                                {
                                    continue;
                                }
                                if planes[(z as i32 + z_pos) as usize][(y as i32 + y_pos) as usize]
                                    [(x as i32 + x_pos) as usize]
                                    == '#'
                                {
                                    num_active_neighbours += 1;
                                }
                            }
                        }
                    }
                    match planes[z][y][x] {
                        '#' => {
                            if !(num_active_neighbours == 2 || num_active_neighbours == 3) {
                                next_state[z][y][x] = '.'
                            }
                        }
                        '.' => {
                            if num_active_neighbours == 3 {
                                next_state[z][y][x] = '#'
                            }
                        }
                        _ => panic!("Unknown cube state"),
                    }
                }
            }
        }

        planes = next_state.clone();
        for z in 0..planes.len() {
            println!("Plane z={}", z);
            for y in 0..planes[0].len() {
                for x in 0..planes[0][0].len() {
                    print!("{}", planes[z][y][x]);
                }
                println!("");
            }
        }
    }

    let mut num_active = 0;
    for z in 0..planes.len() {
        for y in 0..planes[0].len() {
            for x in 0..planes[0][0].len() {
                if planes[z][y][x] == '#' {
                    num_active += 1;
                }
            }
        }
    }

    println!("{}", num_active);
}
