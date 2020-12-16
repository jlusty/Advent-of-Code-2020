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
