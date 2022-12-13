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
