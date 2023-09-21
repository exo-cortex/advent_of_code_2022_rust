use std::fs::File;
use std::io::{BufRead, BufReader};

fn make_range(something: &str) -> Vec<u32> {
    something
        .split("-")
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

fn main() {
    let file = File::open("./example_data.txt").unwrap();
    let reader = BufReader::new(&file);

    let mut count_full_contain = 0;
    let mut count_overlap = 0;

    for line in reader.lines().map(|l| l.unwrap()) {
        let num = line
            .split(&['-', ','])
            .into_iter()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        if (num[0] <= num[2] && num[1] >= num[3]) || (num[0] >= num[2] && num[1] <= num[3]) {
            count_full_contain += 1;
            println!("{}, {}, {}, {}", num[0], num[2], num[1], num[3]);
        }

        if (num[1] >= num[2] && num[2] >= num[0]) || (num[0] <= num[3] && num[0] >= num[2]) {
            count_overlap += 1;
            println!("{}..{}, {}..{}", num[0], num[1], num[2], num[3]);
        }
    }

    println!("{} times one range contains the other", count_full_contain);
    println!("{} times one range overlaps the other", count_overlap);
}
