use std::{fmt::Display, fs::File, io::Read, time::Instant};

pub mod calorie_counting;
pub mod rock_paper_scissors;

pub fn get_input(filename: &str) -> String {
    let mut input: String = String::new();
    File::open(format!("./inputs/{filename:}"))
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    input
}

pub fn run_solution<T>(filename: &str, f: fn(String) -> T)
where
    T: Display,
{
    let input = get_input(filename);
    let now = Instant::now();
    let answer = f(input);
    let elapsed_time = now.elapsed();
    println!("The answer is: {answer:}; took {elapsed_time:?}");
}

fn main() {
    run_solution("2.txt", rock_paper_scissors::part_two)
}
