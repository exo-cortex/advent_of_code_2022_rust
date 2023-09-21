use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

const START_OF_PACKAGE: usize = 4;
const START_OF_MESSAGE: usize = 14;

fn main() {
    let example_file = File::open("./example.txt").expect("couldn't open file");
    let data_file = File::open("./data.txt").expect("couldn't open file");

    // process examples
    let reader = BufReader::new(example_file);
    for (i, line) in reader.lines().into_iter().enumerate() {
        match i {
            0 | 1 | 2 | 3 => println!(
                "line {} package-marker appears at position {}.",
                i,
                index_after_marker(&line.unwrap(), START_OF_PACKAGE)
            ),
            _ => println!(
                "line {} message-marker appears at position {}.",
                i,
                index_after_marker(&line.unwrap(), START_OF_MESSAGE)
            ),
        }
    }

    // process actual data
    let reader = BufReader::new(data_file);
    for line in reader.lines() {
        println!(
            "in data transmission message-marker appears at position {}.",
            index_after_marker(&line.unwrap(), START_OF_MESSAGE)
        );
    }
}

fn index_after_marker(line: &str, sequence_length: usize) -> usize {
    if let Some((index, _)) = line
        .chars()
        .collect::<Vec<char>>()
        .windows(sequence_length)
        .find_position(|win| sequence_is_marker(win, sequence_length))
    {
        return index + sequence_length;
    }

    0
}

fn sequence_is_marker(characters: &[char], unique_characters: usize) -> bool {
    for j in 0..unique_characters - 1 {
        for i in j + 1..unique_characters {
            if characters[j] == characters[i] {
                return false;
            }
        }
    }
    true
}
