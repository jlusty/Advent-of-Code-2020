use regex::Regex;
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

#[derive(Debug)]
struct Passport2 {
    byr: u32,
    iyr: u32,
    eyr: u32,
    hgt: Height,
    hcl: String,
    ecl: String,
    pid: String,
    // cid: Option<String>,
}

#[derive(Debug)]
struct Height {
    value: u32,
    unit: String,
}

fn part_2() {
    let input = read_file("./input.txt".to_string()).unwrap();

    let mut passports_vec = Vec::<String>::new();
    let mut passport_str = String::from("");
    for line in input.lines() {
        if line == "" {
            passports_vec.push(passport_str);
            passport_str = String::from("");
        } else {
            passport_str.push_str(line);
            passport_str.push_str(" ");
        }
    }
    passports_vec.push(passport_str);

    let mut num_valid = 0;
    for ps in passports_vec {
        match get_passport_2(&ps) {
            Some(pass) => {
                if validate_pass2(pass) {
                    num_valid += 1;
                };
            }
            None => {}
        }
    }

    println!("{}", num_valid);
}

fn get_keyed_value_2(key: &str, value: &str) -> Option<String> {
    let re = Regex::new(&format!(r"{}:(?P<value>[^\s]*)", key)).unwrap();
    match re.captures(value) {
        Some(caps) => Some(caps["value"].to_string()),
        None => None,
    }
}

fn get_passport_2(stringified: &str) -> Option<Passport2> {
    let height_re = Regex::new(r"(?P<value>\d*)(?P<unit>cm|in)").unwrap();
    let height_str = get_keyed_value_2("hgt", stringified)?;
    Some(Passport2 {
        byr: get_keyed_value_2("byr", stringified)?.parse::<u32>().ok()?,
        iyr: get_keyed_value_2("iyr", stringified)?.parse::<u32>().ok()?,
        eyr: get_keyed_value_2("eyr", stringified)?.parse::<u32>().ok()?,
        hgt: Height {
            value: height_re.captures(&height_str)?["value"]
                .parse::<u32>()
                .ok()?,
            unit: height_re.captures(&height_str)?["unit"].to_string(),
        },
        hcl: get_keyed_value_2("hcl", stringified)?,
        ecl: get_keyed_value_2("ecl", stringified)?,
        pid: get_keyed_value_2("pid", stringified)?,
    })
}

fn validate_pass2(pass: Passport2) -> bool {
    if pass.byr < 1920 || pass.byr > 2002 {
        return false;
    }
    if pass.iyr < 2010 || pass.iyr > 2020 {
        return false;
    }
    if pass.eyr < 2020 || pass.eyr > 2030 {
        return false;
    }
    if pass.hgt.unit == "cm" {
        if pass.hgt.value < 150 || pass.hgt.value > 193 {
            return false;
        }
    } else if pass.hgt.unit == "in" {
        if pass.hgt.value < 59 || pass.hgt.value > 76 {
            return false;
        }
    } else {
        return false;
    }
    let hair_re = Regex::new(r"#[0-9a-f]{6}").unwrap();
    if pass.hcl.len() != 7 || !hair_re.is_match(&pass.hcl) {
        return false;
    }
    if !(pass.ecl == "amb"
        || pass.ecl == "blu"
        || pass.ecl == "brn"
        || pass.ecl == "gry"
        || pass.ecl == "grn"
        || pass.ecl == "hzl"
        || pass.ecl == "oth")
    {
        return false;
    }
    if pass.pid.len() != 9 || pass.pid.parse::<u32>().ok().is_none() {
        return false;
    }
    return true;
}
