use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    println!("{}", p1(input));
    println!("{}", p2(input));
}

type Stack = Vec<char>;

#[derive(Debug)]
struct Instruction {
    mov: u32,
    from: u32,
    to: u32,
}

impl Instruction {
    fn new(s: &str) -> Self {
        let mut parts = s.split(' ');
        let mov = parts.nth(1).unwrap().parse::<u32>().unwrap();
        let from = parts.nth(1).unwrap().parse::<u32>().unwrap();
        let to = parts.nth(1).unwrap().parse::<u32>().unwrap();
        Self { mov, from, to }
    }
}

fn parse_input(input: &str) -> (HashMap<u32, Stack>, Vec<Instruction>) {
    let mut instructions = vec![];
    let mut lines = input.lines();
    let mut levels: Vec<&str> = vec![];
    loop {
        let v = lines.next().unwrap();
        if v.is_empty() {
            break;
        }
        levels.push(v);
    }
    let m = parse_stack(levels);
    for line in lines {
        instructions.push(Instruction::new(line));
    }
    (m, instructions)
}

fn parse_stack(s: Vec<&str>) -> HashMap<u32, Stack> {
    // println!("{:?}", s);
    let mut m: HashMap<u32, Stack> = HashMap::new();
    let mut rev_lines = s.iter().rev();
    let stack_ids = rev_lines.next().unwrap();
    let stack = rev_lines.map(|v| v.to_owned()).collect::<Vec<&str>>();

    let ids = stack_ids
        .chars()
        .filter(|v| v.is_numeric())
        .map(|v| v.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    // println!("{:?}", ids);
    let mut el_idx = 1usize;
    for id in ids {
        m.entry(id).or_insert_with(std::vec::Vec::new);
        for level in &stack {
            let el = level.chars().nth(el_idx).unwrap();
            if el.is_alphabetic() {
                m.get_mut(&id).unwrap().push(el);
            }
        }
        el_idx += 4;
    }

    // for (k, v) in &m {
    //     println!("{:?} {:?}", k, v);
    // }

    m
}

fn p1(input: &str) -> String {
    let (mut stacks, ops) = parse_input(input);
    for op in ops {
        for _ in 0..op.mov {
            let from = stacks.get_mut(&op.from).unwrap();
            let el = from.pop().unwrap();
            let to = stacks.get_mut(&op.to).unwrap();
            to.push(el);
        }
    }
    let mut s: String = "".to_string();
    for (_, v) in stacks.iter_mut().sorted() {
        v.reverse();
        s.push(v.first().unwrap().to_owned());
    }

    s
}

fn p2(input: &str) -> String {
    let (mut stacks, ops) = parse_input(input);
    for op in ops {
        let mut moved = vec![];
        for _ in 0..op.mov {
            let from = stacks.get_mut(&op.from).unwrap();
            let el = from.pop().unwrap();
            moved.push(el);
        }
        moved.reverse();
        let to = stacks.get_mut(&op.to).unwrap();
        to.append(&mut moved);
    }
    let mut s: String = "".to_string();
    for (_, v) in stacks.iter_mut().sorted() {
        v.reverse();
        s.push(v.first().unwrap().to_owned());
    }

    s
}
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn can_parse_stack() {
        const STACK: &str = r#"
    [M]             [Z]     [V]    
    [Z]     [P]     [L]     [Z] [J]
[S] [D]     [W]     [W]     [H] [Q]
[P] [V] [N] [D]     [P]     [C] [V]
[H] [B] [J] [V] [B] [M]     [N] [P]
[V] [F] [L] [Z] [C] [S] [P] [S] [G]
[F] [J] [M] [G] [R] [R] [H] [R] [L]
[G] [G] [G] [N] [V] [V] [T] [Q] [F]
 1   2   3   4   5   6   7   8   9 "#;

        let want: HashMap<u32, Stack> = [
            (1u32, vec!['G', 'F', 'V', 'H', 'P', 'S']),
            (2u32, vec!['G', 'J', 'F', 'B', 'V', 'D', 'Z', 'M']),
            (3u32, vec!['G', 'M', 'L', 'J', 'N']),
            (4u32, vec!['N', 'G', 'Z', 'V', 'D', 'W', 'P']),
            (5u32, vec!['V', 'R', 'C', 'B']),
            (6u32, vec!['V', 'R', 'S', 'M', 'P', 'W', 'L', 'Z']),
            (7u32, vec!['T', 'G', 'P']),
            (8u32, vec!['Q', 'R', 'S', 'N', 'C', 'H', 'Z', 'V']),
            (9u32, vec!['F', 'L', 'G', 'P', 'V', 'Q', 'J']),
        ]
        .iter()
        .cloned()
        .collect();
        assert_eq!(
            want,
            parse_stack(
                STACK
                    .lines()
                    .filter(|l| !l.is_empty())
                    .collect::<Vec<&str>>()
            )
        );
    }
}
/*
Part 1


*/
