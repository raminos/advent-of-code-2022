use std::env;
use std::fs;

fn get_calories_per_elf(values: &Vec<i32>) -> Vec<i32> {
    values.iter().fold(Vec::new(), |mut acc, &number| {
        if acc.is_empty() {
            acc.push(number);
            acc
        } else if number == 0 {
            acc.push(0);
            acc
        } else {
            let last_idx = acc.len() - 1;
            acc[last_idx] += number;
            acc
        }
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut contents = fs::read_to_string(file_path).unwrap();
    // Last line is not needed
    contents.pop();

    let calories = contents
        .split('\n')
        .map(|value| {
            if value == "" {
                0
            } else {
                value.parse::<i32>().unwrap()
            }
        })
        .collect();

    let mut calories_per_elf = get_calories_per_elf(&calories);
    calories_per_elf.sort();

    let start_idx = calories_per_elf.len() - 3;
    let highest_three = &calories_per_elf[start_idx..];
    let sum: i32 = highest_three.iter().sum();

    println!("Result Part One: {}", highest_three[2]);
    println!("Result Part Two: {}", sum)
}
