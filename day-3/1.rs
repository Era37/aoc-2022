use std::fs::read_to_string;

fn comapare(first: &str, second: &str) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();
    first.chars().for_each(|c: char| {
        let part2_chars: Vec<char> = second.chars().collect();
        if part2_chars.contains(&c) && !result.contains(&c) {
            result.push(c);
        }
    });
    result
}

fn priority(character: char) -> u8 {
    let letter_ranks: usize = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .position(|x| x == character)
        .unwrap()
        + 1; // returns the index + 1 representing the priority
    return letter_ranks as u8;
}
fn main() {
    let content: String = read_to_string("input.txt").unwrap();
    let content_split: Vec<&str> = content.split("\n").collect();

    let mut total: i32 = 0;
    for entry in content_split.chunks_exact(3) {
        let owned_entry = entry.to_owned();
        let mut final_char: char = char::default();
        let (compared1, compared2) = (
            comapare(owned_entry[0], owned_entry[1]),
            comapare(owned_entry[0], owned_entry[2]),
        );
        for char in compared1 {
            if compared2.contains(&char) {
                final_char = char;
            }
        }
        total += i32::from(priority(final_char));
    }
    println!("{total}")
}
