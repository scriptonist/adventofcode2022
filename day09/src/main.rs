use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    println!("{}", p1(input));
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn up(&mut self, by: i32) {
        self.y += by;
    }
    fn down(&mut self, by: i32) {
        self.y += by;
    }
    fn left(&mut self, by: i32) {
        self.x += by;
    }
    fn right(&mut self, by: i32) {
        self.x += by;
    }
    fn jump_up_right_diag(&mut self) {
        self.y += 1;
        self.x += 1;
    }
    fn jump_up_left_diag(&mut self) {
        self.y += 1;
        self.x -= 1;
    }
    fn jump_down_right_diag(&mut self) {
        self.y -= 1;
        self.x += 1;
    }
    fn jump_down_left_diag(&mut self) {
        self.y -= 1;
        self.x -= 1;
    }
    fn is_one_apart_left(&self, p: &Pos) -> bool {
        p.x - self.x == 2 && p.y == self.y
    }
    fn is_one_apart_right(&self, p: &Pos) -> bool {
        self.x - p.x == 2 && self.y == p.y
    }
    fn is_one_apart_up(&self, p: &Pos) -> bool {
        self.y - p.y == 2 && self.x == p.x
    }
    fn is_one_apart_down(&self, p: &Pos) -> bool {
        p.y - self.y == 2 && p.x == self.x
    }
    fn is_up_row_left(&self, p: &Pos) -> bool {
        self.x != p.x
            && self.y != p.y
            && self.y > p.y
            && self.x < p.x
            && ((self.y - p.y).abs() == 2 || (self.x - p.x).abs() == 2)
    }
    fn is_up_row_right(&self, p: &Pos) -> bool {
        self.x != p.x
            && self.y != p.y
            && self.y > p.y
            && self.x > p.x
            && ((self.y - p.y).abs() == 2 || (self.x - p.x).abs() == 2)
    }
    fn is_down_row_left(&self, p: &Pos) -> bool {
        self.x != p.x
            && self.y != p.y
            && self.y < p.y
            && self.x < p.x
            && ((self.y - p.y).abs() == 2 || (self.x - p.x).abs() == 2)
    }
    fn is_down_row_right(&self, p: &Pos) -> bool {
        self.x != p.x
            && self.y != p.y
            && self.y < p.y
            && self.x > p.x
            && ((self.y - p.y).abs() == 2 || (self.x - p.x).abs() == 2)
    }
}
struct Rope {
    head: Pos,
    tail: Pos,
    tail_pos: Vec<Pos>,
}

impl Rope {
    pub fn new() -> Self {
        Self {
            head: Pos { x: 0, y: 0 },
            tail: Pos { x: 0, y: 0 },
            tail_pos: vec![Pos { x: 0, y: 0 }],
        }
    }
    pub fn mov(&mut self, mov: Move) {
        for _ in 0..mov.count {
            self.mov_head(&mov.direction);
            self.mov_tail();
            self.tail_pos.push(self.tail);
        }
    }

    fn mov_head(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.head.up(1),
            Direction::Down => self.head.down(-1),
            Direction::Right => self.head.right(1),
            Direction::Left => self.head.left(-1),
        }
    }
    fn mov_tail(&mut self) {
        if self.head.is_one_apart_left(&self.tail) {
            self.tail.left(-1);
        } else if self.head.is_one_apart_right(&self.tail) {
            self.tail.right(1);
        } else if self.head.is_one_apart_up(&self.tail) {
            self.tail.up(1);
        } else if self.head.is_one_apart_down(&self.tail) {
            self.tail.down(-1);
        } else if self.head.is_up_row_right(&self.tail) {
            self.tail.jump_up_right_diag()
        } else if self.head.is_up_row_left(&self.tail) {
            self.tail.jump_up_left_diag()
        } else if self.head.is_down_row_right(&self.tail) {
            self.tail.jump_down_right_diag()
        } else if self.head.is_down_row_left(&self.tail) {
            self.tail.jump_down_left_diag()
        }
    }
}

#[derive(Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}
#[derive(Debug)]
struct Move {
    direction: Direction,
    count: i32,
}

fn parse_input(input: &str) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    for line in input.lines() {
        let (direction, count) = line.split_once(' ').unwrap();
        let direction: Direction = match direction {
            "R" => Direction::Right,
            "U" => Direction::Up,
            "L" => Direction::Left,
            "D" => Direction::Down,
            _ => unreachable!(),
        };
        let count = count.parse::<i32>().unwrap();
        moves.push(Move { direction, count })
    }
    moves
}

fn p1(input: &str) -> i32 {
    let moves = parse_input(input);
    let mut rope = Rope::new();
    for mov in moves {
        rope.mov(mov);
    }
    rope.tail_pos.iter().collect::<HashSet<&Pos>>().len() as i32
}
