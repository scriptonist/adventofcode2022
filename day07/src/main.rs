use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    iter::Peekable,
    path::{Path, PathBuf},
    str::Lines,
    vec,
};

fn main() {
    let input = include_str!("./input.txt");
    println!("{}", p1(input));
    println!("{}", p2(input));
}

#[derive(Debug)]
struct File {
    name: String,
    size: i64,
}

type Files = Vec<File>;
type Dirs = HashMap<String, Files>;

struct Parser<'a> {
    lines: Peekable<Lines<'a>>,
    dir_pointer: PathBuf,
    dirs: Dirs,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let lines = input.lines().peekable();
        Self {
            lines,
            dir_pointer: PathBuf::new(),
            dirs: HashMap::new(),
        }
    }

    fn parse_cd(&mut self) {
        let dir = self.lines.next().unwrap().split(' ').nth(2).unwrap();
        match dir {
            ".." => {
                self.dir_pointer.pop();
            }
            _ => {
                self.dir_pointer.push(dir);
            }
        }
    }

    fn parse_ls(&mut self) {
        self.lines.next();
        loop {
            if let Some(next) = self.lines.peek() {
                if next.starts_with("$ cd") {
                    break;
                }
            }
            if self.lines.peek().is_none() {
                break;
            }
            let (size, name) = self.lines.next().unwrap().split_once(' ').unwrap();
            let cur_dir = self.dir_pointer.to_str().unwrap();
            if !self.dirs.contains_key(cur_dir) {
                self.dirs.insert(cur_dir.to_owned(), vec![]);
            }
            if size.chars().next().unwrap().is_numeric() {
                self.dirs.get_mut(cur_dir).unwrap().push(File {
                    name: name.to_owned(),
                    size: size.parse::<i64>().unwrap(),
                });
            }
        }
    }

    pub fn parse(mut self) -> Dirs {
        while let Some(next) = self.lines.peek() {
            if next.starts_with("$ cd") {
                self.parse_cd();
            } else {
                self.parse_ls();
            }
        }

        self.dirs
    }
}

fn parse_input(input: &str) -> HashMap<String, i64> {
    let dirs = Parser::new(input).parse();
    let mut sizes: HashMap<String, i64> = HashMap::new();
    for dir in dirs.keys() {
        find_size(dir, &dirs, &mut sizes);
    }
    // for (k, v) in dirs {
    //     println!("{} ----", k);
    //     for file in v {
    //         println!("{:?}", file);
    //     }
    // }

    sizes
}
fn p1(input: &str) -> i64 {
    let sizes = parse_input(input);
    let mut total = 0;
    // println!("{:?}", sizes);
    for (file, size) in &sizes {
        if *size <= 100000 {
            // println!("{} {}", file, size);
            total += size;
        }
    }
    total
}

fn p2(input: &str) -> i64 {
    let sizes = parse_input(input);
    let root_size = sizes.get("/").unwrap();
    let space_needed: i64 = 70000000 - 30000000;
    let space_needed_to_free = root_size - space_needed;
    sizes
        .into_values()
        .filter(|v| *v >= space_needed_to_free)
        .min()
        .unwrap()
}

fn find_size(cur_dir: &str, dirs: &Dirs, memo: &mut HashMap<String, i64>) -> i64 {
    if !dirs.contains_key(cur_dir) {
        return 0;
    }
    let mut size = 0;
    if let Some(size) = memo.get(cur_dir) {
        return *size;
    }
    for file in dirs.get(cur_dir).unwrap() {
        size += file.size;
    }
    let mut visited: HashSet<String> = HashSet::new();
    for directory in dirs.keys() {
        if directory.starts_with(cur_dir) && directory != cur_dir && !visited.contains(directory) {
            if !memo.contains_key(directory) {
                size += find_size(directory, dirs, memo);
            } else {
                size += memo.get(directory).unwrap_or(&0i64);
            }
            visited.insert(directory.to_owned());
        }
    }
    memo.insert(cur_dir.to_owned(), size);
    size
}
