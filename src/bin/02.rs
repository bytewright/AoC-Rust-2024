use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let levels_list = parse_levels_lists(input);
    let mut valid_count = 0;
    for x in levels_list {
        // true flag is to avoid 'problem dampener' for solution 2
        let is_valid = check_level_report(true, x);
        if is_valid {
            valid_count += 1;
        }
    }
    Some(valid_count)
}

fn check_level_report(report_rerun: bool, levels_report: Vec<i32>) -> bool {
    if levels_report.is_empty() {
        false
    } else {
        let direction = check_level_direction(levels_report.clone());
        let distance = check_level_distance(levels_report.clone());
        if direction.1 && distance {
            println!("found safe {} report: {:?}", direction.0, levels_report);
            true
        } else if !report_rerun {
            println!("found unsafe {} report, retry with permutations: {:?}", direction.0, levels_report);
            let vec = get_subsets_without_one(levels_report.clone());
            vec.into_iter().map(|x| check_level_report(true, x)).any(|x| x)
        } else {
            false
        }
    }
}
fn get_subsets_without_one(levels_report: Vec<i32>) -> Vec<Vec<i32>> {
    (0..levels_report.len())
        .map(|skip_index| {
            levels_report
                .iter()
                .enumerate()
                .filter(|&(index, _)| index != skip_index)
                .map(|(_, &val)| val)
                .collect()
        })
        .collect()
}

fn check_level_direction(levels_report: Vec<i32>) -> (&'static str, bool) {
    if levels_report.len() < 2 {
        return ("too short", false);
    }

    // Check if all reports are the same
    if levels_report.iter().all(|&x| x == levels_report[0]) {
        return ("sameness", false);
    }

    let batched_reports = levels_report.iter().tuple_windows();
    let is_decreasing = batched_reports.clone().all(|(a, b)| *a > *b);
    let is_increasing = batched_reports.clone().all(|(a, b)| *a < *b);
    if is_increasing {
        ("increase", true)
    } else if is_decreasing {
        ("descrease", true)
    } else {
        ("unsafe", false)
    }
}

fn check_level_distance(levels_report: Vec<i32>) -> bool {
    levels_report
        .iter()
        .tuple_windows()
        .map(|(a, b)| (a - b).abs())
        .all(move |x| x > 0 && x < 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    let levels_list = parse_levels_lists(input);
    let mut valid_count = 0;
    for x in levels_list {
        let is_valid = check_level_report(false, x);
        if is_valid {
            valid_count += 1;
        }
    }
    Some(valid_count)
}

fn parse_levels_lists(input: &str) -> Vec<Vec<i32>> {
    let lines: Vec<&str> = input
        .trim()
        .split('\n')
        .filter(|line| !line.trim().is_empty())
        .collect();
    let list: Vec<Vec<i32>> = lines
        .iter()
        .map(|line| {
            let levels = line
                .split_whitespace()
                .filter_map(|token| token.parse::<i32>().ok())
                .collect::<Vec<i32>>();
            levels
        })
        .collect::<Vec<Vec<i32>>>();
    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
