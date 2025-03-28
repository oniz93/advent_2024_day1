use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::num::ParseIntError;

/// Calculates the total distance between two lists based on sorted pairs.
///
/// Sorts both lists, pairs elements at the same sorted index, calculates
/// the absolute difference for each pair, and sums these differences.
/// Panics if the lists do not have the same length.
fn calculate_total_distance(left: &[i64], right: &[i64]) -> i64 {
    // Ensure the lists have the same length, as implied by the pairing rule.
    assert_eq!(
        left.len(),
        right.len(),
        "Input lists must have the same length for pairing."
    );

    let mut left_sorted = left.to_vec();
    let mut right_sorted = right.to_vec();

    left_sorted.sort_unstable();
    right_sorted.sort_unstable();

    // Pair up elements from the sorted lists using zip.
    // Calculate the absolute difference for each pair and sum them up.
    let total_distance: i64 = left_sorted
        .iter()
        .zip(right_sorted.iter())
        .map(|(&l, &r)| (l - r).abs())
        .sum();

    total_distance
}

fn main() -> io::Result<()> {
    let filename = "input.txt";
    println!("Attempting to read input from '{}'...", filename);

    let file = match File::open(filename) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error: Could not open file '{}'. Reason: {}", filename, e);
            eprintln!("Please ensure the file exists in the same directory as the executable.");
            return Err(e);
        }
    };

    let reader = BufReader::new(file);

    let mut left_list: Vec<i64> = Vec::new();
    let mut right_list: Vec<i64> = Vec::new();
    let mut line_num = 0;

    for line_result in reader.lines() {
        line_num += 1;
        let line = match line_result {
            Ok(line) => line,
            Err(e) => {
                eprintln!("Error reading line {} from file: {}", line_num, e);
                continue;
            }
        };

        let trimmed_line = line.trim();

        if trimmed_line.is_empty() {
            continue;
        }
        
        let parts: Vec<&str> = trimmed_line.split_whitespace().collect();

        if parts.len() == 2 {
            let parse_left: Result<i64, ParseIntError> = parts[0].parse();
            let parse_right: Result<i64, ParseIntError> = parts[1].parse();

            match (parse_left, parse_right) {
                (Ok(left_num), Ok(right_num)) => {
                    left_list.push(left_num);
                    right_list.push(right_num);
                }
                
                (Err(e), _) => {
                    eprintln!(
                        "Error parsing first number on line {}: '{}' ({})",
                        line_num, parts[0], e
                    );
                }
                (_, Err(e)) => {
                    eprintln!(
                        "Error parsing second number on line {}: '{}' ({})",
                        line_num, parts[1], e
                    );
                }
            }
        } else {
            eprintln!(
                "Warning: Line {} ('{}') does not contain exactly two numbers separated by whitespace. Skipping.",
                line_num, trimmed_line
            );
        }
    }

    println!("Finished reading file. Found {} valid pairs.", left_list.len());

    if left_list.is_empty() {
        println!("No valid number pairs were read from the file.");
    }
    else if left_list.len() != right_list.len() {
        eprintln!(
            "Error: Internal inconsistency. Read {} left numbers and {} right numbers. Cannot pair.",
            left_list.len(), right_list.len()
        );
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Mismatched number of left and right values read"));
    }
    else {
        let total_dist = calculate_total_distance(&left_list, &right_list);
        println!("\nTotal distance between the lists: {}", total_dist);
    }

    Ok(())
}