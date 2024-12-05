advent_of_code::solution!(3);

use regex::Regex;
use std::{env, fs};

pub fn part_one(input: &str) -> Option<i32> {
    let data: Vec<Vec<(i32, i32)>> = parse_data_lists(input);
    let mut sum: i32 = 0;
    for x in data {
        for tuple in x {
            sum += tuple.0 * tuple.1;
            println!("sum: {} from {} and {}", sum, tuple.0, tuple.1);
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<i32> {
    let data: Vec<Vec<(i32, i32)>> = parse_data_lists(input);
    let mut sum: i32 = 0;
    for x in data {
        for tuple in x {
            sum += tuple.0 * tuple.1;
            println!("sum: {} from {},{}", sum, tuple.0, tuple.1);
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let cwd = env::current_dir().unwrap();
        let filepath = cwd.join("data").join("examples").join("03.2.txt");
        println!("filepath: {filepath:?}");
        let f = fs::read_to_string(filepath);
        let result = part_two(f.unwrap().as_str());
        assert_eq!(result, Some(48));
    }
}

fn parse_data_lists(input: &str) -> Vec<Vec<(i32, i32)>> {
    filter_donts(input).unwrap().iter()
        .map(|line| parse_line(line))
        .filter(|line| line.is_some())
        .map(|line| line.unwrap())
        .collect()
    //Part 2: 107516772 (86.8ms)
}

fn parse_line(input: &str) -> Option<Vec<(i32, i32)>> {
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let matches: Vec<&str> = mul_regex.find_iter(input).map(|m| m.as_str()).collect();
    let mut parsed_numbers: Vec<(i32, i32)> = Vec::new();
    for full_match in matches {
        let trimmed = &full_match[4..full_match.len() - 1];
        let numbers: Vec<&str> = trimmed.split(',').collect();
        if numbers.len() != 2 {
            println!("found more numbers than expected...");
            continue;
        }
        let x = numbers[0].parse::<i32>().unwrap();
        let y = numbers[1].parse::<i32>().unwrap();
        parsed_numbers.push((x, y));
    }
    Some(parsed_numbers)
}

fn filter_donts(input: &str) -> Option<Vec<&str>> {
    let mut split = input.split("don't()");
    let first_part = split.next().unwrap();

    let mut filtered_parts: Vec<&str> = vec![first_part];

    filtered_parts.extend(
        split
            .filter(|&x| !x.trim().is_empty() && x.contains("do()"))
            .flat_map(|x| x.trim().split("do()").skip(1)),
    );

    println!("Found {} parts in filtered input", filtered_parts.len());
    for x in filtered_parts.clone() {
        println!("{:?}", x);
    }

    // Check if any filtered parts exist beyond the first part
    if filtered_parts.len() > 1 {
        Some(filtered_parts)
    } else {
        None
    }
}
