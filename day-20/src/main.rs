use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn main() {
    part_2();
}

fn read_file(filename: String) -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string(filename)?;
    Ok(input)
}

#[derive(Debug)]
struct Tile {
    all: Vec<Vec<char>>,
    top: Vec<char>,
    bottom: Vec<char>,
    left: Vec<char>,
    right: Vec<char>,
    id: u32,
}

fn part_1() {
    let input = read_file("./input.txt".to_string()).unwrap();

    let mut tiles_vec = Vec::<Tile>::new();
    let mut tile_all = Vec::new();
    let mut tile_id: u32 = 0;
    for line in input.lines() {
        if line == "" {
            tiles_vec.push(Tile {
                all: tile_all.clone(),
                top: tile_all[0].clone(),
                bottom: tile_all[tile_all.len() - 1].clone(),
                left: tile_all.iter().map(|row| row[0]).collect::<Vec<char>>(),
                right: tile_all
                    .iter()
                    .map(|row| row[row.len() - 1])
                    .collect::<Vec<char>>(),
                id: tile_id,
            });
            tile_all = Vec::new();
        } else if &line[0..4] == "Tile" {
            let tile_id_str = line.split(' ').nth(1).unwrap();
            tile_id = tile_id_str[0..tile_id_str.len() - 1].parse().unwrap();
        } else {
            tile_all.push(line.chars().collect::<Vec<char>>())
        }
    }
    // Need to include bottom tile in input
    tiles_vec.push(Tile {
        all: tile_all.clone(),
        top: tile_all[0].clone(),
        bottom: tile_all[tile_all.len() - 1].clone(),
        left: tile_all.iter().map(|row| row[0]).collect::<Vec<char>>(),
        right: tile_all
            .iter()
            .map(|row| row[row.len() - 1])
            .collect::<Vec<char>>(),
        id: tile_id,
    });

    let mut matching_edges = Vec::new();
    let mut matching_edges_count = Vec::new();
    for tile_idx in 0..tiles_vec.len() {
        let tile = &tiles_vec[tile_idx];
        let edges: Vec<Vec<&char>> = vec![
            tile.top.iter().collect(),
            tile.right.iter().collect(),
            tile.bottom.iter().collect(),
            tile.left.iter().collect(),
            tile.top.iter().rev().collect(),
            tile.right.iter().rev().collect(),
            tile.bottom.iter().rev().collect(),
            tile.left.iter().rev().collect(),
        ];
        let mut num_matching_edges = 0;
        for tile_idx2 in 0..tiles_vec.len() {
            let tile2 = &tiles_vec[tile_idx2];
            if tile.id == tile2.id {
                continue;
            }
            let edges2: Vec<Vec<&char>> = vec![
                tile2.top.iter().collect(),
                tile2.right.iter().collect(),
                tile2.bottom.iter().collect(),
                tile2.left.iter().collect(),
                tile2.top.iter().rev().collect(),
                tile2.right.iter().rev().collect(),
                tile2.bottom.iter().rev().collect(),
                tile2.left.iter().rev().collect(),
            ];
            for i in 0..edges.len() {
                if edges2.contains(&edges[i]) {
                    matching_edges.push((
                        tile.id,
                        i,
                        tile2.id,
                        edges2.iter().position(|r| r == &edges[i]).unwrap(),
                    ));
                    num_matching_edges += 1;
                }
            }
        }
        matching_edges_count.push((tile.id, num_matching_edges / 2))
    }

    let corners: u64 = matching_edges_count
        .iter()
        .filter(|(_, num)| *num == 2)
        .map(|(id, _)| *id as u64)
        .product();
    println!("{}", corners);
}

#[derive(Debug)]
struct MatchingEdge {
    id: u32,
    matching_from: usize,
    matching_to: usize,
}

