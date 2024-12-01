pub fn solve_part1(input: &str) -> String {
    let mut array1: Vec<i32> = Vec::new();
    let mut array2: Vec<i32> = Vec::new();

    for line in input.lines() {
        if !line.trim().is_empty() {
            let numbers: Vec<&str> = line.split_whitespace().collect();
            let num1: i32 = numbers[0].parse().expect("invalid num");
            let num2: i32 = numbers[1].parse().expect("invalid num");
            array1.push(num1);
            array2.push(num2);
        }
    }

    array1.sort();
    array2.sort();

    let pair_distance: Vec<i32> = array1
        .iter()
        .zip(array2.iter())
        .map(|(a, b)| (a - b).abs())
        .collect();

    let total_sum: i32 = pair_distance.iter().sum();

    total_sum.to_string() // 2192892
}

pub fn solve_part2(input: &str) -> String {
    let mut array1: Vec<i64> = Vec::new();
    let mut array2: Vec<i64> = Vec::new();

    for line in input.lines() {
        if !line.trim().is_empty() {
            let numbers: Vec<&str> = line.split_whitespace().collect();
            let num1: i64 = numbers[0].parse().expect("invalid num");
            let num2: i64 = numbers[1].parse().expect("invalid num");
            array1.push(num1);
            array2.push(num2);
        }
    }

    array1.sort();
    array2.sort();

    let mut total_sum = 0;

    for i in 0..array1.len() {
        let mut count = 0;

        for j in 0..array2.len() {
            if array2[j] == array1[i] {
                count += 1;
            }
        }
        total_sum += array1[i] * count;
    }

    total_sum.to_string() // 22962826
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"
        1 2
        3 1
        4 1
        "#;
        assert_eq!(solve_part1(input), "4");
    }
}
