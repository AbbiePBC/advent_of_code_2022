use std::fs::read_to_string;


const POSITION_NUM: usize = 1;
const POSITION_START: usize = 3;
const POSITION_END: usize = 5;

fn main() {
    let input_data = read_to_string("dec_05/actual_input.txt").unwrap();
    let (stacks, instructions) = parse_input(&input_data);
    let result = do_instructions(instructions, stacks);

    println!("Solution = {}", get_solution(result));

}


fn parse_input(input_data: &str) -> (Vec<Vec<char>>, Vec<&str>) {

    let separated_data: Vec<&str> = input_data.split("\n\n").collect();
    let raw_initial_structure: Vec<&str> = separated_data[0].split("\n").collect();
    let initial_stacks = get_stacks(raw_initial_structure);
    let raw_instructions: Vec<&str> = separated_data[1].split("\n").collect();

    return (initial_stacks, raw_instructions);
}


fn get_stacks(raw_initial_structure: Vec<&str>) -> Vec<Vec<char>> {
    let number_of_stacks = raw_initial_structure[raw_initial_structure.len() - 1]
        .split("   ")
        .collect::<Vec<&str>>()
        .len();
    let mut all_stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..number_of_stacks {
        all_stacks.push(Vec::new());
    }
    // the relevant char appears at the xth place in the string, where x = 4n + 1(zero indexed)

    for i in 0..raw_initial_structure.len() - 1 {
        // top is on the left of the stack
        let line = raw_initial_structure[i];
        for j in 0..line.len() {
            let index_into_string = 4 * j + 1;

            if line.is_empty() || (line.len() < index_into_string) {
                continue;
            }

            let ch = line.chars().nth(4 * j + 1);

            if ch.expect("is the char formula correct?").is_alphanumeric() {
                all_stacks[j].push(ch.unwrap());
            }
        }
    }

    reverse_stacks(&mut all_stacks);

    return all_stacks

}

fn reverse_stacks(all_stacks: &mut Vec<Vec<char>>) {
    for stack in all_stacks {
        stack.reverse();
    }
}

fn do_instruction(instruction: &str, all_stacks : &mut Vec<Vec<char>>) {
    let (num_boxes_to_move, start_point, end_point) = parse_instruction(instruction);
    for _ in 0..num_boxes_to_move {
        let char = all_stacks[start_point - 1].pop();
        match char {
            Some(ch) => { all_stacks[end_point - 1].push(ch); },
            None => { println!("this is an error."); }
        };
        //todo: error handle this properly

    }
}

fn parse_instruction(instruction: &str) -> (usize, usize, usize) {
    let inst: Vec<&str> = instruction.split(" ").collect();
    return (
        inst[POSITION_NUM].parse::<usize>().unwrap(),
        inst[POSITION_START].parse::<usize>().unwrap(),
        inst[POSITION_END].parse::<usize>().unwrap(),
    );
}

fn do_instructions(instructions: Vec<&str>, mut all_stacks: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for i in 0..instructions.len() -1 {
        do_instruction(instructions[i], &mut all_stacks);
    }
    return all_stacks;
}


fn get_solution(stacks: Vec<Vec<char>>) -> String {
    let mut answer : Vec<char> = Vec::new();
    for stack in stacks {
        answer.push(stack[stack.len() -1]);
    }
    return answer.into_iter().collect();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_stacks() {
        let result = get_stacks(vec!["    [D]", "[N] [C]", "[Z] [M] [P]", " 1   2   3"]);
        assert_eq!(result, vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]);
    }
    #[test]
    fn test_parse_input() {
        let input_data = read_to_string("dec_05/test_input.txt").unwrap();
        let (stacks, instructions) = parse_input(&input_data);
        assert_eq!(stacks, vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]);
        assert_eq!(
            instructions,
            [
                "move 1 from 2 to 1",
                "move 3 from 1 to 3",
                "move 2 from 2 to 1",
                "move 1 from 1 to 2",
                ""
            ]
        );
    }
    #[test]
    fn test_parse_instruction() {
        let (num, start, end) = parse_instruction("move 1 from 2 to 1");
        assert_eq!(num, 1);
        assert_eq!(start, 2);
        assert_eq!(end, 1);
        let (num, start, end) = parse_instruction("move 11 from 200 to 1");
        assert_eq!(num, 11);
        assert_eq!(start, 200);
        assert_eq!(end, 1);
    }
    #[test]
    fn test_single_instruction() {
        // test move 1, move x
        let mut stacks = get_stacks(vec!["    [D]", "[N] [C]", "[Z] [M] [P]", " 1   2   3"]);
        do_instruction("move 1 from 2 to 1", &mut stacks);
        assert_eq!(stacks, vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']]);

        do_instruction("move 3 from 1 to 3", &mut stacks);
        assert_eq!(stacks, vec![vec![], vec!['M', 'C'], vec!['P', 'D', 'N', 'Z']]);
    }
    #[test]
    fn test_all() {
        let input_data = read_to_string("dec_05/test_input.txt").unwrap();
        let (stacks, instructions) = parse_input(&input_data);
        let result = do_instructions(instructions, stacks);

        let ans = get_solution(result);
        assert_eq!(ans, "CMZ");
    }
}
