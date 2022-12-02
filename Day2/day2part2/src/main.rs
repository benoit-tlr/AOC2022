use std::fs;

fn main() {
    let lines = fs::read_to_string("./input").expect("Couldn't read the file.");
    let lines: Vec<&str> = lines.split("\n").collect();

    let mut score = 0;

    for line in lines {
        if line.len() == 0 { continue }
        let line = line.replace(" ", "");
        let line = line.replace("A", "R");
        let line = line.replace("B", "P");
        let mut line = line.replace("C", "S");
        let sequence = "RSP";

        let current_index = sequence.chars().position(|c| c == line.chars().nth(0).unwrap()).unwrap() as i8;
        let replacement: i8 = match line.chars().nth(1) {
            Some('X') => 1,
            Some('Y') => 0,
            Some('Z') => -1,
            _ => continue,
        };
        let new_index: i8 = current_index + replacement;
        let new_index: i8 = match new_index {
            -1 => 2,
            3 => 0,
            _ => new_index,
        };
        line.replace_range(1..2, sequence.chars().nth(new_index as usize)
            .unwrap().to_string().as_str());

        let win = match line.as_str() {
            "SR" => true,
            "PS" => true,
            "RP" => true,
            _ => false,
        };
        let draw = match line.as_str() {
            "RR" => true,
            "PP" => true,
            "SS" => true,
            _ => false,
        };

        let mut subscore = if win {6} else { if draw {3} else {0} };

        subscore += match line.chars().nth(1) {
            Some('R') => 1,
            Some('P') => 2,
            Some('S') => 3,
            _ => 0,
        };
        score += subscore;
        println!("Round {line} : {subscore}");
    }
    println!("Score: {score}");
}
