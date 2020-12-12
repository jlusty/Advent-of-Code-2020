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

    let mut seat_grid = Vec::<Vec<char>>::new();
    for line in input.lines() {
        seat_grid.push(line.chars().collect());
    }
    // println!("{:?}", seat_grid);
    let grid_width = seat_grid[0].len();
    let grid_height = seat_grid.len();

    let empty_row = vec!['.'; grid_width];

    let mut has_changed = true;
    while has_changed {
        has_changed = false;
        let mut next_seat_grid = seat_grid.clone();
        for y in 0..grid_height {
            let mut above_row = &empty_row;
            let mut below_row = &empty_row;
            if y != 0 {
                above_row = &seat_grid[y - 1];
            }
            if y != grid_height - 1 {
                below_row = &seat_grid[y + 1];
            }
            for x in 0..grid_width {
                let mut left_end = 0;
                let mut right_end = grid_width - 1;
                if x != 0 {
                    left_end = x - 1;
                }
                if x != grid_width - 1 {
                    right_end = x + 1;
                }

                let above_seats = &above_row[left_end..right_end + 1];
                let current_row = &seat_grid[y][left_end..right_end + 1];
                let below_seats = &below_row[left_end..right_end + 1];
                let all_adj_seats = vec![above_seats, current_row, below_seats];
                match seat_grid[y][x] {
                    'L' => {
                        let mut num_seats = 0;
                        let mut num_unoccupied = 0;
                        for row in all_adj_seats {
                            for s in row {
                                num_seats += 1;
                                if s == &'.' || s == &'L' {
                                    num_unoccupied += 1;
                                }
                            }
                        }
                        if num_unoccupied == num_seats {
                            next_seat_grid[y][x] = '#';
                            has_changed = true;
                        }
                    }
                    '#' => {
                        let mut num_occupied = 0;
                        // print!("{}, {}, {:?}", x, y, &all_adj_seats);
                        for row in all_adj_seats {
                            for s in row {
                                if s == &'#' {
                                    num_occupied += 1;
                                }
                            }
                        }
                        if num_occupied > 4 {
                            next_seat_grid[y][x] = 'L';
                            has_changed = true;
                        }
                        // println!(", {}", num_occupied);
                    }
                    _ => {}
                }
            }
        }
        seat_grid = next_seat_grid;
        // for y in 0..grid_height {
        //     for x in 0..grid_width {
        //         print!("{}", seat_grid[y][x]);
        //     }
        //     println!("");
        // }
        // println!("");
    }

    let mut num_occupied_seats = 0;
    for y in 0..grid_height {
        for x in 0..grid_width {
            if seat_grid[y][x] == '#' {
                num_occupied_seats += 1;
            }
        }
    }
    println!("{}", num_occupied_seats);
}

fn part_2() {
    let input = read_file("./input.txt".to_string()).unwrap();

    let mut seat_grid = Vec::<Vec<char>>::new();
    for line in input.lines() {
        seat_grid.push(line.chars().collect());
    }
    // println!("{:?}", seat_grid);
    let grid_width = seat_grid[0].len();
    let grid_height = seat_grid.len();

    let mut has_changed = true;
    while has_changed {
        has_changed = false;
        let mut next_seat_grid = seat_grid.clone();
        for y in 0..grid_height {
            for x in 0..grid_width {
                if seat_grid[y][x] == '.' {
                    continue;
                }

                let mut num_empty = 0;
                let mut num_filled = 0;
                let mut num_seats = 0;

                for y_count in 0..3 {
                    let y_diff: i32 = y_count - 1;
                    for x_count in 0..3 {
                        let x_diff: i32 = x_count - 1;
                        if y_diff == 0 && x_diff == 0 {
                            continue;
                        }
                        // println!("{} {}", x_diff, y_diff);

                        let mut y_to_check = y as i32 + y_diff;
                        let mut x_to_check = x as i32 + x_diff;
                        let mut i = 1;
                        while x_to_check >= 0
                            && x_to_check < grid_width as i32
                            && y_to_check >= 0
                            && y_to_check < grid_height as i32
                        {
                            // println!(
                            //     "{} {} {} {} {} {}",
                            //     x_to_check, y_to_check, x, y, x_diff, y_diff
                            // );
                            match seat_grid[y_to_check as usize][x_to_check as usize] {
                                'L' => {
                                    num_empty += 1;
                                    num_seats += 1;
                                    // println!("L: {} {}", x_diff, y_diff);
                                    break;
                                }
                                '#' => {
                                    num_filled += 1;
                                    num_seats += 1;
                                    // println!("#: {} {}", x_diff, y_diff);
                                    break;
                                }
                                _ => {}
                            }
                            i += 1;
                            x_to_check = x as i32 + x_diff * i;
                            y_to_check = y as i32 + y_diff * i;
                        }
                    }
                }
                // println!("({}, {}): {} {} {}", x, y, num_seats, num_filled, num_empty);
                if seat_grid[y][x] == 'L' && num_seats == num_empty {
                    next_seat_grid[y][x] = '#';
                    has_changed = true;
                }
                if seat_grid[y][x] == '#' && num_filled >= 5 {
                    next_seat_grid[y][x] = 'L';
                    has_changed = true;
                }
            }
        }

        seat_grid = next_seat_grid;
        //     for y in 0..grid_height {
        //         for x in 0..grid_width {
        //             print!("{}", seat_grid[y][x]);
        //         }
        //         println!("");
        //     }
        //     println!("");
    }

    let mut num_occupied_seats = 0;
    for y in 0..grid_height {
        for x in 0..grid_width {
            if seat_grid[y][x] == '#' {
                num_occupied_seats += 1;
            }
        }
    }
    println!("{}", num_occupied_seats);
}
