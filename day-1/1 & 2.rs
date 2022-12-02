use std::fs;

fn read_file() -> String {
    fs::read_to_string("./elf_food.txt").unwrap()
}

fn iterate_content(content: String) -> Vec<Elf> {
    /*
        Employs a double loop to
        calculate the total amount
        of elf food
    */
    let mut elves: Vec<Elf> = Vec::new();
    let raw_elf_split: Vec<&str> = content.split("\n\n").collect(); // Each Vec entry represents an elf
    for elf in raw_elf_split {
        let elf_food_pieces: Vec<&str> = elf.split("\n").collect();
        let mut elf_calories: i32 = 0;
        for food_piece in elf_food_pieces {
            elf_calories += food_piece.parse::<i32>().unwrap();
        }
        let elf: Elf = Elf {
            food_amount: elf_calories,
        };
        elves.push(elf);
    }
    elves
}
fn main() {
    let content: String = read_file();
    let elves: Vec<Elf> = iterate_content(content);

    let mut most_food: i32 = 0;
    for elf in &elves {
        /* Gets the elf with the most food */
        let food_amount = elf.food_amount;
        if food_amount > most_food {
            most_food = food_amount;
        }
    }
    println!("{}, {}", get_top_3(elves), most_food);
}

fn get_top_3(mut elves: Vec<Elf>) -> i32 {
    elves.sort_by_key(|n| std::i32::MAX - n.food_amount);
    elves[0].food_amount + elves[1].food_amount + elves[2].food_amount
}

#[derive(Debug)]
struct Elf {
    food_amount: i32,
}
