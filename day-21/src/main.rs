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

    let mut allergens_poss_map = HashMap::<String, Vec<String>>::new();
    let mut all_ingredients = Vec::<String>::new();
    for line in input.lines() {
        let mut ingredients_vec = Vec::<String>::new();
        let mut allergens_vec = Vec::<String>::new();
        let mut adding_ingredients = true;
        for word in line.split(" ") {
            if word == "(contains" {
                adding_ingredients = false;
                continue;
            }
            if adding_ingredients {
                ingredients_vec.push(word.to_string());
                all_ingredients.push(word.to_string());
            } else {
                allergens_vec.push(word[0..word.len() - 1].to_string());
            }
        }

        for allerg in allergens_vec {
            if allergens_poss_map.get(&allerg).is_some() {
                let possible_ings = allergens_poss_map.get(&allerg).unwrap();
                let mut new_possible_ings = Vec::new();
                for ing in possible_ings {
                    if ingredients_vec.contains(ing) {
                        new_possible_ings.push(ing.to_string());
                    }
                }
                allergens_poss_map.insert(allerg, new_possible_ings);
            } else {
                allergens_poss_map.insert(allerg, ingredients_vec.clone());
            }
        }
    }

    let mut all_allerg_assigned = false;
    let mut assigned_ing = Vec::new();
    while !all_allerg_assigned {
        all_allerg_assigned = true;
        for ing_vec in allergens_poss_map.values() {
            if ing_vec.len() == 1 && !assigned_ing.contains(&ing_vec[0]) {
                assigned_ing.push(ing_vec[0].clone());
                all_allerg_assigned = false;
            }
        }
        // println!("{:?}", assigned_ing);

        for (allerg, ing_vec) in &allergens_poss_map.clone() {
            let mut new_ings = Vec::new();
            if ing_vec.len() == 1 {
                continue;
            }
            for ing in ing_vec {
                if !assigned_ing.contains(&ing) {
                    new_ings.push(ing.to_string());
                }
            }
            allergens_poss_map.insert(allerg.to_string(), new_ings);
        }
    }

    // println!("{:?}", allergens_poss_map);
    let mut allergens_map = HashMap::new();
    let mut ingredients_map = HashMap::new();
    for (allerg, ing_vec) in &allergens_poss_map {
        allergens_map.insert(allerg, ing_vec[0].clone());
        ingredients_map.insert(ing_vec[0].clone(), allerg);
    }
    // println!("{:?}", allergens_map);
    // println!("{:?}", all_ingredients);

    let mut no_allergy_ings_count = 0;
    for ing in all_ingredients {
        if ingredients_map.get(&ing).is_none() {
            no_allergy_ings_count += 1;
        }
    }
    println!("{}", no_allergy_ings_count);
}

fn part_2() {
    let input = read_file("./input.txt".to_string()).unwrap();

    let mut allergens_poss_map = HashMap::<String, Vec<String>>::new();
    let mut all_ingredients = Vec::<String>::new();
    for line in input.lines() {
        let mut ingredients_vec = Vec::<String>::new();
        let mut allergens_vec = Vec::<String>::new();
        let mut adding_ingredients = true;
        for word in line.split(" ") {
            if word == "(contains" {
                adding_ingredients = false;
                continue;
            }
            if adding_ingredients {
                ingredients_vec.push(word.to_string());
                all_ingredients.push(word.to_string());
            } else {
                allergens_vec.push(word[0..word.len() - 1].to_string());
            }
        }

        for allerg in allergens_vec {
            if allergens_poss_map.get(&allerg).is_some() {
                let possible_ings = allergens_poss_map.get(&allerg).unwrap();
                let mut new_possible_ings = Vec::new();
                for ing in possible_ings {
                    if ingredients_vec.contains(ing) {
                        new_possible_ings.push(ing.to_string());
                    }
                }
                allergens_poss_map.insert(allerg, new_possible_ings);
            } else {
                allergens_poss_map.insert(allerg, ingredients_vec.clone());
            }
        }
    }

    let mut all_allerg_assigned = false;
    let mut assigned_ing = Vec::new();
    while !all_allerg_assigned {
        all_allerg_assigned = true;
        for ing_vec in allergens_poss_map.values() {
            if ing_vec.len() == 1 && !assigned_ing.contains(&ing_vec[0]) {
                assigned_ing.push(ing_vec[0].clone());
                all_allerg_assigned = false;
            }
        }

        for (allerg, ing_vec) in &allergens_poss_map.clone() {
            let mut new_ings = Vec::new();
            if ing_vec.len() == 1 {
                continue;
            }
            for ing in ing_vec {
                if !assigned_ing.contains(&ing) {
                    new_ings.push(ing.to_string());
                }
            }
            allergens_poss_map.insert(allerg.to_string(), new_ings);
        }
    }

    let mut allergens_vec = Vec::new();
    for (allerg, ing_vec) in &allergens_poss_map {
        allergens_vec.push((allerg, ing_vec[0].clone()));
    }

    allergens_vec.sort_by_key(|a| a.0);
    // println!("{:?}", allergens_vec);

    let ing_list = allergens_vec
        .iter()
        .map(|(_, i)| i.to_string())
        .collect::<Vec<String>>();

    println!("{}", ing_list.join(","));
}
