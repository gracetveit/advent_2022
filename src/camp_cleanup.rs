pub fn part_one(pair_listings: String) -> i32 {
    let mut total = 0;
    for pair in pair_listings.split("\n") {
        //Splits the pair into assigment 1, assigment 2
        let assignemnts: Vec<&str> = pair.split(",").collect();
        if does_contain(assignemnts) {
            total += 1;
        }
    }
    total
}

fn does_contain(assignments: Vec<&str>) -> bool {
    // Parse assignments, to have a smallest and largest value
    let (assignment_a, assignment_b) = parse_assignments(assignments);
    // If the smallest of A is bigger than the smallest of B
    // AND
    // The largest of A is smaller than the biggest of B
    // OR
    // The smallest of B is bigger than the smallest of A
    // AND
    // the largest of B is smaller than the biggest of A
    (assignment_a[0] >= assignment_b[0] && assignment_a[1] <= assignment_b[1])
        || (assignment_b[0] >= assignment_a[0] && assignment_b[1] <= assignment_a[1])
}

fn parse_assignments(assignments: Vec<&str>) -> ([i32; 2], [i32; 2]) {
    (
        parse_single_assignment(assignments[0]),
        parse_single_assignment(assignments[1]),
    )
}

fn parse_single_assignment(assignment: &str) -> [i32; 2] {
    let mut parsed_assignment = [0, 0];

    let split_assigment = assignment.split("-").collect::<Vec<&str>>();

    parsed_assignment[0] = split_assigment[0].parse::<i32>().unwrap() as i32;
    parsed_assignment[1] = split_assigment[1].parse::<i32>().unwrap() as i32;

    parsed_assignment
}

fn any_contain(assignments: Vec<&str>) -> bool {
    let (assignment_a, assignment_b) = parse_assignments(assignments);
    !(assignment_a[1] < assignment_b[0] ||
    assignment_b[1] < assignment_a[0])
}

pub fn part_two(pair_listings: String) -> i32 {
    let mut total = 0;
    for pair in pair_listings.split("\n") {
        let assignments: Vec<&str> = pair.split(",").collect();
        if any_contain(assignments) {
            total += 1;
        }
    }
    total
}

#[cfg(test)]
fn test_string() -> String {
    String::from(
        "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8",
    )
}

#[test]
fn test_part_one() {
    let test_string = test_string();
    assert_eq!(part_one(test_string), 2)
}

#[test]
fn test_part_two() {
    let test_string = test_string();
    assert_eq!(part_two(test_string), 4)
}
