use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let value_map: HashMap<&str, u8> = HashMap::from([
        ("A X", 3),
        ("A Y", 4),
        ("A Z", 8),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 2),
        ("C Y", 6),
        ("C Z", 7),
    ]);
    let file_string: String = read_to_string("input.txt").unwrap();
    let collected: Vec<&str> = file_string.split("\n").collect();
    let mut score: i32 = 0;
    for game in collected {
        score += i32::from(value_map.get(game).unwrap().to_owned());
    }
    println!("{score}");
}
