use std::fs::read_to_string;
use substring::Substring;
use anyhow::{anyhow, Error};

fn main() {
    let score = get_rucksack_compartments("dec_03/actual_input.txt");
    println!("score = {:?}", score);
}

fn get_rucksack_compartments(file_path : &str) -> Result<u32, Error> {
    let input_data = read_to_string(file_path).unwrap();
    let rucksacks : Vec<&str> = input_data.split("\n").collect();
    let mut total_score = 0;

    // part 1
    // for rucksack in rucksacks {
    //     if rucksack.is_empty() {
    //         continue;
    //     }
    //     let ch = get_common_chars(rucksack.to_string())?;
    //     let score = get_score(ch);
    //     total_score += score;
    // }

    let num = rucksacks.len() -1;
    for i in (0..num).step_by(3) {
        let elf_1 = rucksacks[i];
        let elf_2 = rucksacks[i + 1];
        let elf_3 = rucksacks[i+2];
        let ch = get_common_chars_for_elves(elf_1.to_string(), elf_2.to_string(), elf_3.to_string())?;
        let score = get_score(ch);
        total_score += score;
    }

    return Ok(total_score);

}

fn get_score(ch: char) -> u32 {

    if ch.is_lowercase() {
        return (ch as u32 - 'a' as u32) + 1
    } else {
        return (ch as u32 - 'A' as u32) + 27
    }
}

fn get_common_chars(rucksack: String) -> Result<char, Error> {
    let num_items : f64 = rucksack.len() as f64;
    let first_compartment = rucksack.substring(0, (num_items/2.0).ceil() as usize);
    let second_compartment = rucksack.substring((num_items/2.0).floor() as usize, num_items as usize);

    for ch in first_compartment.chars() {
        if second_compartment.contains(ch) {
            return Ok(ch);
        }
    }

    return Err(anyhow!("The elf didn't make a mistake this time!"));
}

fn get_common_chars_for_elves(elf_1: String, elf_2: String, elf_3: String) -> Result<char, Error> {

    for ch in elf_1.chars() {
        if elf_2.contains(ch) && elf_3.contains(ch) {
            return Ok(ch)
        }
    }
    return Err(anyhow!("Common character not found for ef group"));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_char() {
        let ch = get_common_chars("JrwpWtwJgWrhcsFMMfFFhFp".to_string());
        assert_eq!(ch.unwrap(), 'p');
        // vJrwpWtwJgWrhcsFMMfFFhFp
        // jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        // PmmdzqPrVvPwwTWBwg
        // wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        // ttgJtRGJQctTZtZT
        // CrZsJsPPZsGzwwsLwLmpwMDw
    }
    #[test]
    fn test_score(){
        let score = get_score('p');
        assert_eq!(score, 16);

        let upper = get_score('L');
        assert_eq!(upper, 38);
        // 16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s); the sum of these is 157.
    }
    // #[test]
    // fn test_all(){
    //     let score : u32 = get_rucksack_compartments("test_input.txt").unwrap();
    //     assert_eq!(score, 157);
    // }
    #[test]
    fn test_elf_group(){
        let score : u32 = get_rucksack_compartments("dec_03/test_input.txt").unwrap();
        assert_eq!(score, 70);
    }
}
