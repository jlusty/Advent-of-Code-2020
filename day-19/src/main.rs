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

#[derive(Debug)]
enum RuleValue {
    RuleStr(String),
    RuleRef(u32),
}

#[derive(Debug)]
struct SingleRule {
    rule: Vec<RuleValue>,
}

#[derive(Debug)]
struct Rule {
    rules: Vec<SingleRule>,
}

fn part_1() {
    let input = read_file("./input.txt".to_string()).unwrap();

    let mut lines_vec = Vec::<String>::new();
    for line in input.lines() {
        lines_vec.push(line.to_string())
    }

    let mut rules = HashMap::<u32, Rule>::new();
    let mut messages = Vec::<String>::new();
    let mut getting_rules = true;
    for line in lines_vec {
        if line == "" {
            getting_rules = false;
            continue;
        }
        if getting_rules {
            let mut rule_parts = line.split(": ");
            let rule_num = rule_parts.next().unwrap().parse().unwrap();
            let rule_unparsed = rule_parts.next().unwrap().to_string();
            let mut rule = Rule { rules: Vec::new() };
            for w in rule_unparsed.split(" ") {
                match w {
                    "|" => rule.rules.push(SingleRule { rule: Vec::new() }),
                    "\"a\"" => rule
                        .rules
                        .last()
                        .unwrap()
                        .rule
                        .push(RuleValue::RuleStr(String::from("a"))),
                    "\"b\"" => rule
                        .rules
                        .last()
                        .unwrap()
                        .rule
                        .push(RuleValue::RuleStr(String::from("b"))),
                    num => rule
                        .rules
                        .last()
                        .unwrap()
                        .rule
                        .push(RuleValue::RuleRef(num.parse().unwrap())),
                }
            }
            rules.insert(rule_num, rule);
        } else {
            messages.push(line);
        }
    }
    // println!("{:?}", rules);
    // println!("{:?}", messages);

    // let to_trace = rules.get(&0).unwrap();
    // let mut vec_of_rules = to_trace.refs.clone();
    // let is_replacing = true;
    // while is_replacing {
    //     for rule in &vec_of_rules {
    //         for num in &rule {
    //             let tracing_rule = true;
    //             while tracing_rule
    //         }
    //     }
    // }

    // let mut rule_str_vec = Vec::new();
    // // println!("refs: {:?}", refs);
    // for rule in vec_of_rules {
    //     let mut rule_str = String::from("");
    //     for num in rule {
    //         let r = rules.get(&num).unwrap();
    //         rule_str = format!("{}{}", rule_str, r.st.as_ref().unwrap()[0])
    //     }
    //     rule_str_vec.push(rule_str);
    // }
    // // println!("rule_str_vec: {:?}", rule_str_vec);

    // let mut num_valid = 0;
    // for m in messages {
    //     // println!("{}", m);
    //     if rule_str_vec.contains(&m) {
    //         num_valid += 1;
    //     }
    // }
    // println!("{}", num_valid);
}
