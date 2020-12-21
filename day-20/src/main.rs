use regex::Regex;
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
    matching_from: usize,
    matching_to_id: u32,
    matching_to: usize,
    matching_to_flipped: bool,
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
                    let matching_to = edges2.iter().position(|r| r == &edges[i]).unwrap();
                    matching_edges
                        .entry(tile.id)
                        .or_insert_with(|| Vec::new())
                        .push(MatchingEdge {
                            matching_from: i,
                            matching_to_id: tile2.id,
                            matching_to: matching_to % 4,
                            matching_to_flipped: matching_to >= 4,
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

    let mut image_layout: Vec<Vec<(u32, usize, bool)>> = Vec::new();
    let mut starting_directions = matching_edges
        .get(&corners[0])
        .unwrap()
        .iter()
        .map(|e| e.matching_from)
        .collect::<Vec<usize>>();
    starting_directions.sort();
    let starting_up = (starting_directions[0] + 3) % 4;

    image_layout.push(vec![(corners[0], starting_up, false)]);

    let mut next_below = matching_edges
        .get(&corners[0])
        .unwrap()
        .iter()
        .filter(|e| {
            e.matching_from
                == if starting_directions[1] == 3 {
                    0
                } else {
                    starting_directions[1]
                }
        })
        .next();

    let mut current_is_flipped = false;
    while next_below.is_some() {
        let current_edge = next_below.unwrap();
        image_layout.push(vec![(
            current_edge.matching_to_id,
            current_edge.matching_to,
            current_is_flipped && current_edge.matching_to_flipped,
        )]);
        current_is_flipped = current_edge.matching_to_flipped;
        let new_match = (current_edge.matching_to + 2) % 4;
        next_below = matching_edges
            .get(&current_edge.matching_to_id)
            .unwrap()
            .iter()
            .filter(|e| e.matching_from == new_match)
            .next();
    }

    for r in 0..image_layout.len() {
        let mut next_to_right = matching_edges
            .get(&image_layout[r][0].0)
            .unwrap()
            .iter()
            .filter(|e| {
                if r > 0 && r < image_layout.len() - 1 {
                    e.matching_to_id != image_layout[r - 1][0].0
                        && e.matching_to_id != image_layout[r + 1][0].0
                } else if r > 0 {
                    e.matching_to_id != image_layout[r - 1][0].0
                } else {
                    e.matching_to_id != image_layout[r + 1][0].0
                }
            })
            .next();
        let mut current_is_flipped = false;
        while next_to_right.is_some() {
            let current_edge = next_to_right.unwrap();
            image_layout[r].push((
                current_edge.matching_to_id,
                (current_edge.matching_to + 1) % 4,
                current_is_flipped && current_edge.matching_to_flipped,
            ));
            current_is_flipped = current_edge.matching_to_flipped;
            let new_match = (current_edge.matching_to + 2) % 4;
            next_to_right = matching_edges
                .get(&current_edge.matching_to_id)
                .unwrap()
                .iter()
                .filter(|e| e.matching_from == new_match)
                .next();
        }
    }

    for row in &image_layout {
        println!("{:?}", row);
    }

    let chunk_height = tiles_vec[0].top.len() - 2;
    let mut image_grid: Vec<Vec<char>> = vec![Vec::new(); chunk_height * image_layout.len()];
    for row in 0..image_layout.len() {
        for col in 0..image_layout[row].len() {
            let tile_id = image_layout[row][col].0;
            let mut tile_top = image_layout[row][col].1;
            let tile_is_flipped = image_layout[row][col].2;
            let tile = tiles_vec.iter().filter(|t| t.id == tile_id).next().unwrap();
            let mut tile_without_edge = tile.all[1..tile.all.len() - 1]
                .iter()
                .map(|l| l[1..l.len() - 1].iter().cloned().collect())
                .collect::<Vec<Vec<char>>>();
            if tile_is_flipped {
                tile_without_edge = tile_without_edge
                    .iter()
                    .map(|row| row.clone().into_iter().rev().collect())
                    .collect();
                // if tile_top % 2 == 1 {
                //     tile_top = (tile_top + 2) % 4
                // }
            }
            for y in 0..tile_without_edge.len() {
                let to_extend = match tile_top {
                    0 => tile_without_edge[y].clone(),
                    1 => tile_without_edge
                        .clone()
                        .iter()
                        .map(|r| r[chunk_height - 1 - y])
                        .collect::<Vec<char>>(),
                    2 => tile_without_edge[chunk_height - 1 - y]
                        .clone()
                        .into_iter()
                        .rev()
                        .collect::<Vec<char>>(),
                    3 => tile_without_edge
                        .clone()
                        .iter()
                        .map(|r| r[y])
                        .rev()
                        .collect::<Vec<char>>(),
                    _ => panic!("Unknown tile top!"),
                };
                image_grid[row * chunk_height + y].extend(to_extend)
            }
        }
    }

    // Search for sea monsters
    let mut found_sea_monsters = false;
    for row in &image_grid {
        for col in row {
            print!("{}", &col);
        }
        println!("")
    }

    let sea_monster_regex = (
        Regex::new(r"^..................#.").unwrap(),
        Regex::new(r"^#....##....##....###").unwrap(),
        Regex::new(r"^.#..#..#..#..#..#...").unwrap(),
    );
    let sea_monster_regex_initial = Regex::new(r"..................#.").unwrap();
    let mut sea_monster_count = 0;

    let chunk_height = image_grid.len();
    let mut tile_top = 0;
    let image_grid_original = image_grid.clone();
    while !found_sea_monsters {
        let mut image_grid = image_grid_original.clone();
        for y in 0..image_grid_original.len() {
            let to_extend = match tile_top {
                0 => image_grid_original[y].clone(),
                1 => image_grid_original
                    .clone()
                    .iter()
                    .map(|r| r[chunk_height - 1 - y])
                    .collect::<Vec<char>>(),
                2 => image_grid_original[chunk_height - 1 - y]
                    .clone()
                    .into_iter()
                    .rev()
                    .collect::<Vec<char>>(),
                3 => image_grid_original
                    .clone()
                    .iter()
                    .map(|r| r[y])
                    .rev()
                    .collect::<Vec<char>>(),
                _ => panic!("Too far"),
            };
            image_grid[y] = to_extend;
        }
        for r in 0..image_grid.len() - 2 {
            for c in 0..image_grid[r].len() - 20 {
                let row_0 = &image_grid[r][c..image_grid[r].len()];
                let row_1 = &image_grid[r + 1][c..image_grid[r].len()];
                let row_2 = &image_grid[r + 2][c..image_grid[r].len()];
                let row_string_0 = row_0.into_iter().collect::<String>();
                let row_string_1 = row_1.into_iter().collect::<String>();
                let row_string_2 = row_2.into_iter().collect::<String>();
                let mat = sea_monster_regex_initial.find(&row_string_0);
                if mat.is_some() {
                    let start_pos = mat.unwrap().start();
                    // println!("{}: {}", r, start_pos);
                    if sea_monster_regex.1.is_match_at(&row_string_1, start_pos)
                        && sea_monster_regex.2.is_match_at(&row_string_2, start_pos)
                    {
                        sea_monster_count += 1;
                        println!("yes");
                        found_sea_monsters = true;
                    }
                }
            }
        }
        tile_top += 1;
    }
    println!("");
    println!("{}", sea_monster_count);

    let mut total_hashes = 0;
    for row in &image_grid {
        for col in row {
            if *col == '#' {
                total_hashes += 1;
            }
        }
    }
    println!("{}", total_hashes - 15 * sea_monster_count)
}
