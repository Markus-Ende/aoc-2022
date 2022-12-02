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
        let instruction = &round[1];
        // print!("opponent {} instruction {}", opponent, instruction);
        let own = choose_move(&opponent, &instruction);
        // println!(" -> move {}", own);
        score += calculate_score(opponent, &own);
    }

    println!("{}", score);
}

fn choose_move(opponent: &String, instruction: &String) -> String {
    // A: Rock
    // B: Paper
    // C: Scissors
    // X: need to lose
    // Y: need to draw
    // Z: need to win
    match &instruction[..] {
        "X" => get_losing_move(opponent),
        "Y" => get_draw_move(opponent),
        "Z" => get_winning_move(opponent),
        _ => panic!(),
    }
}

fn get_winning_move(opponent: &String) -> String {
    // A: Rock
    // B: Paper
    // C: Scissors
    // X: Rock
    // Y: Paper
    // Z: Scissors
    match &opponent[..] {
        "A" => String::from("Y"),
        "B" => String::from("Z"),
        "C" => String::from("X"),
        _ => panic!(),
    }
}

fn get_draw_move(opponent: &String) -> String {
    // A: Rock
    // B: Paper
    // C: Scissors
    // X: Rock
    // Y: Paper
    // Z: Scissors
    match &opponent[..] {
        "A" => String::from("X"),
        "B" => String::from("Y"),
        "C" => String::from("Z"),
        _ => panic!(),
    }
}

fn get_losing_move(opponent: &String) -> String {
    // A: Rock
    // B: Paper
    // C: Scissors
    // X: Rock
    // Y: Paper
    // Z: Scissors
    match &opponent[..] {
        "A" => String::from("Z"),
        "B" => String::from("X"),
        "C" => String::from("Y"),
        _ => panic!(),
    }
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
        _ => panic!(),
    };

    score
}
