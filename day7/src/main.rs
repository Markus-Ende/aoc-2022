use std::{collections::HashMap, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    // println!("Input file {}", file_path);

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("part1 {}", part1(&input));
    println!("part2 {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let mut dir_sizes = HashMap::<String, usize>::new();
    let mut current_dir = Vec::<String>::new();
    let mut lines = input.lines();
    loop {
        match lines.next() {
            None => break,
            Some(line) => match line {
                "$ cd .." => {
                    current_dir.pop();
                }
                "$ ls" => {}
                l if l.starts_with("dir") => {}
                l if l.starts_with("$ cd ") => {
                    let dir_name = l.split_once("$ cd ").unwrap().1;
                    let absolute_dir = if dir_name == "/" {String::from("/")} else {  String::from([current_dir.last().unwrap(), dir_name].join("/"))};
                    current_dir.push(absolute_dir.clone());
                    dir_sizes.insert(absolute_dir, 0);
                }

                l /* size filename */ => {
                    let size = l.split_once(" ").unwrap().0.parse::<usize>().unwrap();
                    for dir in &current_dir[..] {
                        *dir_sizes.get_mut(dir).unwrap() += size;
                    }
                }
            },
        }
        // println!("current dirs: {:?}", current_dir);
    }
    // println!("sizes: {:?}", dir_sizes);
    dir_sizes.values().filter(|size| **size <= 100000).sum()
}

fn part2(input: &str) -> usize {
    0
}
