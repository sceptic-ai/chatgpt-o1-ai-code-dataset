//
// File: Fibonacci.swift
// This Swift program calculates the first N Fibonacci numbers iteratively.
// Usage:
//   swift Fibonacci.swift
// Example Output:
//   Enter the number of Fibonacci terms: 5
//   Fibonacci series: 0 1 1 2 3
//

import Foundation

func fibonacci(n: Int) -> [Int] {
    // Return an empty list if n <= 0
    if n <= 0 {
        return []
    }
    
    // If n is 1, just return [0]
    if n == 1 {
        return [0]
    }
    
    var series = [0, 1]
    
    // Build the series iteratively
    for _ in 2..<n {
        let nextValue = series[series.count - 1] + series[series.count - 2]
        series.append(nextValue)
    }
    
    return series
}

print("Enter the number of Fibonacci terms: ", terminator: "")
if let input = readLine(), let num = Int(input) {
    let result = fibonacci(n: num)
    print("Fibonacci series:", result.map { String($0) }.joined(separator: " "))
} else {
    print("Invalid input.")
}
