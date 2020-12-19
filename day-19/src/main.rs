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
struct Rule {
    st: Option<Vec<String>>,
    refs: Vec<Vec<u32>>,
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
            let mut rule_parsed = vec![Vec::new()];
            let mut rule_st = None;
            for w in rule_unparsed.split(" ") {
                match w {
                    "|" => rule_parsed.push(Vec::new()),
                    "\"a\"" => rule_st = Some(vec![String::from("a")]),
                    "\"b\"" => rule_st = Some(vec![String::from("b")]),
                    num => {
                        let last_idx = rule_parsed.len() - 1;
                        rule_parsed[last_idx].push(num.parse().unwrap())
                    }
                }
            }
            rules.insert(
                rule_num,
                Rule {
                    st: rule_st,
                    refs: rule_parsed,
                },
            );
        } else {
            messages.push(line);
        }
    }
    // println!("{:?}", rules);
    // println!("{:?}", messages);

    let to_trace = rules.get(&0).unwrap();
    let mut refs = to_trace.refs.clone();
    for i in 0..10 {
        println!("{}", i);
        let mut all_new_refs = Vec::new();
        for rule in &refs {
            let mut new_refs = vec![Vec::<u32>::new()];
            // println!("rule: {:?}", rule);
            for num in rule {
                if rules.get(&num).unwrap().st.is_none() {
                    let traced_ref = &rules.get(&num).unwrap().refs;
                    // println!("{:?}", &traced_ref);
                    let mut new_new_refs = Vec::new();
                    for single_traced_ref in traced_ref {
                        // println!("str: {:?}", single_traced_ref);
                        let mut single_new_ref = new_refs.clone();
                        for nr in &mut single_new_ref {
                            for tr in single_traced_ref {
                                nr.push(*tr);
                            }
                        }
                        for snf in single_new_ref {
                            new_new_refs.push(snf);
                        }
                    }
                    // println!("nnr: {:?}", new_new_refs);
                    new_refs = new_new_refs;
                } else {
                    let mut single_new_ref = new_refs.clone();
                    for nr in &mut single_new_ref {
                        nr.push(*num);
                    }
                    new_refs = single_new_ref;
                }
            }
            // println!("{:?}", new_refs);
            for n in new_refs {
                all_new_refs.push(n);
            }
        }
        refs = all_new_refs;
    }

    let mut rule_str_vec = Vec::new();
    // println!("refs: {:?}", refs);
    for rule in refs {
        let mut rule_str = String::from("");
        for num in rule {
            let r = rules.get(&num).unwrap();
            rule_str = format!("{}{}", rule_str, r.st.as_ref().unwrap()[0])
        }
        rule_str_vec.push(rule_str);
    }
    // println!("rule_str_vec: {:?}", rule_str_vec);

    let mut num_valid = 0;
    for m in messages {
        println!("{}", m);
        if rule_str_vec.contains(&m) {
            num_valid += 1;
        }
    }
    println!("{}", num_valid);
}
