// File: prime_check.go
// This Go program determines if a given integer is prime.
// Usage:
//   go run prime_check.go
// Example Output:
//   Enter a number: 13
//   13 is prime.

package main

import (
    "fmt"
    "math"
)

// isPrime checks if a given integer n is prime.
func isPrime(n int) bool {
    if n < 2 {
        return false
    }
    // Only check divisibility up to the square root of n
    limit := int(math.Sqrt(float64(n)))
    for i := 2; i <= limit; i++ {
        if n%i == 0 {
            return false
        }
    }
    return true
}

func main() {
    var num int
    fmt.Print("Enter a number: ")
    fmt.Scan(&num)

    if isPrime(num) {
        fmt.Printf("%d is prime.\n", num)
    } else {
        fmt.Printf("%d is not prime.\n", num)
    }
}
