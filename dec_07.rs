use std::{collections::HashMap, fs::read_to_string};
use std::thread::current;

use substring::Substring;

fn main(){
    // how to const in rust?

    // could formulate this problem as a tree and then search all nodes for their weight
    // i dont want to do that

    // {"path": size} for every possible path and duplicate info is easier
    let fs = set_up_filesystem("inputs/dec_07_input.txt");
    //let fs = set_up_filesystem("inputs/dec_07_input.txt");
    // println!("{:?}", fs.get_size_of_small_directories());
    // println!("space used = {}", fs["//"]);
    // println!("space allowed {}", 70000000 - 30000000);
    // println!("delete smallest dir larger than {}", fs["//"] - (70000000 - 30000000 ));

    // todo: add cd .. until home in code rather than editing input lol
}

fn set_up_filesystem(file_path: &str) -> HashMap<String, u64> {
    let mut filesystem : HashMap<String, u64> = HashMap::new();

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
        if line.starts_with("$ cd ..") {
            current_path = filesystem.change_directory_back_and_update_size(previous_line, &mut current_size, &mut previous_size, &current_path.clone());
        } else if line.starts_with("$ cd") {
            current_path = filesystem.change_directory_forward_and_update_size(line, &mut current_size,  &current_path.clone());

        } else {
            let l : Vec<&str>  = line.split(" ").collect();
            if l[0] == "dir" || l[1] == "ls" {
                continue;
            } else {
                current_size += l[0].parse::<u64>().unwrap();
            }

        }
        previous_line = line;
    }

    while current_path != "~" {
        current_path = filesystem.change_directory_back_and_update_size(previous_line, &mut current_size, &mut previous_size, &current_path.clone());
    }


    return filesystem
}

fn set_up_test_filesystem() -> HashMap<String, u64> {
    let mut filesystem = HashMap::new();

    filesystem.insert("/a/e/i".to_string(), 584);
    filesystem.insert("/a/e/".to_string(), 584+29116+2557+62596);
    filesystem.insert("/b.txt/".to_string(), 14848514);
    filesystem.insert("/c.dat/".to_string(), 8504156);
    filesystem.insert("/d/".to_string(), 4060174+8033020+5626152+7214296);

    return filesystem;

}

trait PathTraits {
    fn update_path(&mut self, dir_to_add: &str);
}

impl PathTraits for String{
    fn update_path(&mut self, dir_to_add: &str) {
        let mut path = "/".to_owned();
        if dir_to_add == "/".to_string() {
            path = "~".to_string();
        } else {
            path += dir_to_add;
        }
        *self = self.clone().to_owned() + &path;
    }

}

trait FileSystemTraits {
    fn change_directory_back_and_update_size(&mut self, previous_line: &str, current_size: &mut u64, previous_size: &mut u64, current_path: &str) -> String;
    fn change_directory_forward_and_update_size(&mut self, line: &str, current_size: &mut u64, current: &str) -> String;

    }

impl FileSystemTraits for HashMap<String, u64> {
    fn change_directory_back_and_update_size(&mut self, previous_line: &str, current_size: &mut u64, previous_size: &mut u64, current: &str) -> String{
        let mut current_path = current.clone().to_string();
        if previous_line.starts_with("$ cd .."){
            *current_size = *previous_size;
        } else {
            self.get_mut(&current_path).map(|val| { *val += *current_size; });

        }

        let idx = current_path.rfind('/');
        match idx {
            Some(idx_val) => {
                // go back to the parent folder
                current_path = current_path.substring(0, idx_val).to_string();
            }
            None => {
                current_path = "~".to_string();
            }
        }
        self.get_mut(&current_path).map(|val| { *val += *current_size; });
        *previous_size = *current_size;
        return current_path;

    }


    fn change_directory_forward_and_update_size(&mut self, line: &str, current_size: &mut u64, current: &str) -> String {
        let mut current_path = current.to_string();
        let l : Vec<&str>  = line.split(" ").collect();
        let dir = l[l.len() - 1];
        self.get_mut(&current_path).map(|val| { *val += *current_size; });
        *current_size = 0;
        current_path.update_path(dir);
        self.insert(current_path.to_string(), 0);
        return current_path;
    }
}

trait SizeOfLargeDirectories {
    fn get_size_of_small_directories(&self) -> u64;
}

impl SizeOfLargeDirectories for HashMap<String, u64> {

fn get_size_of_small_directories(&self) -> u64 {

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

        let fs = set_up_filesystem("inputs/dec_07_test_small.txt");
        assert_eq!(HashMap::from([("~".to_string(), 14848514+8504156+7214296), ("~/a".to_string(), 7214296)]), fs);

    }
    #[test]
    fn test_setup_fs_and_get_solution(){

        let fs = set_up_filesystem("inputs/dec_07_test.txt");
        assert_eq!(HashMap::from([("~".to_string(), 48381165), ("~/d".to_string(), 24933642), ("~/a".to_string(), 94853), ("~/a/e".to_string(), 584)]), fs);
        assert_eq!(fs.get_size_of_small_directories(), 95437);
    }
    #[test]
    fn test_find_bug(){
        let fs = set_up_filesystem("inputs/dec_07_bug.txt");
        assert_eq!(HashMap::from([("~/wzpth".to_string(), 15123,), ("~".to_string(), 15123), ("~/wzpth/snhss/hlw".to_string(), 15123), ("~/wzpth/snhss".to_string(), 15123)]), fs);

    }
    #[test]
    fn test_home_dir(){

        let fs = set_up_filesystem("inputs/dec_07_test_small.txt");
        assert_eq!(HashMap::from([("~".to_string(), 14848514 + 8504156 + 7214296), ("~/a".to_string(), 7214296)]), fs);
    }

    #[test]
    fn test_size_output(){

        let fs = set_up_test_filesystem();

        assert_eq!(fs.get_size_of_small_directories(), 95437);

    }

}