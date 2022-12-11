use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");
    println!("{}", p1(input));
    println!("{}", p2(input));
}

fn p2(input: &str) -> u32 {
    let sacks = input.lines().collect::<Vec<&str>>();
    let mut total = 0;
    for group in sacks.chunks(3) {
        let mut count: HashMap<char, i32> = HashMap::new();
        for sack in group {
            let mut visited: HashSet<char> = HashSet::new();
            for char in sack.chars() {
                if !visited.contains(&char) {
                    visited.insert(char);
                    *count.entry(char).or_default() += 1;
                }
            }
        }
        for (k, v) in count {
            if v == 3 {
                total += get_priority(k);
            }
        }
    }

    total
}

fn p1(input: &str) -> u32 {
    let mut s = 0;
    for line in input.lines() {
        let (first, second) = line.split_at(line.len() / 2);
        let mut visited: HashSet<char> = HashSet::new();
        for c in first.chars() {
            if visited.contains(&c) {
                continue;
            }
            if let Some(_) = second.find(c) {
                println!("{} {}", c, get_priority(c));
                s += get_priority(c);
                break;
            }
            visited.insert(c);
        }
    }
    s
}
fn get_priority(c: char) -> u32 {
    if c.is_lowercase() {
        return 26 - (122 - c as u32);
    }

    52 - (90 - c as u32)
}

#[cfg(test)]
mod tests {
    use crate::get_priority;

    #[test]
    fn can_get_code() {
        assert_eq!(get_priority('a'), 1);
        assert_eq!(get_priority('b'), 2);
        assert_eq!(get_priority('z'), 26);
        assert_eq!(get_priority('A'), 27);
        assert_eq!(get_priority('B'), 28);
        assert_eq!(get_priority('Z'), 52);
    }
}
