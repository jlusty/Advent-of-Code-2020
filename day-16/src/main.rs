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

    let mut rules_vec = Vec::<((u32, u32), (u32, u32))>::new();
    let mut my_ticket = Vec::<u32>::new();
    let mut ticket_vec = Vec::<Vec<u32>>::new();

    let mut lines_vec = Vec::new();
    for line in input.lines() {
        lines_vec.push(line);
    }

    let mut i = 0;
    while i < lines_vec.len() {
        let line = lines_vec[i];
        if line == "" {
            break;
        }
        let mut word_iter = line.split(' ');
        while word_iter.next().unwrap().chars().last().unwrap() != ':' {
            // Get next in while loop
        }
        let mut first_range = word_iter.next().unwrap().split('-');
        word_iter.next();
        let mut second_range = word_iter.next().unwrap().split('-');

        rules_vec.push((
            (
                first_range.next().unwrap().parse().unwrap(),
                first_range.next().unwrap().parse().unwrap(),
            ),
            (
                second_range.next().unwrap().parse().unwrap(),
                second_range.next().unwrap().parse().unwrap(),
            ),
        ));
        i += 1;
    }
    // println!("{:?}", rules_vec);
    i += 2;
    my_ticket = lines_vec[i]
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u32>>();
    // println!("{:?}", my_ticket);
    i += 3;
    while i < lines_vec.len() {
        ticket_vec.push(
            lines_vec[i]
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<u32>>(),
        );
        i += 1;
    }
    // println!("{:?}", ticket_vec);

    let mut completely_invalid_sum = 0;
    for ticket in ticket_vec {
        for num in ticket {
            let mut is_valid_somewhere = false;
            for rule in &rules_vec {
                if (num >= (rule.0).0 && num <= (rule.0).1)
                    || (num >= (rule.1).0 && num <= (rule.1).1)
                {
                    is_valid_somewhere = true;
                }
            }
            if !is_valid_somewhere {
                completely_invalid_sum += num;
            }
        }
    }
    println!("{}", completely_invalid_sum);
}

fn part_2() {
    let input = read_file("./input.txt".to_string()).unwrap();

    let mut rules_vec = Vec::<((u32, u32), (u32, u32), String)>::new();
    let mut my_ticket = Vec::<u32>::new();
    let mut ticket_vec = Vec::<Vec<u32>>::new();

    let mut lines_vec = Vec::new();
    for line in input.lines() {
        lines_vec.push(line);
    }

    let mut i = 0;
    while i < lines_vec.len() {
        let line = lines_vec[i];
        if line == "" {
            break;
        }
        let mut word_iter = line.split(' ');
        let mut field_name = word_iter.next().unwrap().to_string();
        while field_name.chars().last().unwrap() != ':' {
            field_name = format!("{}{}", field_name, word_iter.next().unwrap());
        }
        let mut first_range = word_iter.next().unwrap().split('-');
        word_iter.next();
        let mut second_range = word_iter.next().unwrap().split('-');

        rules_vec.push((
            (
                first_range.next().unwrap().parse().unwrap(),
                first_range.next().unwrap().parse().unwrap(),
            ),
            (
                second_range.next().unwrap().parse().unwrap(),
                second_range.next().unwrap().parse().unwrap(),
            ),
            field_name,
        ));
        i += 1;
    }
    // println!("{:?}", rules_vec);
    i += 2;
    my_ticket = lines_vec[i]
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u32>>();
    // println!("{:?}", my_ticket);
    i += 3;
    while i < lines_vec.len() {
        ticket_vec.push(
            lines_vec[i]
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<u32>>(),
        );
        i += 1;
    }
    // println!("{:?}", ticket_vec);

    let mut valid_tickets = Vec::new();
    for ticket in ticket_vec {
        let mut ticket_valid = true;
        for num in &ticket {
            let mut is_valid_somewhere = false;
            for rule in &rules_vec {
                if (*num >= (rule.0).0 && *num <= (rule.0).1)
                    || (*num >= (rule.1).0 && *num <= (rule.1).1)
                {
                    is_valid_somewhere = true;
                }
            }
            if !is_valid_somewhere {
                ticket_valid = false;
            }
        }
        if ticket_valid {
            valid_tickets.push(ticket);
        }
    }
    // println!("{:?}", valid_tickets);
    let mut impossible_fields = vec![Vec::new(); rules_vec.len()];
    for ticket in valid_tickets {
        for num_pos in 0..ticket.len() {
            let num = ticket[num_pos];
            for i in 0..rules_vec.len() {
                let rule = &rules_vec[i];
                if !((num >= (rule.0).0 && num <= (rule.0).1)
                    || (num >= (rule.1).0 && num <= (rule.1).1))
                {
                    if !impossible_fields[i].contains(&num_pos) {
                        impossible_fields[i].push(num_pos);
                    }
                }
            }
        }
    }
    let mut possible_fields = vec![Vec::new(); rules_vec.len()];
    for i in 0..impossible_fields.len() {
        impossible_fields[i].sort();
        for j in 0..rules_vec.len() {
            if !impossible_fields[i].contains(&j) {
                possible_fields[i].push(j)
            }
        }
    }
    let field_names = rules_vec
        .iter()
        .map(|r| r.2.clone())
        .collect::<Vec<String>>();
    let mut assigned_fields = HashMap::new();

    // println!("{:?}", field_names);
    let mut all_empty = false;
    while !all_empty {
        for i in 0..possible_fields.len() {
            if possible_fields[i].len() == 1 {
                assigned_fields.insert(&field_names[i], possible_fields[i][0]);
                // println!("{:?}", possible_fields[i]);
                // println!("{:?}", possible_fields[i]);
                // println!("Inserted {} for {}", &field_names[i], possible_fields[i][0]);
                possible_fields[i] = Vec::new();
                break;
            }
        }
        all_empty = true;
        for j in 0..possible_fields.len() {
            for assigned in assigned_fields.values() {
                let index = possible_fields[j].iter().position(|x| x == assigned);
                if index.is_some() {
                    // println!("removing {}", possible_fields[j][index.unwrap()]);
                    possible_fields[j].remove(index.unwrap());
                }
            }
            if possible_fields[j].len() != 0 {
                all_empty = false;
            }
        }
        // println!("{:?}", possible_fields);
    }
    // println!("{:?}", assigned_fields);
    let mut running_total: u64 = 1;
    for (field_name, val) in assigned_fields.into_iter() {
        if field_name.len() > 9 && &field_name[0..9] == "departure" {
            running_total *= my_ticket[val] as u64;
        }
    }
    println!("{}", running_total);
}
