use std::fs::read_to_string;

fn get_content() -> String {
    read_to_string("input.txt").unwrap()
}

fn check_duplicate(check_vec: &Vec<char>) -> bool {
    let mut flag = false;
    let mut cross_vec: Vec<char> = Vec::new();
    for element in check_vec {
        if cross_vec.contains(&element) {
            flag = true
        };
        cross_vec.push(element.to_owned());
    }
    flag
}
fn main() {
    let content: String = get_content();
    let mut bytes: Vec<char> = Vec::new();
    let mut counter: i32 = 0;
    for letter in content.chars() {
        if bytes.len() == 14 {
            bytes.remove(0);
        }
        bytes.push(letter);
        counter += 1;
        if !check_duplicate(&bytes) && bytes.len() >= 14 {
            println!("{counter}");
            break;
        }
    }
}
