use std::fs;

fn main() {
    let lines = fs::read_to_string("./input").expect("Couldn't read the file.");
    let lines: Vec<&str> = lines.split("\n").collect();

    let mut score = 0;

    for line in lines {
        let line = line.replace(" ", "");
        let line = line.replace("A", "R");
        let line = line.replace("X", "R");
        let line = line.replace("B", "P");
        let line = line.replace("Y", "P");
        let line = line.replace("C", "S");
        let line = line.replace("Z", "S");

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
