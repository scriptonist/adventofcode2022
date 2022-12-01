fn main() {
    let input = include_str!("./input.txt");
    println!("{}", p1(input));
    println!("{}", p2(input));
}

fn p1(input: &str) -> i32 {
    let mut max = 0;
    let mut cur_max = 0;
    for line in input.lines() {
        if line.is_empty() {
            if cur_max > max {
                max = cur_max;
            }
            cur_max = 0;
            continue;
        }
        cur_max += line.parse::<i32>().unwrap();
    }
    max
}

fn p2(input: &str) -> i32 {
    let mut cur_max = 0;
    let mut calories: Vec<i32> = vec![];
    for line in input.lines() {
        if line.is_empty() {
            calories.push(cur_max);
            cur_max = 0;
            continue;
        }
        cur_max += line.parse::<i32>().unwrap();
    }
    calories.push(cur_max);
    calories.sort();
    calories.iter().rev().take(3).sum()
}
