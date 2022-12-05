use std::fs;
use sscanf;

fn main() {
    let lines = fs::read_to_string("./input").expect("Couldn't read the file.");
    let lines: Vec<&str> = lines.split("\n").collect();
    let num_lines = lines.len();
    let mut counter = 0;
    let mut current_line = lines[counter].to_string();

    let mut stacks: Vec<Vec<String>> = Vec::new();

    for _ in 0..9 {
        stacks.push(Vec::new())
    }

    while !(current_line.starts_with(" 1")) {
        let mut idx = 1;
        let mut char_counter = 0;
        while idx < current_line.len() {
            let char = current_line.chars().nth(idx).unwrap().to_string();
            if char != " " {
                stacks[char_counter].push(char);
            }
            idx += 4;
            char_counter += 1;
        }
        counter += 1;
        current_line = lines[counter].to_string();
    }
    
    for i in 0..9 {
        stacks[i].reverse();
    }
    
    counter += 2;
    current_line = lines[counter].to_string();

    while counter < num_lines {
        let parsed = sscanf::scanf!(current_line, "move {} from {} to {}", u16, u16, u16); 
        let parsed = match parsed {
            Ok(value) => value,
            Err(_) => { counter += 1; continue },
        };
        let mut tmp_vec = Vec::new();
        for _ in 0..parsed.0 {
            //print!("{:?} -> {} {} {} -> ", stacks, parsed.0 as usize, parsed.1 as usize - 1, parsed.2 as usize - 1);
            let tmp = stacks[parsed.1 as usize - 1].pop().unwrap();
            tmp_vec.push(tmp);
            //println!("{:?}", stacks);
        }
        while !tmp_vec.is_empty() {
            stacks[parsed.2 as usize - 1].push(tmp_vec.pop().unwrap());
        }
        counter += 1;
        current_line = lines[counter].to_string();
    }

    for i in 0..9 {
        print!("{}", stacks[i][stacks[i].len() - 1]);
    }

}
