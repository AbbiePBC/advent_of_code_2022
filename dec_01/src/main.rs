use anyhow::{Context, Result};
use std::cmp;
use std::fs::read_to_string;

fn main() {
    let max = max_calories_for_elf("puzzle_input.txt");
    println!("{:?}", max)
}

fn max_calories_for_elf(file_path: &str) -> Result<i64, anyhow::Error> {
    let lines = read_to_string(file_path)
        .with_context(|| format!("Failed to read file from {}", file_path))?;
    let calories_for_elf: Vec<&str> =  lines.split("\n\n").collect();
    let mut max_calories_for_elf = 0;
    for elf in calories_for_elf {
        let all_calories_collected: Vec<&str> = elf.split("\n").collect();
        let mut sum_for_elf = 0;
        for calories in all_calories_collected {
            let _ = match calories.parse::<i64>() {
                Ok(cal) => { sum_for_elf += cal; },
                Err(_) => {},
            };
        }
        max_calories_for_elf = cmp::max(max_calories_for_elf, sum_for_elf);
    }
    return Ok(max_calories_for_elf);
}

#[cfg(test)]
mod tests {
    use crate::max_calories_for_elf;
    #[test]
    fn test_file_parsing() {
        let calorie_list_for_elf = max_calories_for_elf("test/short_test.txt");
        println!("{:?}", calorie_list_for_elf);
    }
}
