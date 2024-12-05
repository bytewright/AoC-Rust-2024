advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let number_lists = parse_number_lists(input);
    let mut sorted_list1 = number_lists.0.clone();
    let mut sorted_list2 = number_lists.1.clone();

    sorted_list1.sort();
    sorted_list2.sort();

    if sorted_list1.len() != sorted_list2.len() {
        return None;
    }

    let mut distance = 0;
    for i in 0..sorted_list1.len() {
        let left_num = sorted_list1.iter().nth(i).unwrap();
        let right_num = sorted_list2.iter().nth(i).unwrap();
        distance += (left_num - right_num).abs();
    }
    Some(distance)
}

pub fn part_two(input: &str) -> Option<i32> {
    let number_lists = parse_number_lists(input);
    if number_lists.0.len() != number_lists.1.len() {
        return None;
    }

    let mut distance = 0;
    for i in 0..number_lists.0.len() {
        let left_num = *number_lists.0.iter().nth(i).unwrap();
        let right_num = number_lists
            .1
            .iter()
            .filter(|&&num| num == left_num)
            .count();
        distance += left_num * (right_num as i32);
    }
    Some(distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 11);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 31);
    }
}

fn parse_number_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let lines: Vec<&str> = input
        .trim()
        .split('\n')
        .filter(|line| !line.trim().is_empty())
        .collect();
    let (list1, list2): (Vec<i32>, Vec<i32>) = lines
        .iter()
        .map(|line| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .filter_map(|token| token.parse::<i32>().ok()) // Parse only valid integers
                .collect();
            (nums[0], nums[1])
        })
        .unzip();
    (list1, list2)
}
