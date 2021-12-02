use std::fs;

fn part_one() {
    let contents = fs::read_to_string("assets/data.txt").expect("Something went wrong");
    let data:Vec<&str> = contents.split("\n").collect();
    let mut last_val: u32 = 0;
    let mut counter = 0;

    for (i, datum) in data.iter().enumerate() {
        let parse_datum:u32 = datum.trim().parse().unwrap();
        if i == 0 {
            last_val = parse_datum;
        }
        if parse_datum > last_val {
            println!("Increase");
            counter += 1;
        } else {
            println!("Same or lower");
        }
        last_val = parse_datum;
    }

    println!("Part 1: total increase: {}", counter);
}

fn part_two() {
    let contents = fs::read_to_string("assets/data.txt").expect("Something went wrong");
    let data:Vec<&str> = contents.split("\n").collect();
    let mut last_val: u32 = 
        data[0].trim().parse::<u32>().unwrap() +
        data[1].trim().parse::<u32>().unwrap() +
        data[2].trim().parse::<u32>().unwrap();
    let mut counter = 0;

    for (i, _) in data.iter().enumerate() {
        if i + 2 >= data.len() {
            break;
        }

        let new_val: u32 = 
            data[i].trim().parse::<u32>().unwrap() +
            data[i+1].trim().parse::<u32>().unwrap() +
            data[i+2].trim().parse::<u32>().unwrap();

        if new_val > last_val {
            println!("Increase");
            counter += 1;
        } else {
            println!("Same or lower");
        }
        last_val = new_val;
    }

    println!("Part 2: total increase: {}", counter);
}

fn main() {
    // part_one();
    part_two();
}