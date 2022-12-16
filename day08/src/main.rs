fn main() {
    let input = include_str!("./input.txt");
    println!("{}", p1(input));
    println!("{}", p2(input));
}
type Grid = Vec<Vec<u32>>;

fn parse_input(input: &str) -> Grid {
    let mut grid: Grid = vec![];
    for line in input.lines() {
        let mut row: Vec<u32> = vec![];
        for char in line.chars() {
            row.push(char.to_digit(10).unwrap())
        }
        grid.push(row);
    }
    grid
}

fn p1(input: &str) -> i32 {
    let grid = parse_input(input);
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut count = (((rows + cols) * 2) - 4) as i32;
    for row in 1..(rows - 1) {
        for col in 1..(cols - 1) {
            if is_visible(Tree { row, col }, &grid) {
                // println!("{} {} - {}", row, col, grid[row][col]);
                count += 1;
            }
        }
    }

    count
}

fn p2(input: &str) -> i32 {
    let grid = parse_input(input);
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut score = 0;
    for row in 1..(rows - 1) {
        for col in 1..(cols - 1) {
            let cur_score = get_scenic_score(Tree { row, col }, &grid);
            // println!("{} {} - {}: {}", row, col, grid[row][col], cur_score);
            if cur_score > score {
                score = cur_score;
            }
        }
    }

    score
}

struct Tree {
    row: usize,
    col: usize,
}

fn get_scenic_score(t: Tree, grid: &Grid) -> i32 {
    let cur_tree = grid[t.row][t.col];
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut score = 1;
    let mut local_score = 1;
    for (idx, col) in (0..(t.col as i32)).rev().enumerate() {
        if idx != 0 {
            local_score += 1;
        }
        // Left
        if grid[t.row][col as usize] >= cur_tree {
            break;
        }
    }
    score *= local_score;
    local_score = 1;
    for (idx, row) in (0..(t.row as i32)).rev().enumerate() {
        if idx != 0 {
            local_score += 1;
        }
        // top
        if grid[row as usize][t.col] >= cur_tree {
            break;
        }
    }
    score *= local_score;
    local_score = 1;
    for (idx, col) in ((t.col + 1)..cols).enumerate() {
        if idx != 0 {
            local_score += 1;
        }
        if grid[t.row][col] >= cur_tree {
            break;
        }
    }
    score *= local_score;
    local_score = 1;
    for (idx, row) in ((t.row + 1)..rows).enumerate() {
        if idx != 0 {
            local_score += 1;
        }
        if grid[row][t.col] >= cur_tree {
            break;
        }
    }
    score *= local_score;
    score
}

fn is_visible(t: Tree, grid: &Grid) -> bool {
    let cur_tree = grid[t.row][t.col];
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut visible_count = 0;
    for col in (0..(t.col as i32)).rev() {
        // Left
        if grid[t.row][col as usize] >= cur_tree {
            visible_count += 1;
            break;
        }
    }
    for row in (0..(t.row as i32)).rev() {
        // top
        if grid[row as usize][t.col] >= cur_tree {
            visible_count += 1;
            break;
        }
    }
    for col in (t.col + 1)..cols {
        if grid[t.row][col] >= cur_tree {
            visible_count += 1;
            break;
        }
    }
    for row in (t.row + 1)..rows {
        if grid[row][t.col] >= cur_tree {
            visible_count += 1;
            break;
        }
    }
    visible_count != 4
}
