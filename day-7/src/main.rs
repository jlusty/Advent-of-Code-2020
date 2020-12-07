use regex::Regex;
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
