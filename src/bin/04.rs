use itertools::Itertools;
use std::ops::Index;
use std::str::Chars;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let matrix_2d: Vec<Vec<char>> = create_matrix_from_string(input);
    let search_terms: Vec<Vec<char>> = vec![
        "xmas".chars().collect::<Vec<char>>(),
        "samx".chars().collect::<Vec<char>>(),
    ];
    let mut match_count = 0;
    match_count += find_matches(&matrix_2d, &search_terms);
    println!(
        "matches so far: {}, Rotate the matrix by 45 degrees",
        match_count
    );
    let rotated_matrix = rotate_matrix_45(&matrix_2d);
    match_count += find_matches(&rotated_matrix, &search_terms);
    println!(
        "matches so far: {}, Rotate the matrix by 90 degrees",
        match_count
    );
    let rotated_matrix = rotate_matrix_90(&matrix_2d);
    match_count += find_matches(&rotated_matrix, &search_terms);
    println!(
        "matches so far: {}, Rotate the matrix by 135 degrees",
        match_count
    );
    let rotated_matrix = rotate_matrix_45(&rotated_matrix);
    match_count += find_matches(&rotated_matrix, &search_terms);

    Some(match_count)
}

fn find_matches(matrix_2d: &Vec<Vec<char>>, search_terms: &Vec<Vec<char>>) -> u32 {
    let mut match_count = 0;
    for row in matrix_2d.iter() {
        let row_string: String = row.iter().join("");
        println!("row:{} {}", row_string.len(), row_string);
        for term in search_terms {
            let term_string: String = term.iter().join("");
            println!("term: {}", term_string);
            match_count += row_string.matches(&term_string).count() as u32;
        }
    }
    match_count
}

fn rotate_matrix_90(matrix_2d: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = matrix_2d.len();
    let cols = matrix_2d[0].len();

    let mut rotated_matrix = vec![vec![' '; rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            rotated_matrix[j][rows - 1 - i] = matrix_2d[i][j];
        }
    }

    rotated_matrix
}

fn rotate_matrix_45(matrix_2d: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut diagonals: Vec<Vec<char>> = Vec::new();
    let rows = matrix_2d.len();
    let cols = matrix_2d[0].len();

    for d in 0..(rows + cols - 1) {
        let mut diagonal = Vec::new();
        for i in 0..rows {
            let j = d as isize - i as isize;
            if j >= 0 && (j as usize) < cols {
                diagonal.push(matrix_2d[i][j as usize]);
            }
        }
        if !diagonal.is_empty() {
            diagonals.push(diagonal);
        }
    }

    diagonals
}

pub fn part_two(input: &str) -> Option<u32> {
    let matrix_2d: Vec<Vec<char>> = create_matrix_from_string(input);
    let pattern = vec![
        vec!['m', '.', 'm'],
        vec!['.', 'a', '.'],
        vec!['s', '.', 's'],
    ];
    let variations = generate_pattern_variations(&pattern);
    let mut match_count: u32 = 0;
    for (i, pattern_variation) in variations.iter().enumerate() {
        println!("Variation {}:", i);
        for row in pattern_variation {
            println!("{}", row.iter().collect::<String>());
        }
        println!();

        let matches = find_pattern_matches(&matrix_2d, &pattern_variation);
        match_count += matches;
        println!("found {} matches", matches);
    }

    Some(match_count)
}

fn rotate_pattern(pattern: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = pattern.len();
    let mut rotated = vec![vec![' '; n]; n];

    for i in 0..n {
        for j in 0..n {
            rotated[i][j] = pattern[n - 1 - j][i];
        }
    }

    rotated
}

fn generate_pattern_variations(original: &Vec<Vec<char>>) -> Vec<Vec<Vec<char>>> {
    let mut variations = Vec::new();

    // Original pattern
    variations.push(original.clone());

    // Rotations
    let mut current = original.clone();
    for _ in 0..3 {
        current = rotate_pattern(&current);
        variations.push(current.clone());
    }

    variations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}

fn create_matrix_from_string(input: &str) -> Vec<Vec<char>> {
    let size = input.matches("\n").count();
    println!("matrix size: {}", size);
    input
        .trim()
        .chars()
        .filter(|&c| c != '\n' && c != '\r')
        .map(|c| c.to_ascii_lowercase())
        .collect::<Vec<_>>()
        .chunks(size)
        .map(|chunk| chunk.to_vec())
        .collect()
}

use std::vec::Vec;

fn extract_submatrix(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> Vec<Vec<char>> {
    let mut submatrix = vec![vec![' '; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            submatrix[i][j] = matrix[row + i][col + j];
        }
    }

    submatrix
}

fn matches_pattern(submatrix: &Vec<Vec<char>>, pattern: &Vec<Vec<char>>) -> bool {
    for i in 0..3 {
        for j in 0..3 {
            // Skip the check if the pattern has a wildcard '.'
            if pattern[i][j] != '.' && pattern[i][j].to_ascii_lowercase() != submatrix[i][j].to_ascii_lowercase() {
                return false;
            }
        }
    }
    true
}

fn find_pattern_matches(matrix: &Vec<Vec<char>>, pattern: &Vec<Vec<char>>) -> u32 {
    let mut match_count: u32 = 0;
    if matrix.is_empty() || matrix[0].len() < 3 || matrix.len() < 3 {
        return 0;
    }

    // Slide 3x3 matrix window over input
    for row in 0..=matrix.len() - 3 {
        for col in 0..=matrix[0].len() - 3 {
            let submatrix = extract_submatrix(matrix, row, col);

            if matches_pattern(&submatrix, pattern) {
                println!("Found match at row:col {}:{}", row, col);
                match_count += 1;
            }
        }
    }
    match_count
}
