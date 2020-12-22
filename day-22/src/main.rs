use std::error::Error;
use std::fs;

fn main() {
    part_2();
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

fn part_2() {
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

    let winning_deck = play_game(player_1_deck, player_2_deck).1;

    let mut score = 0;
    for i in 0..winning_deck.len() {
        score += (i as u32 + 1) * winning_deck[winning_deck.len() - 1 - i];
    }
    println!("{}", score);
}

// Returns (is_player_1, winning deck)
fn play_game(mut player_1_deck: Vec<u32>, mut player_2_deck: Vec<u32>) -> (bool, Vec<u32>) {
    // println!("New game started: {:?} {:?}", player_1_deck, player_2_deck);
    let mut past_game_states = vec![(player_1_deck.clone(), player_2_deck.clone())];
    while player_1_deck.len() != 0 && player_2_deck.len() != 0 {
        // println!("Next round: {:?} {:?}", player_1_deck, player_2_deck);
        if player_1_deck.len() > player_1_deck[0] as usize
            && player_2_deck.len() > player_2_deck[0] as usize
        {
            // Do sub-game
            if play_game(
                player_1_deck[1..player_1_deck[0] as usize + 1]
                    .iter()
                    .cloned()
                    .collect(),
                player_2_deck[1..player_2_deck[0] as usize + 1]
                    .iter()
                    .cloned()
                    .collect(),
            )
            .0
            {
                player_1_deck.push(player_1_deck[0]);
                player_1_deck.push(player_2_deck[0]);
            } else {
                player_2_deck.push(player_2_deck[0]);
                player_2_deck.push(player_1_deck[0]);
            }
        } else {
            if player_1_deck[0] > player_2_deck[0] {
                player_1_deck.push(player_1_deck[0]);
                player_1_deck.push(player_2_deck[0]);
            } else {
                player_2_deck.push(player_2_deck[0]);
                player_2_deck.push(player_1_deck[0]);
            }
        }
        player_1_deck.remove(0);
        player_2_deck.remove(0);

        // Check infinite game condition
        if past_game_states.contains(&(player_1_deck.clone(), player_2_deck.clone())) {
            // Force player 1 win
            // println!("Player 1 wins (inifinite condition), {:?}", player_1_deck);
            return (true, player_1_deck);
        }
        past_game_states.push((player_1_deck.clone(), player_2_deck.clone()))
    }
    return if player_1_deck.len() > 0 {
        // println!("Player 1 wins, {:?}", player_1_deck);
        (true, player_1_deck)
    } else {
        // println!("Player 2 wins, {:?}", player_2_deck);
        (false, player_2_deck)
    };
}
