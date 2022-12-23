use std::fs::read_to_string;

fn main(){
    let input_data = read_to_string("dec_05/test_input.txt").unwrap();
    let (stacks, instructions) = parse_input(&input_data);
    //todo
}

fn parse_input(input_data: &str) ->  (Vec<Vec<char>>, Vec<&str>) {
    println!("input data = {}", input_data);
    let separated_data : Vec<&str> = input_data.split("\n\n").collect();
    let raw_initial_structure : Vec<&str> = separated_data[0].split("\n").collect();
    let initial_stacks = get_stacks(raw_initial_structure);

    let raw_instructions : Vec<&str> = separated_data[1].split("\n").collect();
    println!("{:?}", raw_instructions);

    return (initial_stacks, raw_instructions)
}

fn get_stacks(raw_initial_structure : Vec<&str>) -> Vec<Vec<char>> {
    println!("raw initial structre = {:?}", raw_initial_structure);
    let number_of_stacks = raw_initial_structure[raw_initial_structure.len()-1].split("   ").collect::<Vec<&str>>().len();
    let mut all_stacks : Vec<Vec<char>> = Vec::new();
    for i in 0..number_of_stacks {
        all_stacks.push(Vec::new());
    }
    // the relevant char appears at the xth place in the string, where x = 4n + 1(zero indexed)

    for i in 0..raw_initial_structure.len()-1 {
        // top is on the left of the stack
        let line = raw_initial_structure[i];
        for j in 0..line.len() {
            let index_into_string = 4*j + 1;

            if line.is_empty() || (line.len() < index_into_string){
                continue;
            }

            let ch = line.chars().nth(4*j+1);

            if ch.expect("is the char formula correct?").is_alphanumeric() {
                all_stacks[j].push(ch.unwrap());
            }
        }
    }
    println!("stacks = {:?}", all_stacks);
    return all_stacks;
}

fn do_instruction() {
    //todo: move x ... should be one by one
}

fn do_instructions() {
    //todo: do_instruction per line of instructions
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_stacks(){
        let result = get_stacks(vec!["    [D]", "[N] [C]", "[Z] [M] [P]", " 1   2   3"]);
        assert_eq!(result, vec![vec!['N', 'Z'], vec!['D', 'C', 'M'], vec!['P']]);
    }
    #[test]
    fn test_parse_input(){
        let input_data = read_to_string("dec_05/test_input.txt").unwrap();
        let (stacks, instructions) = parse_input(&input_data);
        assert_eq!(stacks, vec![vec!['N', 'Z'], vec!['D', 'C', 'M'], vec!['P']]);
        assert_eq!(instructions, ["move 1 from 2 to 1", "move 3 from 1 to 3", "move 2 from 2 to 1", "move 1 from 1 to 2", ""]);
    }
    #[test]
    fn test_single_instruction(){
        // test move 1, move x
    }
    #[test]
    fn test_all(){
        // test move 1, move x
    }
}
