use regex::Regex;
use std::error::Error;
use std::fs;

fn main() {
    part_1();
}

fn read_file(filename: String) -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string(filename)?;
    Ok(input)
}

#[derive(Debug)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    // cid: Option<String>,
}

fn part_1() {
    let input = read_file("./input.txt".to_string()).unwrap();

    let mut passports_vec = Vec::<String>::new();
    let mut passport_str = String::from("");
    for line in input.lines() {
        if line == "" {
            passports_vec.push(passport_str);
            passport_str = String::from("");
        } else {
            passport_str.push_str(line);
        }
    }
    passports_vec.push(passport_str);

    let mut num_valid = 0;
    for ps in passports_vec {
        match get_passport(&ps) {
            Some(_) => num_valid += 1,
            None => {}
        }
    }

    println!("{}", num_valid);
}

fn get_keyed_value(key: &str, value: &str) -> Option<String> {
    let re = Regex::new(&format!(r"{}:(?P<value>[^\s]*)", key)).unwrap();
    match re.captures(value) {
        Some(caps) => Some(caps["value"].to_string()),
        None => None,
    }
}

fn get_passport(stringified: &str) -> Option<Passport> {
    Some(Passport {
        byr: get_keyed_value("byr", stringified)?,
        iyr: get_keyed_value("iyr", stringified)?,
        eyr: get_keyed_value("eyr", stringified)?,
        hgt: get_keyed_value("hgt", stringified)?,
        hcl: get_keyed_value("hcl", stringified)?,
        ecl: get_keyed_value("ecl", stringified)?,
        pid: get_keyed_value("pid", stringified)?,
        // cid: Some(get_keyed_value("cid", stringified)).unwrap_or(None),
    })
}
