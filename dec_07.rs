use std::{collections::HashMap, fs::read_to_string};

use substring::Substring;

fn main(){
    // how to const in rust?

    // could formulate this problem as a tree and then search all nodes for their weight
    // i dont want to do that

    // {"path": size} for every possible path and duplicate info is easier
    let fs = set_up_filesystem("inputs/dec_07_input.txt");
    //let fs = set_up_filesystem("inputs/dec_07_input.txt");
    println!("{:?}", fs.get_size_of_small_directories());

    // todo: add cd .. until home in code rather than editing input lol
}

fn set_up_filesystem(file_path: &str) -> HashMap<String, i32> {
    let mut filesystem = HashMap::new();

    // not error handling this
    let contents = read_to_string(file_path).ok().unwrap();
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut current_path = "".to_string();
    let mut current_size = 0;
    let mut previous_size = 0;

    let mut previous_line = "/";
    for line in lines {
        if line.is_empty() {
            break;
        }
        println!("current path = {}", current_path);
        println!("current size = {}", current_size);

        println!("reading line {}", line);
        if line.starts_with("$ cd ..") {
            println!("current path: {}", current_path);
            if previous_line.starts_with("$ cd .."){
                current_size = previous_size;
            } else {
                 println!("go back from {}", current_path);
                // add path and weight to fs
                filesystem.get_mut(&current_path).map(|val| { *val += current_size; });

            }
           
            let idx = current_path.rfind('/').unwrap();
            // go back to the parent folder
            current_path = current_path.substring(0, idx).to_string();
            println!("current size - checking path {}", current_path);
            filesystem.get_mut(&current_path).map(|val| { *val += current_size; });
            previous_size = current_size;
            current_size = 0;

            
        } else if line.starts_with("$ cd") {
            let l : Vec<&str>  = line.split(" ").collect();
            let dir = l[l.len() - 1];
            filesystem.get_mut(&current_path).map(|val| { *val += current_size; });
            current_size =0;
            // update path
            current_path.update_path(dir);
            filesystem.insert(current_path.to_string(), 0);
        } else {
            //if dir, do nothing as we haven't gone there yet
            // if file, add size
            let l : Vec<&str>  = line.split(" ").collect();
            println!("directory = {}", l[1]);
            if l[0] == "dir" || l[1] == "ls" {
                continue;
            } else {
                let file_size = l[0];
                println!("file size {}", file_size);
                current_size += file_size.parse::<i32>().unwrap();
            }

        }
        previous_line = line;
    }
    return filesystem
}

fn set_up_test_filesystem() -> HashMap<String, i32> {
    let mut filesystem = HashMap::new();

    filesystem.insert("/a/e/i".to_string(), 584);
    filesystem.insert("/a/e/".to_string(), 584+29116+2557+62596);
    filesystem.insert("/b.txt/".to_string(), 14848514);
    filesystem.insert("/c.dat/".to_string(), 8504156);
    filesystem.insert("/d/".to_string(), 4060174+8033020+5626152+7214296);

    return filesystem;

}

trait EditPath {
    fn update_path(&mut self, dir_to_add: &str);
}

impl EditPath for String {
    fn update_path(&mut self, dir_to_add: &str) {
        let path = "/".to_owned() + dir_to_add;
        *self = self.clone().to_owned() + &path;
    }
}

trait SizeOfLargeDirectories {
    fn get_size_of_small_directories(&self) -> i32;
}

impl SizeOfLargeDirectories for HashMap<String, i32> {

    fn get_size_of_small_directories(&self) -> i32 {

        let min_directory_size = 100000;
    
        let mut total_size_of_directories_smaller_than_max = 0;
        for directory in self {
            if *directory.1 <= min_directory_size {
                println!("Directory: {} has size {}", directory.0, directory.1);
                total_size_of_directories_smaller_than_max += directory.1;
            }
        }
        return total_size_of_directories_smaller_than_max;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_update_path(){
    
        let mut path = "/dir_a".to_string();
        path.update_path("dir_b");
        assert_eq!(path, "/dir_a/dir_b");
    }
    #[test]
    fn test_setup_fs(){
        //$ cd /
        // $ ls
        // dir a
        // 14848514 b.txt
        // 8504156 c.dat
        // $ cd a
        // $ ls
        // 7214296 k

        let fs = set_up_filesystem("inputs/dec_07_test_small.txt");
        assert_eq!(HashMap::from([("//".to_string(), 14848514+8504156+7214296), ("///a".to_string(), 7214296)]), fs);

    }
    #[test]
    fn test_setup_fs_and_get_solution(){

        let fs = set_up_filesystem("inputs/dec_07_test.txt");
        assert_eq!(HashMap::from([("//".to_string(), 48381165), ("///d".to_string(), 24933642), ("///a".to_string(), 94853), ("///a/e".to_string(), 584)]), fs);
        assert_eq!(fs.get_size_of_small_directories(), 95437);
    }
    #[test]
    fn test_find_bug(){
        let fs = set_up_filesystem("inputs/dec_07_bug.txt");
        assert_eq!(HashMap::from([("///wzpth".to_string(), 15123,), ("//".to_string(), 15123), ("///wzpth/snhss/hlw".to_string(), 15123), ("///wzpth/snhss".to_string(), 15123)]), fs);
        
    }

    #[test]
    fn test_size_output(){
    
        let fs = set_up_test_filesystem();

        assert_eq!(fs.get_size_of_small_directories(), 95437);

    }
    
}    