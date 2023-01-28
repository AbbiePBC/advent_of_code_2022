use std::fs::read_to_string;

fn main() {
    let input_data = read_to_string("inputs/dec_08_input.txt").unwrap();
    let mut grid = Forest::new_from_string(&input_data);

    grid.update_visibility();
    println!("part 1: {}", grid.num_visible_trees);
}

#[derive(Debug, Clone, PartialEq)]
struct Tree {
    height: i32,
    is_visible: bool,
}

impl Tree {
    fn new(height: i32, is_visible: bool) -> Tree {
        return Tree { height, is_visible };
    }
}

struct Forest {
    forest: Vec<Vec<Tree>>,
    num_visible_trees: usize,
}
impl Forest {
    fn new_from_string(input: &str) -> Forest {
        let input_lines: Vec<&str> = input.split("\n").collect();
        let grid_width = input_lines.len() - 1; // note assumption that the grid is a square

        let mut forest: Vec<Vec<Tree>> = vec![vec![Tree::new(0, false); grid_width]; grid_width];
        let mut h: usize = 0;
        while h < grid_width {
            // width == height
            let line_of_trees = input_lines[h];
            for w in 0..grid_width {
                //read line into forest
                let tree_height = line_of_trees.chars().nth(w).unwrap();
                let can_see = is_outer_tree(w, h, grid_width); //todo: initialise w false instead
                forest[h][w] = Tree::new(tree_height as i32 - '0' as i32, can_see);
            }
            h += 1;
        }
        return Forest {
            forest,
            num_visible_trees: grid_width * 4 - 4,
        };
    }

    fn update_visibility(&mut self) {
        self.update_visibility_left_to_right();
        self.update_visibility_right_to_left();
        self.update_visibility_top_to_bottom();
        self.update_visibility_bottom_to_top();
    }

    fn update_tree_visibility(&mut self, x: usize, y: usize, height: i32) {
        self.forest[x][y] = Tree::new(height, true);
    }

    fn update_visibility_top_to_bottom(&mut self) {
        let grid_size = self.forest.len();
        for j in 0..grid_size {
            let mut min_tree_height_in_dimension = self.forest[0][j].height;

            for i in 0..grid_size {
                let tree_height = self.forest[i][j].height;
                if tree_height > min_tree_height_in_dimension {
                    if !self.forest[i][j].is_visible {
                        self.num_visible_trees += 1;
                    }
                    self.update_tree_visibility(i, j, tree_height);
                    min_tree_height_in_dimension = self.forest[i][j].height;
                }
            }
        }
    }

    fn update_visibility_bottom_to_top(&mut self) {
        let grid_size = self.forest.len();
        for j in 0..grid_size {
            let mut min_tree_height_in_dimension = self.forest[grid_size - 1][j].height;

            for i in (0..grid_size).rev() {
                let tree_height = self.forest[i][j].height;
                if tree_height > min_tree_height_in_dimension {
                    if !self.forest[i][j].is_visible {
                        self.num_visible_trees += 1;
                    }
                    self.update_tree_visibility(i, j, tree_height);
                    min_tree_height_in_dimension = self.forest[i][j].height;
                }
            }
        }
    }

    fn update_visibility_left_to_right(&mut self) {
        let grid_size = self.forest.len();
        for i in 0..grid_size {
            let mut min_tree_height_in_dimension = self.forest[i][0].height;

            for j in 0..grid_size {
                let tree_height = self.forest[i][j].height;
                if tree_height > min_tree_height_in_dimension {
                    if !self.forest[i][j].is_visible {
                        self.num_visible_trees += 1;
                    }
                    self.update_tree_visibility(i, j, tree_height);
                    min_tree_height_in_dimension = self.forest[i][j].height;
                }
            }
        }
    }
    fn update_visibility_right_to_left(&mut self) {
        let grid_size = self.forest.len();
        for i in 0..grid_size {
            let mut min_tree_height_in_dimension = self.forest[i][grid_size - 1].height;

            for j in (0..grid_size).rev() {
                let tree_height = self.forest[i][j].height;
                if tree_height > min_tree_height_in_dimension {
                    if !self.forest[i][j].is_visible {
                        self.num_visible_trees += 1;
                    }
                    self.update_tree_visibility(i, j, tree_height);
                    min_tree_height_in_dimension = self.forest[i][j].height;
                }
            }
        }
    }
}

fn is_outer_tree(w: usize, h: usize, grid_width: usize) -> bool {
    if h == 0 || w == 0 || h == grid_width - 1 || w == grid_width - 1 {
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_visible() {
        let grid: [[Tree; 1]; 1] = [[Tree::new(7, true)]; 1];
        assert_eq!(grid[0][0].height, 7);
        assert!(grid[0][0].is_visible);
    }
    #[test]
    fn test_outer_coordinates_are_visible() {
        let mut grid = Forest::new_from_string("303\n205\n653\n");
        grid.update_visibility();
        assert_eq!(grid.forest[0][0].height, 3);
        assert!(grid.forest[0][0].is_visible);

        assert_eq!(grid.forest[1][1].height, 0);
        assert!(!grid.forest[1][1].is_visible);
    }
    #[test]
    fn test_inner_coordinates_are_visible() {
        let mut grid = Forest::new_from_string("343\n265\n653\n");
        grid.update_visibility();
        for line in grid.forest {
            for tree in line {
                assert!(tree.is_visible);
            }
        }
    }
    #[test]
    fn count_visible_trees() {
        let mut grid = Forest::new_from_string("30373\n25512\n65332\n33549\n35390\n");

        // 30373
        // 255X2
        // 65X32
        // 3Y5X9
        // 35390

        grid.update_visibility();
        assert_eq!(grid.num_visible_trees, 21);
        //assert_eq!(grid.num_visible_trees, 21);
    }

    #[test]
    fn count_visible_trees_from_left_to_right() {
        let mut grid = Forest::new_from_string("0123\n0130\n1303\n1001\n");
        // 0123
        // 0110
        // 13X3 <-
        // 1001
        grid.update_visibility_left_to_right();
        assert_eq!(grid.num_visible_trees, 15);
    }
    #[test]
    fn count_visible_trees_from_right_to_left() {
        let mut grid = Forest::new_from_string("3210\n0104\n1303\n1001\n");
        // 3210
        // 0XX4
        // 1XX3
        // 1001
        grid.update_visibility_right_to_left();
        assert_eq!(grid.num_visible_trees, 12);
    }
    #[test]
    fn count_visible_trees_from_top_to_bottom() {
        let mut grid = Forest::new_from_string("3210\n0104\n1303\n1001\n");
        // 3210
        // 0XX4
        // 13X3
        // 1001
        grid.update_visibility_top_to_bottom();
        assert_eq!(grid.num_visible_trees, 13);
    }
    #[test]
    fn count_visible_trees_from_bottom_to_top() {
        let mut grid = Forest::new_from_string("3210\n0104\n1303\n1001\n");
        // 3210
        // 0XX4
        // 13X3
        // 1001
        grid.update_visibility_bottom_to_top();
        assert_eq!(grid.num_visible_trees, 13);
    }
    #[test]
    fn count_visible_trees_small() {
        let mut grid = Forest::new_from_string("3037\n6533\n3359\n3539\n");

        // 3037
        // 65X3
        // 3Y59
        // 3539

        grid.update_visibility();
        assert_eq!(grid.num_visible_trees, 14);
    }
}
