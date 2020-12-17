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

    let mut planes = HashMap::<i32, HashMap<i32, HashMap<i32, char>>>::new();
    planes.insert(0, HashMap::new());
    let mut lines = input.lines();
    let mut line = lines.next();
    let mut y = 0;
    let z_plane = planes.get_mut(&0).unwrap();
    while line.is_some() {
        let y_row = z_plane.entry(y).or_insert_with(|| HashMap::new());
        let l = line.unwrap();
        let mut chars = l.chars();
        let mut c = chars.next();
        let mut x = 0;
        while c.is_some() {
            y_row.insert(x, c.unwrap());
            x += 1;
            c = chars.next();
        }
        y += 1;
        line = lines.next();
    }
    println!("{:?}", planes);

    let mut next_state = planes.clone();
    for (z_base, z) in planes.iter() {
        for (y_base, y) in z.iter() {
            for (x_base, x) in y.iter() {
                let mut num_active_neighbours = 0;
                for z_pos in &[-1, 0, 1] {
                    for y_pos in &[-1, 0, 1] {
                        for x_pos in &[-1, 0, 1] {
                            if *z_pos == 0 && *y_pos == 0 && *x_pos == 0 {
                                continue;
                            }
                            if planes.get(&(z_base + z_pos)).is_some()
                                && planes
                                    .get(&(z_base + z_pos))
                                    .unwrap()
                                    .get(&(y_base + y_pos))
                                    .is_some()
                                && planes
                                    .get(&(z_base + z_pos))
                                    .unwrap()
                                    .get(&(y_base + y_pos))
                                    .unwrap()
                                    .get(&(x_base + x_pos))
                                    .is_some()
                                && *planes
                                    .get(&(z_base + z_pos))
                                    .unwrap()
                                    .get(&(y_base + y_pos))
                                    .unwrap()
                                    .get(&(x_base + x_pos))
                                    .unwrap()
                                    == '#'
                            {
                                num_active_neighbours += 1;
                            }
                        }
                    }
                }
                // println!("{}", num_active_neighbours);
                match x {
                    '#' => {
                        if !(num_active_neighbours == 2 || num_active_neighbours == 3) {
                            *next_state
                                .get_mut(z_base)
                                .unwrap()
                                .get_mut(y_base)
                                .unwrap()
                                .get_mut(x_base)
                                .unwrap() = '.';
                        }
                    }
                    '.' => {
                        if num_active_neighbours == 3 {
                            *next_state
                                .get_mut(z_base)
                                .unwrap()
                                .get_mut(y_base)
                                .unwrap()
                                .get_mut(x_base)
                                .unwrap() = '#';
                        }
                    }
                    _ => panic!("Unknown cube state"),
                }
            }
        }
    }

    println!("{:?}", next_state);
}
