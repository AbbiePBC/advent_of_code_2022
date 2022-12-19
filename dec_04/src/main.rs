use anyhow::Error;
use std::fs::read_to_string;
use std::cmp::{min, max};

fn main() {
    let solution = day_04("actual_input.txt");
    println!("{:?}", solution);
}

fn day_04(file_path: &str) -> Result<u32,  Error> {
    let contents = read_to_string(file_path)?;
    let elf_pairs : Vec<&str> = contents.split("\n").collect();
    let mut total_contained_elves : u32= 0;
    for elf_pair in elf_pairs {
        total_contained_elves += is_included(elf_pair) as u32;
    }
    return Ok(total_contained_elves)
    
}

fn is_included(elf_pair: &str) -> u8 {
    if elf_pair.is_empty() {
        return 0;
    }
    let elves : Vec<&str> = elf_pair.split(",").collect();
    let elf_1 : Vec<&str> = elves[0].split("-").collect();
    let elf_2 : Vec<&str> = elves[1].split("-").collect();

    if (is_min(elf_1[0], elf_2[0]) && is_max(elf_1[1], elf_2[1])) || (
        is_min(elf_2[0], elf_1[0]) && is_max(elf_2[1], elf_1[1])) {
            
        return 1;
    }
    return 0;
    
}


fn is_min(str_1: &str, str_2: &str) -> bool {
    return min(str_1.parse::<i32>().unwrap(), str_2.parse::<i32>().unwrap()) == str_1.parse::<i32>().unwrap()
}


fn is_max(str_1: &str, str_2: &str) -> bool  {
    return max(str_1.parse::<i32>().unwrap(), str_2.parse::<i32>().unwrap()) == str_1.parse::<i32>().unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_included() {
        assert_eq!(is_included("10-20,12-18"), 1);
        assert_eq!(is_included("42-91,42-73"), 1);
        assert_eq!(is_included("10-98,1-2"), 0);
        assert_eq!(is_included("42-91,42-73"), 1);


    }

}