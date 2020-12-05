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

    let mut max_id = 0;
    for line in input.lines() {
        let row = &line[0..7];
        let col = &line[7..10];
        let row2 = row.replace("F", "0").replace("B", "1");
        let col2 = col.replace("L", "0").replace("R", "1");
        let row_int = u32::from_str_radix(&row2, 2).unwrap();
        let col_int = u32::from_str_radix(&col2, 2).unwrap();
        let seat_id = (row_int * 8) + col_int;
        if seat_id > max_id {
            max_id = seat_id;
        }
    }

    println!("{}", max_id);
}

fn part_2() {
    let input = read_file("./input.txt".to_string()).unwrap();

    let mut seat_ids = Vec::<u32>::new();
    for line in input.lines() {
        let row = &line[0..7];
        let col = &line[7..10];
        let row2 = row.replace("F", "0").replace("B", "1");
        let col2 = col.replace("L", "0").replace("R", "1");
        let row_int = u32::from_str_radix(&row2, 2).unwrap();
        let col_int = u32::from_str_radix(&col2, 2).unwrap();
        let seat_id = (row_int * 8) + col_int;
        seat_ids.push(seat_id);
    }

    seat_ids.sort();

    let mut ans = 0;
    for i in 1..seat_ids.len() - 1 {
        if !(seat_ids[i - 1] + 1 == seat_ids[i]) {
            ans = seat_ids[i] - 1;
        }
    }
    println!("{}", ans);
}
