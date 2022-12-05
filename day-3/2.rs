use std::fs::read_to_string;

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
    let mut total_priority: i32 = 0;
    for entry in content_split {
        /*
            Creates a vector (repeats) containing
            all of the repeating values from the two
            comparments (represented by part1 & part2)
        */
        let (part1, part2) = entry.split_at(entry.len() / 2);
        let mut repeat: char = char::default();
        part1.chars().for_each(|x: char| {
            let c: Vec<char> = part2.chars().collect();
            if c.contains(&x) {
                // adds to repeats vec if contained by second compartment (with duplicate checker)
                repeat = x;
            }
        });

        total_priority += i32::from(priority(repeat)); //
    }
    println!("{total_priority}");
}
