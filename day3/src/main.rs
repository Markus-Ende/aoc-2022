use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    // println!("Input file {}", file_path);

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("part1 {}", part1(&input));
}

fn part1(input: &str) -> usize {
    let lines = input.lines();
    lines
        .map(|line| {
            let left = &line[..line.len() / 2];
            let right = &line[line.len() / 2..];
            let duplicate_index =
                right.find(|char| left.find(|leftchar| leftchar == char).is_some());

            match duplicate_index {
                Some(index) => {
                    // println!("found {}", right.chars().nth(index).unwrap());
                    " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
                        .find(right.chars().nth(index).unwrap())
                        .unwrap()
                }
                _ => panic!("has no duplicate char: {}", line),
            }
        })
        .sum()
}
