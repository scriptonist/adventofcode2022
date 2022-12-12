fn main() {
    let input = include_str!("input.txt");
    println!("{}", p1(input));
    println!("{}", p2(input));
}
#[derive(Debug)]
struct Range {
    start: i32,
    end: i32,
}

impl Range {
    pub fn is_member(&self, range: &Self) -> bool {
        range.start >= self.start && range.end <= self.end
    }
    pub fn has_member(&self, range: &Self) -> bool {
        // 5-7 7-9
        // 1...........6
        //__   ______  _____
        (range.start <= self.end && range.start >= self.start)
            || (range.end >= self.start && range.end <= self.end)
    }
}

fn parse_input(input: &str) -> Vec<(Range, Range)> {
    let mut asgmts = vec![];
    for line in input.lines() {
        let (e1, e2) = line.split_once(',').unwrap();
        let parse_to_range = |s: &str| {
            let (r1, r2) = s.split_once('-').unwrap();
            Range {
                start: r1.parse::<i32>().unwrap(),
                end: r2.parse::<i32>().unwrap(),
            }
        };

        asgmts.push((parse_to_range(e1), parse_to_range(e2)))
    }

    asgmts
}

fn p1(input: &str) -> i32 {
    let mut c: i32 = 0;
    let asgmnts = parse_input(input);
    for (e1, e2) in asgmnts {
        if e1.is_member(&e2) || e2.is_member(&e1) {
            c += 1;
        }
    }

    c
}

fn p2(input: &str) -> i32 {
    let mut c: i32 = 0;
    let asgmnts = parse_input(input);
    for (e1, e2) in asgmnts {
        if e1.has_member(&e2) || e2.has_member(&e1) {
            c += 1;
        }
    }

    c
}
