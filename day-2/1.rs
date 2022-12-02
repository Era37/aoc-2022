use std::collections::HashMap;
use std::fs::read_to_string;

/*
Rock: A
Paper: B
Scissors: C
*/

fn main() {
    let content: String = read_to_string("input.txt").unwrap();
    let content_split: Vec<&str> = content.split("\n").collect();

    let value_map: HashMap<&str, i32> = HashMap::from([
        ("A X", 4),
        ("A Y", 8),
        ("A Z", 3),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 7),
        ("C Y", 2),
        ("C Z", 6),
    ]);
    let mut score_total: i32 = 0;
    for game in content_split {
        let score = value_map.get(game).unwrap();
        score_total += score.to_owned();
    }
    println!("{score_total}")
}
