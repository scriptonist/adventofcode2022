use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    println!("{}", p1(input));
    println!("{}", p2(input));
}

fn p1(input: &str) -> i32 {
    for (idx, c) in input.chars().enumerate() {
        if idx > 2 {
            let last4 = input[idx - 3..idx + 1].chars().collect::<HashSet<char>>();
            if last4.len() == 4 {
                return idx as i32 + 1;
            }
        }
    }

    0
}

fn p2(input: &str) -> i32 {
    for (idx, c) in input.chars().enumerate() {
        if idx > 12 {
            let last4 = input[idx - 13..idx + 1].chars().collect::<HashSet<char>>();
            if last4.len() == 14 {
                return idx as i32 + 1;
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_p1() {
        assert_eq!(p1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(p1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(p1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(p1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(p1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
    #[test]
    fn check_p2() {
        assert_eq!(p2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(p2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(p2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(p2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(p2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
