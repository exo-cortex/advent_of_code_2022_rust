use std::fs::File;
use std::io::{BufRead, BufReader};

fn use_crate_mover_9000(stacks: &mut [Vec<char>], amount: &usize, from: &usize, to: &usize) {
    for _ in 0..*amount {
        let move_item: char = *stacks[from - 1].last().unwrap();
        stacks[from - 1].pop();
        stacks[to - 1].push(move_item);
    }
}

fn use_crate_mover_9001(stacks: &mut [Vec<char>], amount: &usize, from: &usize, to: &usize) {
    let from_length = stacks[*from - 1].len();

    let mut moving_items = Vec::from_iter(
        stacks[*from - 1][from_length - *amount..from_length]
            .iter()
            .cloned(),
    );
    stacks[*from - 1].truncate(from_length - *amount);
    stacks[*to - 1].append(&mut moving_items);
}

fn main() {
    let file = File::open("./example.txt").unwrap();
    let reader = BufReader::new(file);

    let mut stacks: Vec<Vec<char>> = vec![Vec::with_capacity(64); 9];
    let mut it = reader.lines().map(|l| l.unwrap());

    for line in (&mut it).take_while(|l| l.len() > 0) {
        let line = line.chars().collect::<Vec<char>>();
        for (ch, stack) in line.chunks(4).zip(&mut stacks) {
            if let ('[', c, ']') = (ch[0], ch[1], ch[2]) {
                stack.push(c)
            }
        }
    }

    stacks.iter_mut().for_each(|s| s.reverse());

    for line in it {
        let line = line.split(' ').collect::<Vec<&str>>();
        let (amount, from, to) = (
            line[1].parse::<usize>().unwrap(),
            line[3].parse::<usize>().unwrap(),
            line[5].parse::<usize>().unwrap(),
        );
        use_crate_mover_9001(&mut stacks, &amount, &from, &to);
    }

    for s in stacks.iter() {
        print!("{}", s.last().unwrap());
    }
    println!();
}
