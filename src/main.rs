use std::{fs::File, io::Read, time::Instant};

use calorie_counting::{calorie_counting, second_solution};

pub mod calorie_counting;

fn get_input(filename: &str) ->String {
    let mut input: String = String::new();
    File::open(filename)
    .unwrap()
    .read_to_string(&mut input).unwrap();
    input
}

fn calorie_counting_answer_1() {
    let input = get_input("./inputs/1.txt");
    let now = Instant::now();
    let answer = calorie_counting(input);
    let elapsed_time = now.elapsed();
    println!("{}, {:?}", answer, elapsed_time);
}

fn calorie_counting_answer_2() {
    let input = get_input("./inputs/1.txt");
    let now = Instant::now();
    let answer = second_solution(input);
    let elapsed_time = now.elapsed();
    println!("{}, {:?}", answer, elapsed_time);
}

fn main() {
    calorie_counting_answer_1();
    calorie_counting_answer_2();
}
