
fn main() {

}

#[derive(Debug, Clone, PartialEq)]
struct Tree {
    height: i32,
    is_visible: bool
}

impl Tree {
    fn new(height: i32, is_visible: bool) -> Tree {
        return Tree {
            height,
            is_visible,
        };
    }
}

struct Forest { }

impl Forest {
    fn new_from_string(input: &str) -> Vec<Vec<Tree>> {

        let input_lines: Vec<&str>= input.split("\n").collect();
        let grid_width = input_lines.len() -1; // note assumption that the grid is a square

        let mut forest : Vec<Vec<Tree>> = vec![vec![Tree::new(0, false); grid_width]; grid_width];
        let mut h : usize =  0;
        while h < grid_width { // width == height
            let line_of_trees = input_lines[h];
            for w in 0..grid_width {
                //read line into forest
                let tree_height = line_of_trees.chars().nth(w).unwrap();
                let can_see = can_see_tree(w, h, grid_width);
                forest[h][w] = Tree::new(tree_height as i32 - '0' as i32, can_see);
            }
            h+=1;
        }
        return forest;
    }

}

fn can_see_tree(w: usize, h: usize, grid_width: usize) -> bool {
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
        let grid : [[Tree; 1]; 1] = [[Tree::new(7, true)]; 1];
        assert_eq!(grid[0][0].height, 7);
        assert!(grid[0][0].is_visible);
    }
    #[test]
    fn test_outer_coordinates_are_visible() {
        let grid = Forest::new_from_string("303\n255\n653\n");
        assert_eq!(grid[0][0].height, 3);
        assert!(grid[0][0].is_visible);

        assert_eq!(grid[1][1].height, 5);
        assert!(!grid[1][1].is_visible);
    }

}
