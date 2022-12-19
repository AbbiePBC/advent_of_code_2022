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
    //let mut total_contained_elves : u32 = 0;
    let mut total_overlapping_elves : u32 = 0;

    for elf_pair in elf_pairs {
        if elf_pair.is_empty() {
            continue;
        }
        let elves : Vec<&str> = elf_pair.split(",").collect();
        let elf_1 : Vec<&str> = elves[0].split("-").collect();
        let elf_2 : Vec<&str> = elves[1].split("-").collect();
    
        total_overlapping_elves += does_overlap(elf_1, elf_2) as u32;
    }
    //return Ok(total_contained_elves)
    return Ok(total_overlapping_elves)

    
}

fn is_included(elf_1: Vec<&str>, elf_2: Vec<&str>) -> u8 {
    
    if (is_min(elf_1[0], elf_2[0]) && is_max(elf_1[1], elf_2[1])) || (
        is_min(elf_2[0], elf_1[0]) && is_max(elf_2[1], elf_1[1])) {
            
        return 1;
    }
    return 0;
    
}


fn is_min(str_1: &str, str_2: &str) -> bool {
    return get_min(str_1, str_2) == str_1.parse::<u32>().unwrap()
}

fn get_min(str_1: &str, str_2: &str) -> u32 {
    return min(str_1.parse::<u32>().unwrap(), str_2.parse::<u32>().unwrap())
}

fn is_max(str_1: &str, str_2: &str) -> bool  {
    return get_max(str_1, str_2) == str_1.parse::<u32>().unwrap()
}

fn get_max(str_1: &str, str_2: &str) -> u32 {
    return max(str_1.parse::<u32>().unwrap(), str_2.parse::<u32>().unwrap())
}

fn does_overlap(elf_1: Vec<&str>, elf_2: Vec<&str>) -> u8 {
    let min_of_maxs = get_min(elf_1[1], elf_2[1]);
    let max_of_mins = get_max(elf_1[0], elf_2[0]);

    return (min_of_maxs >= max_of_mins) as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_included() {
        assert_eq!(is_included(vec!["10","20"], vec!["12","18"]), 1);
        assert_eq!(is_included(vec!["10","98"], vec!["1","2"]), 0);

    }
    #[test]
    fn test_does_overlap() {
       // assert_eq!(count_overlap(vec!["2","3"], vec!["3","4"]), 1);
       // assert_eq!(count_overlap(vec!["2","3"], vec!["30","34"]), 0);
        assert_eq!(does_overlap(vec!["5","7"], vec!["7","9"]), 1);
        assert_eq!(does_overlap(vec!["2","8"], vec!["3","7"]), 1);
        assert_eq!(does_overlap(vec!["6","6"], vec!["4","6"]), 1);
        assert_eq!(does_overlap(vec!["2","6"], vec!["4","8"]), 1);

    }

}