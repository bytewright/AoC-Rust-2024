use regex::Regex;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let (pos_x, pos_y, mut area) = load_area(input);
    let mut heading = get_heading(&area, pos_x, pos_y).unwrap();
    let mut cur_x = pos_x;
    let mut cur_y = pos_y;
    loop {
        println!("cur_x {}, cur_y {}, heading {}", cur_x, cur_y, heading);
        let (new_x, new_y) = move_guard(cur_x, cur_y, heading, &mut area);
        cur_x = new_x;
        cur_y = new_y;
        heading = get_heading(&area, cur_x, cur_y).unwrap();
        if heading == 4
            || cur_x == 0 && heading == 3
            || cur_x == area[0].len() as u32 && heading == 1
            || cur_y == 0 && heading == 0
            || cur_y == area.len() as u32 && heading == 2
        {
            area[cur_y as usize][cur_x as usize] = 'X';
            break;
        }
    }
    println!("guard is out!");
    for line in &area {
        let row: String = line.iter().collect();
        println!("{}", row);
    }
    let mut visited_tiles = 0;
    for line in &area {
        for &c in line {
            if c == 'X' {
                visited_tiles += 1;
            }
        }
    }
    println!("Visited tiles: {}", visited_tiles);
    Some(visited_tiles)
}

fn move_guard(cur_x: u32, cur_y: u32, heading: u32, area: &mut Vec<Vec<char>>) -> (u32, u32) {
    area[cur_y as usize][cur_x as usize] = 'X';
    let mut new_heading = heading;
    let (mut new_x, mut new_y) = match heading {
        0 => (Some(cur_x), cur_y.checked_sub(1)),
        1 => (Some(cur_x + 1), Some(cur_y)),
        2 => (Some(cur_x), Some(cur_y + 1)),
        3 => (cur_x.checked_sub(1), Some(cur_y)),
        _ => (None, None),
    };

    // Bounds checking
    if new_x.is_none() || new_y.is_none() {
        println!("guard is at border, headed outside");
        return (cur_x, cur_y);
    }
    let is_in_bounds = new_y.unwrap() < area.len() as u32 && new_x.unwrap() < area[0].len() as u32;

    let target_char = if is_in_bounds {
        println!("move_guard: new_x {}, new_y {}", new_x.unwrap(), new_y.unwrap());
        area[new_y.unwrap() as usize][new_x.unwrap() as usize]
    } else {
        // If out of bounds, return current position
        println!("guard is at border, headed outside");
        return (cur_x, cur_y);
    };

    if target_char == '#' {
        // Rotate 90 degrees clockwise
        new_heading = (heading + 1) % 4;
        (new_x, new_y) = (Some(cur_x), Some(cur_y))
    }
    let heading_char = match new_heading {
        0 => '^', // North
        1 => '>', // East
        2 => 'v', // South
        3 => '<', // West
        _ => 'O', // Default case
    };

    area[new_y.unwrap() as usize][new_x.unwrap() as usize] = heading_char;

    (new_x.unwrap(), new_y.unwrap())
}

fn is_border(cur_x: u32, cur_y: u32, heading: u32, field_dimensions: (u32, u32)) -> bool {
    if cur_x == 0 && heading == 1 || cur_x == field_dimensions.1 && heading == 3 {
        return true;
    }
    if cur_y == 0 && heading == 0 || cur_y == field_dimensions.0 && heading == 2 {
        return true;
    }
    false
}

fn get_heading(area: &Vec<Vec<char>>, pos_x: u32, pos_y: u32) -> Option<u32> {
    let position = area
        .get(pos_y as usize)
        .unwrap()
        .get(pos_x as usize)
        .unwrap();
    return if *position == '^' {
        Some(0)
    } else if *position == '>' {
        Some(1)
    } else if *position == 'v' {
        Some(2)
    } else if *position == '<' {
        Some(3)
    } else if *position == 'X' {
        println!("got X heading... is the guard out?");
        Some(4)
    } else {
        println!("error! heading is {}", *position);
        None
    };
}

fn load_area(input: &str) -> (u32, u32, Vec<Vec<char>>) {
    let re_newline = Regex::new(r"\n|\r\n").unwrap();
    let area = re_newline
        .split(input)
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
    for (i, line) in area.iter().enumerate() {
        for (j, position) in line.iter().enumerate() {
            if *position == '^' || *position == '<' || *position == '>' || *position == 'v' {
                println!("found guard at x{} y{}", j, i);
                return (j as u32, i as u32, area);
            }
        }
    }
    (0, 0, area)
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
