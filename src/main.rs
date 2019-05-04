use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("In file {}", filename);

    let contents = fs::read_to_string(&filename)
        .expect("Something went wrong reading the file");

    let trimmed = &contents.trim();

    // check trimmed
    //println!("Trimmed:\n{}", trimmed);

    let lines = trimmed.lines().clone();

    let mut splitted: Vec<Vec<i32>> = Vec::<Vec<i32>>::new();

    // if all value is number and filled
    for line in lines {
        let numbers: Vec<i32> = line
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();

        splitted.push(numbers);
    }

    for split in &splitted {
        println!("{:?}", split);
    }
}