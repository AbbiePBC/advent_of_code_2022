use std::thread::current;
use std::{collections::HashMap, fs::read_to_string};

use substring::Substring;

fn main() {
    let fs = set_up_filesystem("inputs/dec_07_input.txt");
    println!("part one: {:?}", fs.get_size_of_small_directories());
    let part_two = fs.smallest_large_directories();
    println!("part two: {}", part_two);
}

fn set_up_filesystem(file_path: &str) -> HashMap<String, u64> {
    let mut filesystem: HashMap<String, u64> = HashMap::new();

    // not error handling this
    let contents = read_to_string(file_path).ok().unwrap();
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut current_path = "".to_string();
    let mut previous_path = "".to_string();

    let mut previous_line = "/";
    for line in lines {
        if line.is_empty() {
            break;
        }
        if line.starts_with("$ cd ..") {
            current_path = filesystem.change_directory_back_and_update_size(
                previous_line,
                &current_path.clone(),
                &previous_path.clone(),
            );
            previous_path = current_path.clone();
        } else if line.starts_with("$ cd") {
            current_path = filesystem.change_directory_forward_and_update_size(
                line,
                &current_path.clone(),
                &previous_path.clone(),
            );
            previous_path = current_path.clone();
        } else {
            let l: Vec<&str> = line.split(" ").collect();
            if l[0] == "dir" || l[1] == "ls" {
                continue;
            } else {
                filesystem.get_mut(&current_path).map(|val| {
                    *val += l[0].parse::<u64>().unwrap();
                });
            }
        }
        previous_line = line;
    }

    while current_path != "~" {
        current_path = filesystem.change_directory_back_and_update_size(
            previous_line,
            &current_path.clone(),
            &previous_path,
        );
    }

    return filesystem;
}

fn set_up_test_filesystem() -> HashMap<String, u64> {
    let mut filesystem = HashMap::new();

    filesystem.insert("/a/e/i".to_string(), 584);
    filesystem.insert("/a/e/".to_string(), 584 + 29116 + 2557 + 62596);
    filesystem.insert("/b.txt/".to_string(), 14848514);
    filesystem.insert("/c.dat/".to_string(), 8504156);
    filesystem.insert("/d/".to_string(), 4060174 + 8033020 + 5626152 + 7214296);

    return filesystem;
}

trait PathTraits {
    fn update_path(&mut self, dir_to_add: &str);
}

impl PathTraits for String {
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
    fn change_directory_back_and_update_size(
        &mut self,
        previous_line: &str,
        current_path: &str,
        previous_path: &str,
    ) -> String;
    fn change_directory_forward_and_update_size(
        &mut self,
        line: &str,
        current_path: &str,
        previous_path: &str,
    ) -> String;
}

impl FileSystemTraits for HashMap<String, u64> {
    fn change_directory_back_and_update_size(
        &mut self,
        previous_line: &str,
        current: &str,
        previous_path: &str,
    ) -> String {
        let mut current_size = 0;
        let mut current_path = current.clone().to_string();
        if previous_line.starts_with("$ cd ..") {
            current_size = *self.get(previous_path).unwrap();
        } else {
            self.get_mut(&current_path).map(|val| {
                *val += current_size;
            });
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
        let cp = self.clone();
        let prev_size = cp.get(previous_path).unwrap();
        self.get_mut(&current_path).map(|val| {
            *val += *prev_size;
        });
        return current_path;
    }

    fn change_directory_forward_and_update_size(
        &mut self,
        line: &str,
        current: &str,
        previous_path: &str,
    ) -> String {
        let mut current_path = current.to_string();
        let l: Vec<&str> = line.split(" ").collect();
        let dir = l[l.len() - 1];
        current_path.update_path(dir);
        self.insert(current_path.to_string(), 0);
        return current_path;
    }
}

trait Solve {
    fn get_size_of_small_directories(&self) -> u64;
    fn smallest_large_directories(&self) -> u64;
}

impl Solve for HashMap<String, u64> {
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
    fn smallest_large_directories(&self) -> u64 {
        let total_space_used = self["~"];
        let maximum_space = 40000000;
        let space_needed = total_space_used - maximum_space;
        let mut directory_to_delete = (&"~".to_string(), &self["~"]);

        for dir in self {
            if dir.1 > &space_needed {
                if dir.1 < &directory_to_delete.1 {
                    directory_to_delete = dir;
                }
            }
        }

        return *directory_to_delete.1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_update_path() {
        let mut path = "/dir_a".to_string();
        path.update_path("dir_b");
        assert_eq!(path, "/dir_a/dir_b");
    }
    #[test]
    fn test_setup_fs() {
        let fs = set_up_filesystem("inputs/dec_07_test_small.txt");
        assert_eq!(
            HashMap::from([
                ("~".to_string(), 14848514 + 8504156 + 7214296),
                ("~/a".to_string(), 7214296)
            ]),
            fs
        );
    }
    #[test]
    fn test_setup_fs_and_get_solution() {
        let fs = set_up_filesystem("inputs/dec_07_test.txt");
        assert_eq!(
            HashMap::from([
                ("~".to_string(), 48381165),
                ("~/d".to_string(), 24933642),
                ("~/a".to_string(), 94853),
                ("~/a/e".to_string(), 584)
            ]),
            fs
        );
        assert_eq!(fs.get_size_of_small_directories(), 95437);
    }
    #[test]
    fn test_find_bug() {
        let fs = set_up_filesystem("inputs/dec_07_bug.txt");
        assert_eq!(
            HashMap::from([
                ("~/wzpth".to_string(), 15123,),
                ("~".to_string(), 15123),
                ("~/wzpth/snhss/hlw".to_string(), 15123),
                ("~/wzpth/snhss".to_string(), 15123)
            ]),
            fs
        );
    }
    #[test]
    fn test_home_dir() {
        let fs = set_up_filesystem("inputs/dec_07_test_small.txt");
        assert_eq!(
            HashMap::from([
                ("~".to_string(), 14848514 + 8504156 + 7214296),
                ("~/a".to_string(), 7214296)
            ]),
            fs
        );
    }

    #[test]
    fn test_size_output() {
        let fs = set_up_test_filesystem();

        assert_eq!(fs.get_size_of_small_directories(), 95437);
    }
}
