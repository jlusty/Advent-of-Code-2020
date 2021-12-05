use regex::Regex;
use std::collections::HashMap;
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

    let re =
        Regex::new(r"(?P<ext>.*) bags contain ((?P<middle>.*)\d+ (?P<last>[^\d]*)|no other bags).")
            .unwrap();
    let middle_re = Regex::new(r"\d+ (?P<color>[^\d]*) bags?(, |.)").unwrap();

    let mut is_contained_by = HashMap::<String, Vec<String>>::new();
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        // let middle = caps["middle"].to_string();
        let middle_caps: Vec<String> = middle_re
            .captures_iter(line)
            .map(|c| c["color"].to_string())
            .collect();
        let external_cap = caps["ext"].to_string();
        // println!("{}: {:?}", external_cap, middle_caps);

        // is_contained_by map
        for c in middle_caps {
            let c_vec = is_contained_by.get_mut(&c);
            match c_vec {
                Some(vec_c) => {
                    vec_c.push(external_cap.clone());
                }
                None => {
                    is_contained_by.insert(c, vec![external_cap.clone()]);
                }
            };
        }
    }
    // println!("{:?}", is_contained_by);
    // Traverse is_contained_by map
    let mut containing_colors_buf = Vec::new();
    match is_contained_by.get("shiny gold") {
        Some(colors) => {
            containing_colors_buf.extend(colors);
            containing_colors_buf.sort();
            containing_colors_buf.dedup();
        }
        None => {}
    }
    let mut containing_colors_done = containing_colors_buf.clone();
    while containing_colors_buf.len() != 0 {
        containing_colors_buf = Vec::new();
        for c in &containing_colors_done {
            match is_contained_by.get(*c) {
                Some(colors) => {
                    for col in colors {
                        if !containing_colors_done.contains(&col) {
                            containing_colors_buf.push(col);
                            containing_colors_buf.sort();
                            containing_colors_buf.dedup();
                        }
                    }
                }
                None => {}
            }
        }
        containing_colors_done.extend(containing_colors_buf.clone());
        // println!("{:?}", containing_colors_buf);
    }
    // println!("{:?}", containing_colors_done);
    println!("{}", containing_colors_done.len());
}

#[derive(Debug, Clone)]
struct NumberColour {
    number: u32,
    color: String,
}

fn part_2() {
    let input = read_file("./input.txt".to_string()).unwrap();

    let re =
        Regex::new(r"(?P<ext>.*) bags contain ((?P<middle>.*)\d+ (?P<last>[^\d]*)|no other bags).")
            .unwrap();
    let middle_re = Regex::new(r"(?P<number>\d+) (?P<color>[^\d]*) bags?(, |.)").unwrap();

    let mut is_containing = HashMap::<String, Vec<NumberColour>>::new();
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let middle_caps: Vec<NumberColour> = middle_re
            .captures_iter(line)
            .map(|c| NumberColour {
                number: c["number"].to_string().parse().unwrap(),
                color: c["color"].to_string(),
            })
            .collect();
        let external_cap = caps["ext"].to_string();

        // containing map
        is_containing.insert(external_cap, middle_caps);
    }
    // println!("{:?}", is_containing);
    let mut is_containing_buf = Vec::<NumberColour>::new();
    match is_containing.get("shiny gold") {
        Some(numcol) => {
            is_containing_buf.extend(numcol.clone());
            // is_containing_count.sort();
            // is_containing_count.dedup();
        }
        None => {}
    }
    // println!("{:?}", is_containing_buf);
    let mut is_containing_done = is_containing_buf.clone();
    let mut at_bottom = false;
    let mut total_bags = 0;
    while !at_bottom {
        at_bottom = true;
        is_containing_buf = Vec::new();
        for c in &is_containing_done {
            match is_containing.get(&c.color) {
                Some(colors) => {
                    at_bottom = false;
                    for col in colors {
                        is_containing_buf.push(NumberColour {
                            number: c.number * col.number,
                            color: col.color.clone(),
                        });
                    }
                }
                None => {}
            }
        }
        for bag in is_containing_done {
            total_bags += bag.number;
        }
        is_containing_done = is_containing_buf.clone();
        // println!("buf: {:?}", is_containing_buf);
        // println!("done: {:?}", is_containing_done);
        // println!("{}", total_bags);
    }
    println!("{}", total_bags);
    // println!("{}", is_containing_done.len());
}
