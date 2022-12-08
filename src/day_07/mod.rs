use std::{fmt::{self}, iter, collections::HashSet};
use regex::{self, Regex};

pub fn calculate_directory_size_sum(input: &str) -> u32 {
    let file_list = parse_commands_to_file_list(input);
    let dir_list = build_directory_list(&file_list);

    println!("Dirs: ");
    for f in &dir_list {
        println!("{:?}", f);
    }

    let total_size = dir_list
        .iter()
        .filter(|d| d.size <= 100000)
        .map(|d| d.size)
        .sum();
    return total_size;
}

fn parse_commands_to_file_list(input: &str) -> Vec<ElvenFile> {
    let mut files: Vec<ElvenFile> = Vec::new();
    let mut cur_dir: Vec<String> = Vec::new();

    let change_dir_regex = Regex::new(r"\$ cd (\S+)").unwrap();
    let file_name_regex = Regex::new(r"(\d+) (\S+)").unwrap();
    
    for l in input.lines() {
        let trimmed = l.trim();
        if let Some(file_name_captures) = file_name_regex.captures(trimmed) {
            let path: Vec<String> = cur_dir
                .iter()
                .cloned()
                .chain(iter::once(file_name_captures[2].to_string()))
                .collect();

            files.push(ElvenFile { 
                path: path,
                size: file_name_captures[1].parse().unwrap()
            });
        }

        if let Some(change_dir) = change_dir_regex.captures(trimmed) {
            match &change_dir[1] {
                "/" => cur_dir.truncate(0),
                ".." => {
                    cur_dir.pop();
                },
                dir => cur_dir.push(dir.to_owned()),
            }
        }
    }

    // for f in &files {
    //     println!("{:?}", f);
    // }
    return files;
}

fn build_directory_list(file_list: &Vec<ElvenFile>) -> Vec<ElvenFile> {
    let dir_paths: HashSet<Vec<String>> = HashSet::from_iter(file_list
        .iter()
        .map(|f| f.path[..(f.path.len() - 1)].to_vec())
    );
    let dirs = dir_paths.iter()
        .map(|path| ElvenFile {
            path: path.to_vec(),
            size: compute_size_for_dir(path, file_list)
        })
        .collect();
    return dirs;
}

fn compute_size_for_dir(path: &Vec<String>, file_list: &Vec<ElvenFile>) -> u32 {
    return file_list
        .iter()
        .filter(|f| f.is_decendent_of(path))
        .map(|f| f.size)
        .sum();
}

struct ElvenFile {
    path: Vec<String>,
    size: u32,
}

impl ElvenFile {
    fn is_decendent_of(&self, path: &Vec<String>) -> bool {
        if self.path.len() < path.len() {
            return false;
        }
        for i in 0..path.len() {
            if self.path[i] != path[i] {
                return false;
            }
        }
        return true;
    }
}

impl fmt::Debug for ElvenFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:?} {:?}", self.path, self.size))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let input = r#"
            $ cd /
            $ ls
            dir a
            14848514 b.txt
            8504156 c.dat
            dir d
            $ cd a
            $ ls
            dir e
            29116 f
            2557 g
            62596 h.lst
            $ cd e
            $ ls
            584 i
            $ cd ..
            $ cd ..
            $ cd d
            $ ls
            4060174 j
            8033020 d.log
            5626152 d.ext
            7214296 k
        "#;
        let size = calculate_directory_size_sum(input.trim());
        assert_eq!(size, 95437);
    }
}