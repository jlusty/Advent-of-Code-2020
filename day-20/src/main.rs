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
