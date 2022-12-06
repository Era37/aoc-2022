use std::collections::HashMap;
use std::fs::read_to_string;

fn read_input() -> String {
    read_to_string("input.txt").unwrap()
}

fn convert_string(raw_string: String) -> (u8, u8, u8) {
    let ss: Vec<&str> = raw_string.split_whitespace().collect();
    fn convert(s: &str) -> u8 {
        s.parse::<u8>().unwrap()
    }
    (convert(ss[0]), convert(ss[1]), convert(ss[2]))
}
fn main() {
    let content: String = read_input();
    let content_lines: Vec<&str> = content.lines().collect();
    let refined_content: Vec<String> = content_lines
        .iter()
        .map(|x| {
            let e = x
                .to_string()
                .replace("move ", "")
                .replace("from ", "")
                .replace(" to", "");
            return e;
        })
        .collect();
    let mut values: HashMap<u8, Vec<char>> = HashMap::from([
        (1, Vec::from(['Q', 'M', 'G', 'C', 'L'])),
        (2, Vec::from(['R', 'D', 'L', 'C', 'T', 'F', 'H', 'G'])),
        (3, Vec::from(['V', 'J', 'F', 'N', 'M', 'T', 'W', 'R'])),
        (4, Vec::from(['J', 'F', 'D', 'V', 'Q', 'P'])),
        (5, Vec::from(['N', 'F', 'M', 'S', 'L', 'B', 'T'])),
        (6, Vec::from(['R', 'N', 'V', 'H', 'C', 'D', 'P'])),
        (7, Vec::from(['H', 'C', 'T'])),
        (8, Vec::from(['G', 'S', 'J', 'V', 'Z', 'N', 'H', 'P'])),
        (9, Vec::from(['Z', 'F', 'H', 'G'])),
    ]);
    for instruction in refined_content {
        let (amount, from, to) = convert_string(instruction);
        for _i in 1..=amount {
            let removed: char = values.get_mut(&from).unwrap().pop().unwrap();
            values.get_mut(&to).unwrap().push(removed);
        }
    }
    for _i in 1..=9 {
        println!("{:?}", values.get(&_i).unwrap().last());
    }
}
