pub fn part_one(rucksack_list: String) -> i32 {
    let mut total = 0;
    for rucksack in rucksack_list.split("\n") {
        // Seperate into two 'compartments'
        let (compartment_a, compartment_b) = seperate(rucksack);
        // Compare the compartments & find matching letter (if any(?))
        let character = compare(compartment_a, compartment_b);
        // parse the letter into a priority number
        // add number to total
        total += convert_priority(character);
    }
    total
}

fn seperate(rucksack: &str) -> (String, String) {
    let compartment_length = rucksack.len() / 2;
    let char_list = rucksack.chars().collect::<Vec<char>>();
    return (
        char_list[0..compartment_length].into_iter().collect(),
        char_list[compartment_length..rucksack.len()]
            .into_iter()
            .collect(),
    );
}

fn compare(compartment_a: String, compartment_b: String) -> char {
    // Iterate over each letter in a & b, and find a match
    for char_a in compartment_a.chars() {
        for char_b in compartment_b.chars() {
            if char_a == char_b {
                return char_a;
            }
        }
    }
    panic!()
}

fn convert_priority(character: char) -> i32 {
    let initial_conversion = u32::from(character);
    if initial_conversion < 0x61 {
        return ((initial_conversion - 0x40) + 26) as i32;
    } else {
        return (initial_conversion - 0x60) as i32;
    }
}

pub fn part_two(rucksack_list: String) -> i32 {
    let mut total = 0;

    for rucksack_group in rucksack_list
        .split("\n")
        .collect::<Vec<&str>>()
        .chunks_exact(3)
    {
        let shared_char = compare_rucksack_group(rucksack_group);
        total += convert_priority(shared_char);
    }

    total
}

fn compare_rucksack_group(rucksack_group: &[&str]) -> char {
    for rucksack_char_a in rucksack_group[0].chars() {
        for rucksack_char_b in rucksack_group[1].chars() {
            for rucksack_char_c in rucksack_group[2].chars() {
                if rucksack_char_a == rucksack_char_b && rucksack_char_b == rucksack_char_c {
                    return rucksack_char_a;
                }
            }
        }
    }
    panic!()
}

#[cfg(test)]
fn test_string() -> String {
    String::from(
        "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
    )
}

#[test]
fn test_part_one() {
    let test_string = test_string();
    assert_eq!(part_one(test_string), 157)
}

#[test]
fn test_part_two() {
    let test_string = test_string();
    assert_eq!(part_two(test_string), 70)
}
