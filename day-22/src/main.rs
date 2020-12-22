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

    let mut player_1_deck = Vec::<u32>::new();
    let mut player_2_deck = Vec::<u32>::new();
    let mut getting_player_1_deck = true;
    for line in input.lines() {
        if line == "" {
            getting_player_1_deck = false;
            continue;
        }
        if line.len() > 6 && &line[0..6] == "Player" {
            continue;
        }
        if getting_player_1_deck {
            player_1_deck.push(line.parse().unwrap());
        } else {
            player_2_deck.push(line.parse().unwrap());
        }
    }

    while player_1_deck.len() != 0 && player_2_deck.len() != 0 {
        if player_1_deck[0] > player_2_deck[0] {
            player_1_deck.push(player_1_deck[0]);
            player_1_deck.push(player_2_deck[0]);
        } else {
            player_2_deck.push(player_2_deck[0]);
            player_2_deck.push(player_1_deck[0]);
        }
        player_1_deck.remove(0);
        player_2_deck.remove(0);
    }

    let winning_deck = if player_1_deck.len() == 0 {
        player_2_deck
    } else {
        player_1_deck
    };
    let mut score = 0;
    for i in 0..winning_deck.len() {
        score += (i as u32 + 1) * winning_deck[winning_deck.len() - 1 - i];
    }
    println!("{}", score);
}
