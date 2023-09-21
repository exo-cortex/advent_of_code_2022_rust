use std::fs::File;
use std::io::{BufRead, BufReader};

fn score_a(left: &char, right: &char) -> u32 {
    match (left, right) {
        ('A', 'X') => 3 + 1,
        ('A', 'Y') => 6 + 2,
        ('A', 'Z') => 0 + 3,
        ('B', 'X') => 0 + 1,
        ('B', 'Y') => 3 + 2,
        ('B', 'Z') => 6 + 3,
        ('C', 'X') => 6 + 1,
        ('C', 'Y') => 0 + 2,
        ('C', 'Z') => 3 + 3,
        (_, _) => 0,
    }
}

fn score_b(left: &char, right: &char) -> u32 {
    match (left, right) {
        ('A', 'X') => 0 + 3,
        ('A', 'Y') => 3 + 1,
        ('A', 'Z') => 6 + 2,
        ('B', 'X') => 0 + 1,
        ('B', 'Y') => 3 + 2,
        ('B', 'Z') => 6 + 3,
        ('C', 'X') => 0 + 2,
        ('C', 'Y') => 3 + 3,
        ('C', 'Z') => 6 + 1,
        (_, _) => 0,
    }
}

fn main() {
    let file = File::open("./strategy_guide.txt").unwrap();
    let reader = BufReader::new(file);

    let mut sum_1 = 0;
    let mut sum_2 = 0;
    for line in reader.lines() {
        let chars = line.unwrap().chars().collect::<Vec<char>>();
        sum_1 += score_a(&chars[0], &chars[2]);
        sum_2 += score_b(&chars[0], &chars[2]);
    }

    println!("day 02, task #1: score is {}", sum_1);
    println!("day 02, task #2: score is {}", sum_2);
}
