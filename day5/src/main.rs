use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::iter::repeat;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    // println!("Input file {}", file_path);

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("part1 {}", part1(parse_input(&input)));
    // println!("part2 {}", part2(&input))
}

fn part1(
    (mut crates, instructions): (HashMap<usize, Vec<char>>, Vec<(usize, usize, usize)>),
) -> String {
    for (amount, from, to) in instructions {
        repeat(0).take(amount).for_each(|_| {
            let c = crates.get_mut(&from).unwrap().pop().unwrap();
            // println!("move {} from {} to {} ({})", amount, from, to, c);
            crates.get_mut(&to).unwrap().push(c);
        });
        // println!("crates {:?}", crates);
    }

    (1..=crates.len())
        .map(|i| crates.get_mut(&i).unwrap().pop().unwrap())
        .collect()
}

fn parse_input(input: &str) -> (HashMap<usize, Vec<char>>, Vec<(usize, usize, usize)>) {
    let (crates_source, instructions_source) = input.split_once("\n\n").unwrap();

    let mut crates = HashMap::<usize, Vec<char>>::new();

    crates_source
        .lines()
        .rev()
        .filter(|line| !line.contains('1'))
        .for_each(|line| {
            let crate_letters: &Vec<char> = &line[1..].chars().step_by(4).collect();
            for (i, c) in crate_letters.iter().enumerate() {
                if *c != ' ' {
                    // println!("{}, {}", i + 1, c);
                    match crates.get_mut(&(i + 1)) {
                        Some(value) => value.push(*c),
                        _ => {
                            crates.insert(i + 1, vec![*c]);
                        }
                    };
                }
            }
        });

    // println!("{:?}", crates);

    let regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let instructions: Vec<(usize, usize, usize)> = instructions_source
        .lines()
        .map(|line| {
            let captures = regex.captures(line).unwrap();
            (
                captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                captures.get(3).unwrap().as_str().parse::<usize>().unwrap(),
            )
        })
        .collect();

    // println!("{:?}", instructions);

    (crates, instructions)
}

// fn part2(input: &str) -> usize {
//     0
// }
