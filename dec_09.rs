use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug, Clone, PartialEq)]
struct CoordinateStruct {
    current_head: (i32, i32),
    current_tail: (i32, i32),
    coordinates_visited: HashSet<(i32, i32)>,
}

impl CoordinateStruct {
    fn new() -> CoordinateStruct {
        return CoordinateStruct {
            current_head: (0, 0),
            current_tail: (0, 0),
            coordinates_visited: HashSet::new(),
        };
    }
    fn move_up(&mut self, step_count: i32) {
        let (current_head_x, mut current_head_y) = self.current_head;
        for _ in 0..step_count {
            current_head_y = current_head_y + 1;
            self.current_head = (current_head_x, current_head_y);
            if self.move_tail_same_direction() {
                let (current_tail_x, current_tail_y) = self.current_tail;
                self.current_tail = (current_tail_x, current_tail_y + 1)
            } else if self.move_tail_diagonally() {
                self.current_tail = (current_head_x, current_head_y - 1)
            }
            self.coordinates_visited.insert(self.current_tail);
        }
    }
    fn move_down(&mut self, step_count: i32) {
        let (current_head_x, mut current_head_y) = self.current_head;
        for _ in 0..step_count {
            current_head_y = current_head_y - 1;
            self.current_head = (current_head_x, current_head_y);
            if self.move_tail_same_direction() {
                let (current_tail_x, current_tail_y) = self.current_tail;
                self.current_tail = (current_tail_x, current_tail_y - 1)
            } else if self.move_tail_diagonally() {
                self.current_tail = (current_head_x, current_head_y + 1)
            }
            self.coordinates_visited.insert(self.current_tail);
        }
    }
    fn move_right(&mut self, step_count: i32) {
        let (mut current_head_x, current_head_y) = self.current_head;
        for _ in 0..step_count {
            current_head_x = current_head_x + 1;
            self.current_head = (current_head_x, current_head_y);
            if self.move_tail_same_direction() {
                let (current_tail_x, current_tail_y) = self.current_tail;
                self.current_tail = (current_tail_x + 1, current_tail_y)
            } else if self.move_tail_diagonally() {
                self.current_tail = (current_head_x - 1, current_head_y)
            }
            self.coordinates_visited.insert(self.current_tail);
        }
    }
    fn move_left(&mut self, step_count: i32) {
        let (mut current_head_x, current_head_y) = self.current_head;
        for _ in 0..step_count {
            current_head_x = current_head_x - 1;
            self.current_head = (current_head_x, current_head_y);
            if self.move_tail_same_direction() {
                let (current_tail_x, current_tail_y) = self.current_tail;
                self.current_tail = (current_tail_x - 1, current_tail_y)
            } else if self.move_tail_diagonally() {
                self.current_tail = (current_head_x + 1, current_head_y)
            }
            self.coordinates_visited.insert(self.current_tail);
        }
    }
    fn move_tail_same_direction(&self) -> bool {
        let (head_x, head_y) = self.current_head;
        let (tail_x, tail_y) = self.current_tail;

        if head_x == tail_x && head_y.abs_diff(tail_y) > 1
            || head_y == tail_y && head_x.abs_diff(tail_x) > 1
        {
            return true;
        }

        return false;
    }

    fn move_tail_diagonally(&self) -> bool {
        let (head_x, head_y) = self.current_head;
        let (tail_x, tail_y) = self.current_tail;

        if head_x.abs_diff(tail_x) == 1 && head_y.abs_diff(tail_y) == 1 {
            return false;
        }
        if head_x.abs_diff(tail_x) >= 1 && head_y.abs_diff(tail_y) >= 1 {
            return true;
        }

        return false;
    }
}

fn main() {
    let instruction_string = read_to_string("inputs/dec_09_input.txt").unwrap();

    let coordinates = do_instructions(instruction_string);

    println!("Part 1: {} different coordinates", coordinates.len());
}

fn do_instructions(instruction_string: String) -> HashSet<(i32, i32)> {
    let instructions: Vec<&str> = instruction_string.split("\n").collect();
    let mut coordinate_struct = CoordinateStruct::new();

    for instruction in &instructions {
        if instruction.is_empty() {
            continue;
        }
        let b = instruction.to_string();
        let step: Vec<&str> = b.split(" ").collect();
        let direction = step[0].clone();
        let step_count = step[1].parse().unwrap();
        match direction {
            "R" => coordinate_struct.move_right(step_count),
            "L" => coordinate_struct.move_left(step_count),
            "U" => coordinate_struct.move_up(step_count),
            "D" => coordinate_struct.move_down(step_count),
            _ => {}
        }
    }

    return coordinate_struct.coordinates_visited;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_move_up() {
        let mut coords = CoordinateStruct::new();

        coords.current_head = (4, 0);
        coords.current_tail = (3, 0);

        coords.move_up(1);

        assert_eq!(coords.current_head, (4, 1));
        assert_eq!(coords.current_tail, (3, 0));

        coords.move_up(1);

        assert_eq!(coords.current_head, (4, 2));
        assert_eq!(coords.current_tail, (4, 1));
    }
    #[test]
    fn test_move_down() {
        let mut coords = CoordinateStruct::new();

        coords.current_head = (1, 4);
        coords.current_tail = (2, 4);

        coords.move_down(1);

        assert_eq!(coords.current_head, (1, 3));
        assert_eq!(coords.current_tail, (2, 4));
    }
    #[test]
    fn test_move_right() {
        let mut coords = CoordinateStruct::new();

        coords.move_right(4);

        assert_eq!(coords.current_head, (4, 0));
        assert_eq!(coords.current_tail, (3, 0));
    }
    #[test]
    fn test_tail_positions() {
        let mut coords = CoordinateStruct::new();

        coords.move_right(4);
        coords.move_up(4);
        coords.move_left(3);
        coords.move_down(1);
        coords.move_right(4);
        coords.move_down(1);
        coords.move_left(5);
        coords.move_right(2);

        assert_eq!(coords.coordinates_visited.len(), 13);
    }
    #[test]
    fn test_tail_positions_from_string() {
        let coords = do_instructions("R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2".to_string());

        assert_eq!(coords.len(), 13);
    }
}
