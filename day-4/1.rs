use std::fs::read_to_string;

fn range_seperate(range: &str) -> (i32, i32) {
    fn convert(value: &str) -> i32 {
        value.parse::<i32>().unwrap()
    }
    let range_vec: Vec<&str> = range.split("-").collect();
    (convert(range_vec[0]), convert(range_vec[1]))
}

fn main() {
    let content: String = read_to_string("input.txt").unwrap();
    let initial_split: Vec<&str> = content.split("\n").collect();
    let mut count: i32 = 0;
    initial_split.iter().for_each(|x| {
        let x_owned: &str = x.to_owned();
        let r: Vec<&str> = x_owned.split(",").collect();
        let (range1, range2) = (r[0], r[1]);

        let (r_1_1, r_1_2) = range_seperate(range1);
        let (r_2_1, r_2_2) = range_seperate(range2);
        if (r_1_1 >= r_2_1 && r_1_2 <= r_2_2) || (r_2_1 >= r_1_1 && r_2_2 <= r_1_2) {
            count += 1;
        }
    });
    println!("{count}");
}
