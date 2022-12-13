use std::{fs::File, io::Read};

pub mod calorie_counting;

fn calorie_counting_answer() {
    let mut input: String = String::new();
    File::open("./inputs/1.txt")
    .unwrap()
    .read_to_string(&mut input).unwrap();
    println!("{}", calorie_counting::calorie_counting(input))
}

fn main() {
    calorie_counting_answer()
}
