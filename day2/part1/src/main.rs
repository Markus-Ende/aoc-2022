use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    // println!("Input file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines = contents.split("\n");
    let mut score = 0;

    for s in lines.filter(|line| line.len() > 0) {
        let round: Vec<String> = s.split(" ").map(str::to_string).collect();
        let opponent = &round[0];
        let own = &round[1];
        // println!(
        //     "opponent {} own {} score {}",
        //     opponent,
        //     own,
        //     calculate_score(opponent, own)
        // );
        score += calculate_score(opponent, own);
    }

    println!("{}", score);
}

fn calculate_score(opponent: &String, own: &String) -> i32 {
    // A: Rock
    // B: Paper
    // C: Scissors
    // X: Rock
    // Y: Paper
    // Z: Scissors
    let mut score = " XYZ".find(own).unwrap() as i32;

    score += match &opponent[..] {
        "A" => "Z  X  Y".find(own).unwrap() as i32,
        "B" => "X  Y  Z".find(own).unwrap() as i32,
        "C" => "Y  Z  X".find(own).unwrap() as i32,
        _ => 0,
    };

    score
}
