use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    // very ugly!

    let path = Path::new("./elves_calories.txt");
    let file = File::open(&path).unwrap();

    let reader = BufReader::new(file);

    let mut calories_per_thing: Vec<u32> = vec![0];

    for line in reader.lines() {
        let line = line.unwrap();
        if line == "" {
            calories_per_thing.push(0);
        } else {
            *calories_per_thing.last_mut().unwrap() += line.parse::<u32>().unwrap();
        }
    }

    // calories_per_thing.sort_by(|a, b| a.partial_cmp(b).unwrap());
    calories_per_thing.sort();
    let largest_three_total = calories_per_thing.iter().rev().take(3).sum::<u32>();
    print!("largest 3 total = {}", largest_three_total);

    let max = calories_per_thing.iter().max().unwrap();
    println!("\nmax: {}", &max);
}
