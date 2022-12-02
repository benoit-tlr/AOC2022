use std::fs;

fn main() {
    const FILE: &str = "./input";

    let content = fs::read_to_string(FILE)
        .expect("Error reading the file.");

    let elfs: Vec<&str> = content.split("\n\n").collect();
    let mut total = vec![0, 0, 0];
    
    for elf in elfs {
        let cals: Vec<&str> = elf.split("\n").collect();
        let mut sum = 0;
        for cal in cals {
            if cal.len() == 0 {continue};
            sum += cal.parse::<i32>().unwrap();
        }
        
        let mut add = false;
        let mut add_idx = 0;

        for i in 0..3 {
            if sum > total[i] {
                add = true;
                add_idx = i;
            }
        }

        if add { 
            if add_idx > 0 {
                for i in 0..add_idx {
                    total[i] = total[i+1];
                }
            }
            total[add_idx] = sum; 
        }
    } 

    let result: i32 = total.iter().sum();
    println!("Total : {result}");

}
