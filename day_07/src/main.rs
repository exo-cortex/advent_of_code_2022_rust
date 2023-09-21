use std::{
    fs::File,
    io::{BufRead, BufReader},
    slice::Chunks,
};

fn main() {
    let file = File::open("./example.txt").unwrap();
    let reader = BufReader::new(file);

    let mut dir_tree = Node::Directory(Vec::new());

    let mut indent = 0;

    let lines: Vec<Vec<char>> = reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    // println!("{:?}", lines);

    let len = lines.len();
    for i in 0..lines.len() {
        println!("{:?}", lines[len - i - 1])
    }
    // for (i, line) in reader.lines().enumerate() {
    //     let mut previous_indent = indent;

    //     let indent = line
    //         .unwrap()
    //         .chars()
    //         .collect::<Vec<char>>()
    //         .chunks(2)
    //         .position(|c| (c[0], c[1]) != (' ', ' '))
    //         .unwrap_or(0);

    //     if previous_indent < indent {
    //         // belongs to parent folder
    //     }

    //     println!("{indent}");
    //     // println!("{}", &indent);
    // }

    // ++++++++++++++++

    dir_tree.insert(Node::File(10));
    dir_tree.insert(Node::Directory(Vec::new()));
    if let Some(children) = dir_tree.get_children() {
        children.push(Node::Directory(Vec::new()));
    }
}

enum Node {
    Directory(Vec<Node>),
    File(usize),
}

impl Node {
    pub fn new_root() -> Self {
        Node::Directory(Vec::new())
    }

    pub fn insert(&mut self, new_node: Node) {
        if let Node::Directory(children) = self {
            children.push(new_node);
        }
    }

    pub fn get_size(&self) -> usize {
        match self {
            Node::Directory(children) => children.into_iter().fold(0, |acc, c| acc + c.get_size()),
            Node::File(filesize) => *filesize,
        }
    }

    pub fn get_children(&mut self) -> Option<&mut Vec<Node>> {
        match self {
            Node::Directory(children) => Some(children),
            Node::File(_) => None,
        }
    }
}
