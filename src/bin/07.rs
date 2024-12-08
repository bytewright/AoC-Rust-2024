advent_of_code::solution!(7);
pub fn part_one(input: &str) -> Option<i128> {
    let re_newline = Regex::new(r"\n|\r\n").unwrap();
    let lines: Vec<&str> = re_newline.split(input).collect();
    let total_sum: i128 = lines
        .par_iter()
        .enumerate()
        .filter_map(|(i, line)| {
            match solve_tree_problem(i, line) {
                Ok(solution) => {
                    if solution.0 != 0 {
                        println!("{} is valid formula, eval took {:?}", i, solution.1);
                        Some(solution.0)
                    } else {
                        None
                    }
                }
                Err(e) => {
                    eprintln!("{} Error: {}", i, e);
                    None
                }
            }
        })
        .sum();

    Some(total_sum)
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

use rayon::prelude::*;
use regex::Regex;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug)]
struct TreeNode {
    value: i128,
    left: Option<Arc<Mutex<TreeNode>>>,
    right: Option<Arc<Mutex<TreeNode>>>,
    operator: Option<Operator>,
}

impl TreeNode {
    // Create a new leaf node
    fn new_leaf(value: i128) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
            operator: None,
        }
    }

    // Create a new internal node with an operator
    fn new_node(
        value: i128,
        left: Arc<Mutex<TreeNode>>,
        right: Arc<Mutex<TreeNode>>,
        op: Operator,
    ) -> Self {
        TreeNode {
            value,
            left: Some(left),
            right: Some(right),
            operator: Some(op),
        }
    }

    fn evaluate(&self) -> i128 {
        match &self.operator {
            Some(Operator::Add) => {
                let left_val = self.left.as_ref().unwrap().lock().unwrap().evaluate();
                let right_val = self.right.as_ref().unwrap().lock().unwrap().evaluate();
                left_val + right_val
            }
            Some(Operator::Multiply) => {
                let left_val = self.left.as_ref().unwrap().lock().unwrap().evaluate();
                let right_val = self.right.as_ref().unwrap().lock().unwrap().evaluate();
                left_val * right_val
            }
            None => self.value,
        }
    }
}

fn parse_input(input: &str) -> Result<(i128, Vec<i128>), &'static str> {
    let parts: Vec<&str> = input.split(':').collect();
    if parts.len() != 2 {
        return Err("Invalid input format");
    }

    let result: i128 = parts[0].parse().map_err(|_| "Invalid result number")?;
    let args: Vec<i128> = parts[1]
        .split_whitespace()
        .map(|s| s.parse().map_err(|_| "Invalid argument"))
        .collect::<Result<Vec<i128>, _>>()?;

    Ok((result, args))
}

fn solve_tree_problem(index: usize, input: &str) -> Result<(i128, Duration), String> {
    let start = Instant::now();
    let (result, args) = parse_input(input)?;
    println!(
        "{}: starting on line {} with {} args: {:?}",
        result,
        index,
        args.len(),
        args
    );
    let trees: Vec<Arc<Mutex<TreeNode>>> = generate_trees_parallel_pruned(&args, result);
    println!(
        "{}: Generated {} trees. Time elapsed: {:?}",
        result,
        trees.len(),
        start.elapsed()
    );

    // Parallel evaluation
    if let Some(solution) = evaluate_trees_parallel(trees, result) {
        return Ok((solution.lock().unwrap().evaluate(), start.elapsed()));
    }

    Err("Couldn't find solution after generating trees".to_string())
}

fn evaluate_trees_parallel(
    trees: Vec<Arc<Mutex<TreeNode>>>,
    target: i128,
) -> Option<Arc<Mutex<TreeNode>>> {
    trees
        .into_par_iter()
        .find_any(|tree| tree.lock().unwrap().evaluate() == target)
}

fn generate_trees_parallel_pruned(args: &[i128], target: i128) -> Vec<Arc<Mutex<TreeNode>>> {
    if args.len() == 1 {
        let value = args[0];
        return vec![Arc::new(Mutex::new(TreeNode::new_leaf(value)))];
    }

    (1..args.len())
        .into_par_iter()
        .filter_map(|i| {
            let left_args = &args[..i];
            let right_args = &args[i..];
            // Recursively generate left and right subtrees
            let left_trees = generate_trees_parallel_pruned(left_args, target);
            let right_trees = generate_trees_parallel_pruned(right_args, target);

            let mut local_trees = Vec::new();

            // Combine left and right subtrees
            for left_tree in &left_trees {
                let left_eval = left_tree.lock().unwrap().evaluate();
                for right_tree in &right_trees {
                    let right_eval = right_tree.lock().unwrap().evaluate();
                    // Add operation
                    let add_result = left_eval + right_eval;
                    if add_result <= target {
                        let add_tree = Arc::new(Mutex::new(TreeNode::new_node(
                            add_result,
                            Arc::clone(left_tree),
                            Arc::clone(right_tree),
                            Operator::Add,
                        )));
                        local_trees.push(add_tree);
                    }

                    // Multiply operation
                    let multiply_result = left_eval * right_eval;
                    if multiply_result <= target {
                        let multiply_tree = Arc::new(Mutex::new(TreeNode::new_node(
                            multiply_result,
                            Arc::clone(left_tree),
                            Arc::clone(right_tree),
                            Operator::Multiply,
                        )));
                        local_trees.push(multiply_tree);
                    }
                }
            }
            Some(local_trees)
        })
        .flatten()
        .collect()
}
