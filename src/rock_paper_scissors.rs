pub fn rock_paper_sciccors(game_list: String) -> i32 {
    let mut total = 0;
    for game in game_list.split("\n") {
        total += score(game);
    }
    total
}

fn score(data: &str) -> i32 {
    let characters = data.chars().collect::<Vec<char>>();
    let (p2, p1) = (characters[0], characters[2]);
    victory_value(p1, p2) + chosen_character_value(p1)
}

fn victory_value(p1: char, p2: char) -> i32 {
    // A/X: Rock
    // B/Y: Paper
    // C/Z: Scissors
    match (p1, p2) {
        ('X', 'A') => 3,
        ('X', 'B') => 0,
        ('X', 'C') => 6,
        ('Y', 'A') => 6,
        ('Y', 'B') => 3,
        ('Y', 'C') => 0,
        ('Z', 'A') => 0,
        ('Z', 'B') => 6,
        ('Z', 'C') => 3,
        (_, _) => panic!("Invalid Character: {p1:}, {p2:}"),
    }
}

fn chosen_character_value(player_one_char: char) -> i32 {
    match player_one_char {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!(),
    }
}

pub fn part_two(game_list: String) -> i32 {
    let mut total = 0;
    for game in game_list.split("\n") {
        total += part_two_score(game);
    }
    total
}

fn part_two_score(game: &str) -> i32 {
    let characters = game.chars().collect::<Vec<char>>();
    let p2 = match characters[0] {
        'A' => RPS::Rock,
        'B' => RPS::Paper,
        'C' => RPS::Sciccors,
        _ => panic!(),
    };
    // Win: +6, Tie: +0, Loss: +0,
    // Rock: +1, Paper: +2, Scissors: +3
    // X: Lose
    // Y: Tie
    // Z: Win
    match (p2, characters[2]) {
        (RPS::Rock, 'X') => 0 + 3,
        (RPS::Rock, 'Y') => 3 + 1,
        (RPS::Rock, 'Z') => 6 + 2,
        (RPS::Paper, 'X') => 0 + 1,
        (RPS::Paper, 'Y') => 3 + 2,
        (RPS::Paper, 'Z') => 6 + 3,
        (RPS::Sciccors, 'X') => 0 + 2,
        (RPS::Sciccors, 'Y') => 3 + 3,
        (RPS::Sciccors, 'Z') => 6 + 1,
        (_, _) => panic!(),
    }
    // todo!()
}

enum RPS {
    Rock,
    Paper,
    Sciccors,
}

#[cfg(test)]
fn test_string() -> String {
    String::from(
        "A Y
B X
C Z",
    )
}

#[test]
fn test_rock_paper_sciccors() {
    let test_string = test_string();
    assert_eq!(rock_paper_sciccors(test_string), 15)
}

#[test]
fn test_part_two() {
    let test_string = test_string();
    assert_eq!(part_two(test_string), 12)
}
