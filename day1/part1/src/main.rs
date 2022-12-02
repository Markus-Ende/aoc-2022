use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Input file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines = contents.split("\n");

    let mut current_max = 0;
    let mut current_acc = 0;
    let mut elf = 1;

    for s in lines {
        if s.len() == 0 {
            if current_acc > current_max {
                current_max = current_acc;
            }
            println!("elf {}: {} calories", elf, current_acc);
            current_acc = 0;
            elf += 1;
            continue;
        }
        let calories: i32 = s.parse().unwrap();
        // println!("calories {}", calories);
        current_acc = current_acc + calories;
    }

    println!("Max calories {}", current_max)
}