fn part_2() {
    let input = read_file("./input.txt".to_string()).unwrap();

    let mut tiles_vec = Vec::<Tile>::new();
    let mut tile_all = Vec::new();
    let mut tile_id: u32 = 0;
    for line in input.lines() {
        if line == "" {
            tiles_vec.push(Tile {
                all: tile_all.clone(),
                top: tile_all[0].clone(),
                bottom: tile_all[tile_all.len() - 1].clone(),
                left: tile_all.iter().map(|row| row[0]).collect::<Vec<char>>(),
                right: tile_all
                    .iter()
                    .map(|row| row[row.len() - 1])
                    .collect::<Vec<char>>(),
                id: tile_id,
            });
            tile_all = Vec::new();
        } else if &line[0..4] == "Tile" {
            let tile_id_str = line.split(' ').nth(1).unwrap();
            tile_id = tile_id_str[0..tile_id_str.len() - 1].parse().unwrap();
        } else {
            tile_all.push(line.chars().collect::<Vec<char>>())
        }
    }
    // Need to include bottom tile in input
    tiles_vec.push(Tile {
        all: tile_all.clone(),
        top: tile_all[0].clone(),
        bottom: tile_all[tile_all.len() - 1].clone(),
        left: tile_all.iter().map(|row| row[0]).collect::<Vec<char>>(),
        right: tile_all
            .iter()
            .map(|row| row[row.len() - 1])
            .collect::<Vec<char>>(),
        id: tile_id,
    });

    let mut matching_edges = HashMap::<u32, Vec<MatchingEdge>>::new();
    let mut matching_edges_count = Vec::new();
    for tile_idx in 0..tiles_vec.len() {
        let tile = &tiles_vec[tile_idx];
        let edges: Vec<Vec<&char>> = vec![
            tile.top.iter().collect(),
            tile.right.iter().collect(),
            tile.bottom.iter().collect(),
            tile.left.iter().collect(),
        ];
        let mut num_matching_edges = 0;
        for tile_idx2 in 0..tiles_vec.len() {
            let tile2 = &tiles_vec[tile_idx2];
            if tile.id == tile2.id {
                continue;
            }
            let edges2: Vec<Vec<&char>> = vec![
                tile2.top.iter().collect(),
                tile2.right.iter().collect(),
                tile2.bottom.iter().collect(),
                tile2.left.iter().collect(),
                tile2.top.iter().rev().collect(),
                tile2.right.iter().rev().collect(),
                tile2.bottom.iter().rev().collect(),
                tile2.left.iter().rev().collect(),
            ];
            for i in 0..edges.len() {
                if edges2.contains(&edges[i]) {
                    matching_edges
                        .entry(tile.id)
                        .or_insert_with(|| Vec::new())
                        .push(MatchingEdge {
                            id: tile2.id,
                            matching_from: i,
                            matching_to: edges2.iter().position(|r| r == &edges[i]).unwrap(),
                        });
                    num_matching_edges += 1;
                }
            }
        }
        matching_edges_count.push((tile.id, num_matching_edges))
    }
    // println!("{:?}", matching_edges);
    let corners = matching_edges_count
        .iter()
        .filter(|(_, num)| *num == 2)
        .map(|(id, _)| *id)
        .collect::<Vec<u32>>();

    // println!("{:?}", corners);

    let mut image_layout: Vec<Vec<u32>> = Vec::new();
    image_layout.push(vec![corners[0]]);

    let starting_directions = matching_edges
        .get(&corners[0])
        .unwrap()
        .iter()
        .map(|e| e.matching_from)
        .collect::<Vec<usize>>();

    let mut next_below = matching_edges
        .get(&corners[0])
        .unwrap()
        .iter()
        .filter(|e| e.matching_from == starting_directions[0])
        .next();
    while next_below.is_some() {
        let current_edge = next_below.unwrap();
        image_layout.push(vec![current_edge.id]);
        let new_match = (current_edge.matching_to + 2) % 4;
        next_below = matching_edges
            .get(&current_edge.id)
            .unwrap()
            .iter()
            .filter(|e| e.matching_from == new_match)
            .next();
    }

    for r in 0..image_layout.len() {
        let mut next_to_right = matching_edges
            .get(&image_layout[r][0])
            .unwrap()
            .iter()
            .filter(|e| {
                if r > 0 && r < image_layout.len() - 1 {
                    e.id != image_layout[r - 1][0] && e.id != image_layout[r + 1][0]
                } else if r > 0 {
                    e.id != image_layout[r - 1][0]
                } else {
                    e.id != image_layout[r + 1][0]
                }
            })
            .next();
        println!("{:?}", matching_edges.get(&image_layout[r][0]));
        while next_to_right.is_some() {
            let current_edge = next_to_right.unwrap();
            image_layout[r].push(current_edge.id);
            // let new_match = if current_edge.matching_to < 4 {
            //     (current_edge.matching_to + 2) % 4
            // } else {
            //     ((current_edge.matching_to + 2) % 4) + 4
            // };
            let new_match = (current_edge.matching_to + 2) % 4;
            next_to_right = matching_edges
                .get(&current_edge.id)
                .unwrap()
                .iter()
                .filter(|e| e.matching_from == new_match)
                .next();
            // println!(
            //     "id: {} is {:?}, {}",
            //     current_edge.id,
            //     matching_edges.get(&current_edge.id),
            //     new_match
            // );
        }
    }

    for row in image_layout {
        println!("{:?}", row);
    }
}
