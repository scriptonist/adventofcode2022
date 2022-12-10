fn main() {
    let input = include_str!("./input.txt");
    println!("{}", p1(input));
    println!("{}", p2(input));
}

fn p1(input: &str) -> i32 {
    let mut score = 0;
    for line in input.lines() {
        let players = line.trim().split(' ').collect::<Vec<&str>>();
        score += get_score(players[0], players[1]);
    }

    score
}

fn p2(input: &str) -> i32 {
    let mut score = 0;
    for line in input.lines() {
        let move_map = line.trim().split(' ').collect::<Vec<&str>>();
        let player1 = move_map[0];
        let player2 = match move_map[1] {
            "X" => get_move(player1, Res::Loose),
            "Y" => get_move(player1, Res::Draw),
            "Z" => get_move(player1, Res::Win),
            _ => unreachable!(),
        };
        // println!("{}{}", player1, player2);
        score += get_score(player1, player2.as_str());
    }

    score
}
enum Res {
    Win,
    Loose,
    Draw,
}

fn get_move(player1: &str, want_res: Res) -> String {
    match want_res {
        Res::Win => match player1 {
            "A" => "Y".to_string(),
            "B" => "Z".to_string(),
            "C" => "X".to_string(),
            _ => unreachable!(),
        },
        Res::Loose => match player1 {
            "A" => "Z".to_string(),
            "B" => "X".to_string(),
            "C" => "Y".to_string(),
            _ => unreachable!(),
        },
        Res::Draw => match player1 {
            "A" => "X".to_string(),
            "B" => "Y".to_string(),
            "C" => "Z".to_string(),
            _ => unreachable!(),
        },
    }
}
fn get_score(player1: &str, player2: &str) -> i32 {
    match format!("{}{}", player1, player2).as_str() {
        // Rock - A,X
        // Paper - B,Y
        // Scissors - C,Z
        "AZ" => 3,
        "BX" => 1,
        "CY" => 2,

        "AX" => 3 + 1,
        "BY" => 3 + 2,
        "CZ" => 3 + 3,

        "CX" => 6 + 1,
        "AY" => 6 + 2,
        "BZ" => 6 + 3,
        _ => unimplemented!(),
    }
}
