use anyhow::{Context, Result};
use std::fs::read_to_string;

fn main() {
    let calories_per_elf = read_and_get_calories_per_elf("puzzle_input.txt");
    match calories_per_elf {
        Ok(mut cals) => {
            cals.sort();
            //println!("Part One: {}", get_top_totals(cals, 1));
            println!("Part Two: {}", get_top_totals(cals, 3));
        },
        Err(_) => {},
    };
}

fn read_and_get_calories_per_elf(file_path: &str) -> Result<Vec<i64>, anyhow::Error> {
    let lines = read_to_string(file_path)
        .with_context(|| format!("Failed to read file from {}", file_path))?;
    let calories_for_elf: Vec<&str> =  lines.split("\n\n").collect();
    let mut calorie_count_for_elf : Vec<i64> = Vec::with_capacity(calories_for_elf.len() -1);
    for elf in calories_for_elf {
        calorie_count_for_elf.push(get_calories_for_elf(elf));
    }
    return Ok(calorie_count_for_elf);
}

fn get_calories_for_elf(elf: &str) -> i64 {
    let all_calories_collected: Vec<&str> = elf.split("\n").collect();
    let mut sum_for_elf = 0;
    for calories in all_calories_collected {
        let _ = match calories.parse::<i64>() {
            Ok(cal) => { sum_for_elf += cal; },
            Err(_) => {},
        };
    }
    return sum_for_elf
}

fn get_top_totals( cals: Vec<i64>, i: usize) -> i64 {
    let mut total = 0;
    let num_elves_less_1 = cals.len();
    for i in  (num_elves_less_1 - i)..num_elves_less_1 {
        total += cals[i];
    }
    return total
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_all() {
        let mut calories_per_elf = read_and_get_calories_per_elf("./test/short_test.txt").unwrap();
        assert_eq!(calories_per_elf, [6000, 4000, 11000, 24000, 10000]);

        calories_per_elf.sort();
        assert_eq!(calories_per_elf, [4000, 6000, 10000, 11000, 24000]);
        assert_eq!(get_top_totals(calories_per_elf, 3), 45000);

    }
}
