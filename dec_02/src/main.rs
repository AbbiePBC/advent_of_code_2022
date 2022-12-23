use anyhow::{anyhow, Context, Error};
use std::fs::read_to_string;

fn main() {
    let total_score = get_scores("dec_02/actual_input.txt");
    println!("total score: {:?}", total_score);
}

const WIN_SCORE: u64 = 6;
const DRAW_SCORE: u64 = 3;
const LOSE_SCORE: u64 = 0;

// paper beats rock, rock beats scissors, scissors beats paper
// 2 beats 1, 1 beats 3, 3 beats 2

fn round_scores(your_score: u64, elf_score: u64) -> u64 {
    if your_score == elf_score {
        return your_score + DRAW_SCORE;
    } else if your_score > elf_score {
        if your_score == 3 && elf_score == 1 {
            return your_score + LOSE_SCORE;
        } else {
            return your_score + WIN_SCORE;
        }
    } else {
        if your_score == 1 && elf_score == 3 {
            return your_score + WIN_SCORE;
        } else {
            return your_score + LOSE_SCORE;
        }
    }
}

fn choose_draw(elf_score: u64) -> u64 {
    return elf_score + DRAW_SCORE;
}

fn choose_lose(elf_score: u64) -> u64 {
    if elf_score == 1 {
        return 3 + LOSE_SCORE;
    } else if elf_score == 2 {
        return 1 + LOSE_SCORE;
    } else {
        return 2 + LOSE_SCORE;
    }
}

fn choose_win(elf_score: u64) -> u64 {
    if elf_score == 3 {
        return 1 + WIN_SCORE;
    } else if elf_score == 1 {
        return 2 + WIN_SCORE;
    } else {
        return 3 + WIN_SCORE;
    }
}

fn get_scores(file_path: &str) -> Result<u64, Error> {
    let input_data = read_to_string(file_path)?;
    let round = input_data.split("\n");
    let mut total_score = 0;
    for r in round {
        if r.is_empty() {
            continue;
        }
        let elf_action = r
            .chars()
            .nth(0)
            .context(format!("Failed parsing: {:?}", r))?;
        let your_action = r
            .chars()
            .nth(2)
            .context(format!("Failed parsing: {:?}", r))?;
        let elf_score = match elf_action {
            'A' => 1,
            'B' => 2,
            'C' => 3,
            _ => return Err(anyhow!("Incorrect char {}", elf_action)),
        };
        let your_score = match your_action {
            'X' => choose_lose(elf_score),
            'Y' => choose_draw(elf_score),
            'Z' => choose_win(elf_score),
            _ => return Err(anyhow!("Incorrect char {}", your_action)),
        };
        total_score += your_score;
    }

    return Ok(total_score);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_all() {
        let total_score = get_scores("dec_02/test_input.txt").unwrap();
        assert_eq!(total_score, 12);
    }
}
