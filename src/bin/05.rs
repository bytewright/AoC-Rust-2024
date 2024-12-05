use regex::Regex;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let data = load_input_data(input);
    let rules = data.0;
    let input_lines = data.1;
    let mut correct_inputs = 0;
    let mut checksum = 0;
    for (i, number_sequence) in input_lines.iter().enumerate() {
        if verify_input(number_sequence.clone(), rules.clone()) {
            correct_inputs += 1;
            let middle_index = number_sequence.len() / 2;
            println!(
                "line {} is valid: {:?}, middle index  {}",
                i, number_sequence, middle_index
            );
            let x = number_sequence.iter().nth(middle_index).unwrap();
            checksum += x;
        }
    }
    println!(
        "Found {} correct input lines. checksum: {}",
        correct_inputs, checksum
    );
    Some(checksum)
}

fn verify_input(number_sequence: Vec<u32>, rules: Vec<(u32, u32)>) -> bool {
    for rule in rules {
        let first: u32 = rule.0;
        let second: u32 = rule.1;
        let first_index = number_sequence.iter().position(|&x| x == first);
        let second_index = number_sequence.iter().position(|&x| x == second);
        if let (Some(first_pos), Some(second_pos)) = (first_index, second_index) {
            if first_pos >= second_pos {
                return false;
            }
        }
    }
    true
}

fn load_input_data(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let re = Regex::new(r"\n\s*\n|\r\n\s*\r\n").unwrap();
    let re_single = Regex::new(r"\n|\r\n").unwrap();
    let mut split = re.split(input);
    let rules: &str = split.next().unwrap();
    let parsed_rules = re_single
        .split(rules)
        .map(|s| s.split('|').collect::<Vec<&str>>())
        .map(|pair| {
            let mut x = pair.iter().map(|s| s.parse::<u32>().unwrap());
            (x.next().unwrap(), x.next().unwrap())
        })
        .collect::<Vec<(u32, u32)>>();
    println!("parsed_rules: {}", parsed_rules.len());
    let input_lines: &str = split.next().unwrap();
    let input_lists = re_single
        .split(input_lines)
        .map(|s| {
            s.split(',')
                .filter(|s| !s.is_empty())
                .map(|s| {
                    let result = s.parse::<u32>();
                    return if result.is_ok() {
                        result.unwrap()
                    } else {
                        println!("unwrap_err {}", result.unwrap_err());
                        0
                    };
                })
                .collect::<Vec<u32>>()
        })
        .filter(|s| !s.is_empty())
        .collect::<Vec<Vec<u32>>>();
    println!("input_lists: {}", input_lists.len());
    (parsed_rules, input_lists)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
