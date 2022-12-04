use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Input file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("part 1 {}", part1(&contents));
    println!("part 2 {}", part2(&contents));
}

fn part1(input: &str) -> i32 {
    let lines = input.split("\n");

    let mut current_max = 0;
    let mut current_acc = 0;
    // let mut elf = 1;

    for s in lines {
        if s.len() == 0 {
            if current_acc > current_max {
                current_max = current_acc;
            }
            // println!("elf {}: {} calories", elf, current_acc);
            current_acc = 0;
            // elf += 1;
            continue;
        }
        let calories: i32 = s.parse().unwrap();
        // println!("calories {}", calories);
        current_acc = current_acc + calories;
    }

    current_max
}

fn part2(input: &str) -> i32 {
    let lines = input.split("\n");

    let mut calories_per_elf = Vec::<i32>::new();
    let mut current_acc = 0;

    for s in lines {
        if s.len() == 0 {
            calories_per_elf.push(current_acc);
            current_acc = 0;
            continue;
        }
        let calories: i32 = s.parse().unwrap();
        // println!("calories {}", calories);
        current_acc += calories;
    }

    calories_per_elf.sort();
    calories_per_elf.reverse();

    calories_per_elf[0] + calories_per_elf[1] + calories_per_elf[2]
}
