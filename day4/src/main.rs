use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    // println!("Input file {}", file_path);

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("part1 {}", part1(&input));
    println!("part2 {}", part2(&input))
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (left, right) = parse_line(line);
            // println!("{}: {:?},{:?}", line, left, right);
            (left.is_subset(&right) || left.is_superset(&right)) as usize // bool is guaranteed to be 0 or 1: https://doc.rust-lang.org/std/primitive.bool.html
        })
        .sum()
}

fn parse_line(line: &str) -> (HashSet<i32>, HashSet<i32>) {
    let (left, right) = line.split_once(",").unwrap();

    let left_sections = parse_assignment(left);
    let right_sections = parse_assignment(right);
    (left_sections, right_sections)
}

fn parse_assignment(assignment: &str) -> HashSet<i32> {
    let (from, to) = assignment.split_once("-").unwrap();
    (from.parse::<i32>().unwrap()..to.parse::<i32>().unwrap() + 1).collect()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (left, right) = parse_line(line);
            // println!("{}: {:?},{:?}", line, left, right);
            (left.intersection(&right).count() > 0) as usize // bool is guaranteed to be 0 or 1: https://doc.rust-lang.org/std/primitive.bool.html
        })
        .sum()
}
