pub fn calorie_counting(calorie_list: String) -> i32 {
    // Parse string into seperate list of things
    parse_string(calorie_list)
        .into_iter()
        .reduce(|acc, cur| {
            if cur >= acc {
                return cur
            }
            return acc
        }).unwrap()
}

fn parse_string(calorie_list: String) -> Vec<i32> {
    calorie_list.split("\n\n").map(|x| {
        x.split("\n")
            .map(|y| {
                y.parse::<i32>().unwrap()
            })
            .sum::<i32>()
    }).collect::<Vec<i32>>()
}

pub fn second_solution(calorie_list: String) -> i32 {
    let mut largest_sum = 0;
    let mut current_sum = 0;
    for x in calorie_list.split("\n") {
        if x == "" {
            if current_sum > largest_sum {
                largest_sum = current_sum;
            }
            current_sum = 0;
        } else {
            current_sum += x.parse::<i32>().unwrap()
        }
    }
    largest_sum
}

pub fn part_two(calorie_list: String) -> i32 {
    let mut largest_sums = [0, 0, 0];
    let mut current_sum = 0;

    for x in calorie_list.split("\n") {
        if x == "" {
            replace_lowest_item(&mut largest_sums, current_sum);
            current_sum = 0;
        } else {
            current_sum += x.parse::<i32>().unwrap()
        }
    }
    replace_lowest_item(&mut largest_sums, current_sum);
    largest_sums.into_iter().sum()
}

fn replace_lowest_item(sum_list: &mut [i32; 3], x: i32) {
    sum_list.sort();
    // find the lowest item, and get index
    if sum_list[0] < x {
        sum_list[0] = x
    }
}

#[cfg(test)]
fn test_string() -> String {
    String::from(
        "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
    )
}

#[test]
fn test_parse_string() {
    let test_string = test_string();
    assert_eq!(parse_string(test_string), vec![6000, 4000, 11000, 24000, 10000])
}

#[test]
fn test_calorie_counting() {
    let test_string = test_string();
    assert_eq!(calorie_counting(test_string), 24000)
}

#[test]
fn test_second_part() {
    let test_string = test_string();
    assert_eq!(part_two(test_string), 45000)
}
