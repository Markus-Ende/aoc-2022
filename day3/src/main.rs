use itertools::Itertools;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    // println!("Input file {}", file_path);

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("part1 {}", part1(&input));
    println!("part2 {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let lines = input.lines();
    lines
        .map(|line| {
            let (left, right) = compartments(line);
            let duplicate_index =
                right.find(|char| left.find(|leftchar| leftchar == char).is_some());

            match duplicate_index {
                Some(index) => priority(right.chars().nth(index).unwrap()),
                _ => panic!("has no duplicate char: {}", line),
            }
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let lines = input.lines();
    lines
        .map(|rucksack| rucksack.chars().unique().join(""))
        .tuples::<(_, _, _)>()
        .map(|rucksacks| {
            // TODO: find a better way to map the items to avoid joining multiple times
            let joined_rucksacks = [rucksacks.0, rucksacks.1, rucksacks.2].into_iter().join("");
            let x = joined_rucksacks.chars().find(|char| {
                joined_rucksacks
                    .chars()
                    .filter(|c| char == c)
                    .join("")
                    .len()
                    == 3
            });
            match x {
                Some(char) => priority(char),
                _ => panic!("has no char in common: {}", joined_rucksacks),
            }
        })
        .sum()
}

fn compartments(rucksack: &str) -> (&str, &str) {
    (
        &rucksack[..rucksack.len() / 2],
        &rucksack[rucksack.len() / 2..],
    )
}

fn priority(char: char) -> usize {
    " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .find(char)
        .unwrap()
}
