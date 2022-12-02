use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Input file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines = contents.split("\n");

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

    println!(
        "Max calories {}",
        calories_per_elf[0] + calories_per_elf[1] + calories_per_elf[2]
    )
}
