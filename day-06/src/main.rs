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

    let mut groups_vec = Vec::<String>::new();
    let mut group_str = String::from("");
    for line in input.lines() {
        if line == "" {
            groups_vec.push(group_str);
            group_str = String::from("");
        } else {
            group_str.push_str(line);
        }
    }
    groups_vec.push(group_str);

    let mut total_questions = 0;
    for group in groups_vec {
        let mut qu_vec: Vec<char> = group.chars().collect();
        qu_vec.sort();
        qu_vec.dedup();
        total_questions += qu_vec.len();
    }
    println!("{}", total_questions);
}

fn part_2() {
    let input = read_file("./input.txt".to_string()).unwrap();

    let mut total_questions = 0;
    let mut everyone_yes = Vec::<char>::new();
    let mut new_group = true;
    for line in input.lines() {
        if line == "" {
            total_questions += everyone_yes.len();
            new_group = true;
        } else {
            if new_group {
                everyone_yes = line.chars().collect();
                new_group = false;
            } else {
                let questions: Vec<char> = line.chars().collect();
                let mut i = 0;
                while i < everyone_yes.len() {
                    if !questions.contains(&everyone_yes[i]) {
                        everyone_yes.remove(i);
                    } else {
                        i += 1;
                    }
                }
            }
        }
    }
    total_questions += everyone_yes.len();
    println!("{}", total_questions);
}
