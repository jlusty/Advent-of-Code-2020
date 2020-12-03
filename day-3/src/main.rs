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

    let mut tree_vecs = Vec::<Vec<char>>::new();
    let mut y = 0;
    for line in input.lines() {
        tree_vecs.push(Vec::new());
        for c in line.chars() {
            tree_vecs[y].push(c);
        }
        y += 1;
    }

    let map_width = tree_vecs[0].len();
    let mut num_trees = 0;
    for y in 0..tree_vecs.len() {
        if tree_vecs[y][(y * 3) % map_width] == '#' {
            num_trees += 1;
        }
    }

    println!("{}", num_trees);
}

fn part_2() {
    let input = read_file("./input.txt".to_string()).unwrap();

    let mut tree_vecs = Vec::<Vec<char>>::new();
    let mut y = 0;
    for line in input.lines() {
        tree_vecs.push(Vec::new());
        for c in line.chars() {
            tree_vecs[y].push(c);
        }
        y += 1;
    }

    let map_width = tree_vecs[0].len();
    let wanted_slopes = vec![1, 3, 5, 7];
    let mut tree_vec = Vec::<usize>::new();
    for mov in wanted_slopes {
        let mut num_trees = 0;
        for y in 0..tree_vecs.len() {
            if tree_vecs[y][(y * mov) % map_width] == '#' {
                num_trees += 1;
            }
        }
        tree_vec.push(num_trees);
    }

    // Down 2
    let mut num_trees = 0;
    for y in 0..tree_vecs.len() {
        if y % 2 == 0 {
            if tree_vecs[y][(y / 2) % map_width] == '#' {
                num_trees += 1;
            }
        }
    }
    tree_vec.push(num_trees);

    let mut ans = 1;
    for n in tree_vec {
        ans *= n;
    }

    println!("{}", ans);
}
