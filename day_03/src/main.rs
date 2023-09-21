use std::fs::File;
use std::io::{BufRead, BufReader};

fn score(letter: &char) -> u32 {
    match letter {
        'a'..='z' => 1 + *letter as u32 - 'a' as u32,
        'A'..='Z' => 27 + *letter as u32 - 'A' as u32,
        _ => 0,
    }
}

fn main() {
    let file = File::open("./example.txt").unwrap();
    let reader = BufReader::new(&file);

    let lines = reader.lines();

    // let backpacks: Vec<String> = lines
    //     .into_iter()
    //     .map(|l| format!("{}", &l.unwrap()))
    //     .collect();

    let backpacks: Vec<Vec<char>> = lines
        .into_iter()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    let mut sum_1 = 0;
    let mut sum_2 = 0;

    // find and score character that occurs in both halves of one line
    for cs in &backpacks {
        let length = cs.len();
        'backpack: for i in 0..length / 2 {
            for j in length / 2..length {
                if cs[i] == cs[j] {
                    sum_1 += score(&cs[i]);
                    break 'backpack;
                }
            }
        }
    }

    // find and score character that appears in three consecutive lines
    for cs2 in backpacks.chunks(3) {
        'outer: for c in &cs2[0] {
            for c2 in &cs2[1] {
                if c == c2 {
                    for c3 in &cs2[2] {
                        if c == c3 {
                            sum_2 += score(c);
                            break 'outer;
                        }
                    }
                }
            }
        }
    }

    println!("summed score part #1: {sum_1}");
    println!("summed score part #1: {sum_2}");
}
