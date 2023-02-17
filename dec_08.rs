use std::fs::read_to_string;
use std::thread::current;
use std::cmp::max;

fn main() {
    let input_data = read_to_string("inputs/dec_08_input.txt").unwrap();
    let mut grid = Forest::new_from_string(&input_data);

    grid.update_visibility();
    println!("part 1: {}", grid.num_visible_trees);

    println!("part 2: {}", grid.update_all_scenic_scores_and_find_max());
}

#[derive(Debug, Clone, PartialEq)]
struct Tree {
    height: i32,
    is_visible: bool,
    scenic_score: usize,
}

impl Tree {
    fn new(height: i32, is_visible: bool, scenic_score: usize) -> Tree {
        return Tree { height, is_visible, scenic_score};
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

        let mut forest: Vec<Vec<Tree>> = vec![vec![Tree::new(0, false, 0); grid_width]; grid_width];
        let mut h: usize = 0;
        while h < grid_width {
            // width == height
            let line_of_trees = input_lines[h];
            for w in 0..grid_width {
                //read line into forest
                let tree_height = line_of_trees.chars().nth(w).unwrap();
                let can_see = is_outer_tree(w, h, grid_width); //todo: initialise w false instead
                forest[h][w] = Tree::new(tree_height as i32 - '0' as i32, can_see, 0);
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
        self.forest[x][y] = Tree::new(height, true, 0);
    }

    // the x and y indexing has somehow switched.

    fn update_tree_scenic_score(&mut self, x: usize, y: usize){
        let left_score = self.closest_distance_tree_left_from(x, y);
        let right_score = self.closest_distance_tree_right_from(x, y);
        let up_score = self.closest_distance_tree_up_from(x, y);
        let down_score = self.closest_distance_tree_down_from(x, y);
        self.forest[y][x].scenic_score = left_score * right_score * up_score * down_score;
    }

    fn update_all_scenic_scores_and_find_max(&mut self) -> usize{
        let mut max_scenic_score = 0;
        for i in 0..self.forest.len(){
            for j in 0..self.forest.len() {
                self.update_tree_scenic_score(i, j);
                max_scenic_score = max(max_scenic_score, self.forest[j][i].scenic_score);
            }
        }
        return max_scenic_score;
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
    fn closest_distance_tree_up_from(&self, x: usize, y: usize) -> usize {
        let tree_height = self.forest[y][x].height;
        for tree in (0..y).rev(){
            if self.forest[tree][x].height >= tree_height {
                return y - tree;
            }
        }
        return y;
    }

    fn closest_distance_tree_down_from(&self, x: usize, y: usize) -> usize {
        let tree_height = self.forest[y][x].height;
        let num_trees_below = self.forest.len() - y - 1;

        for tree in y+1..self.forest.len(){
            let current_height =  self.forest[tree][x].height;
            if current_height >= tree_height {
                return tree - y;
            }
        }
        return num_trees_below;
    }

    fn closest_distance_tree_left_from(&self, x: usize, y: usize) -> usize {
        let tree_height = self.forest[y][x].height;

        for tree in (0..x).rev() {
            if self.forest[y][tree].height >= tree_height {
                return x - tree;
            }
        }
        return x;
    }
    fn closest_distance_tree_right_from(&self, x: usize, y: usize) -> usize {
        let tree_height = self.forest[y][x].height;
        let num_trees_right = self.forest.len() - x - 1 ;

        for tree in x+1..self.forest.len() - 1 {
            if self.forest[y][tree].height >= tree_height {
                return tree - x;
            }
        }
        return num_trees_right;
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
        let grid: [[Tree; 1]; 1] = [[Tree::new(7, true, 0)]; 1];
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

    #[test]
    fn get_closest_up() {
        let grid = Forest::new_from_string("3037\n6533\n3350\n3539\n");

        // 3037
        // 6533
        // 3350
        // 353X

        assert_eq!(grid.closest_distance_tree_up_from(3, 3), 3);
    }

    #[test]
    fn get_closest_down() {
        let grid = Forest::new_from_string("3037\n6533\n3350\n3539\n");

        // 3X37
        // 6533
        // 3350
        // 3539

        assert_eq!(grid.closest_distance_tree_down_from(1, 0), 1);
    }

    #[test]
    fn get_closest_left() {
        let grid = Forest::new_from_string("3337\n6533\n3350\n3539\n");

        // 33X7
        // 6533
        // 3350
        // 3539

        assert_eq!(grid.closest_distance_tree_left_from(2, 0), 1);
    }

    #[test]
    fn get_closest_right() {
        let grid = Forest::new_from_string("3037\n6533\n3350\n3539\n");

        // 3037
        // 6533
        // 335X
        // 3539

        assert_eq!(grid.closest_distance_tree_right_from(3, 2), 0);
    }

    #[test]
    fn scenic_score() {
        let mut grid = Forest::new_from_string("30373\n25512\n65332\n33549\n35390\n");

        // 30373
        // 25X12
        // 65332
        // 33549
        // 35390

        grid.update_tree_scenic_score(2, 1);
        // indexing is the wrong way round with the new additions to the code
        assert_eq!(grid.forest[1][2].scenic_score, 4);

        grid.update_tree_scenic_score(2, 3);
        assert_eq!(grid.forest[3][2].scenic_score, 8);
    }


    #[test]
    fn max_scenic_score() {
        let mut grid = Forest::new_from_string("30373\n25512\n65332\n33549\n35390\n");
        assert_eq!(grid.update_all_scenic_scores_and_find_max(), 8);
    }

}
