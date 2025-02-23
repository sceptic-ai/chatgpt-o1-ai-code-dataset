// File: linear_search.rs
// This Rust program performs a linear search for a target element in an integer array.
// Usage:
//   cargo run
// Example Output:
//   Enter a number to search: 25
//   25 found at index 2

use std::io;

/// Performs a linear search on the given slice for the target value.
/// Returns Some(index) if found, or None if not found.
fn linear_search(arr: &[i32], target: i32) -> Option<usize> {
    for (index, &value) in arr.iter().enumerate() {
        if value == target {
            return Some(index);
        }
    }
    None
}

fn main() {
    let arr = [10, 15, 25, 30, 45];
    
    println!("Enter a number to search: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line.");
    let num: i32 = input.trim().parse().expect("Please enter a valid integer.");
    
    match linear_search(&arr, num) {
        Some(index) => println!("{} found at index {}", num, index),
        None => println!("{} not found in the array", num),
    }
}
