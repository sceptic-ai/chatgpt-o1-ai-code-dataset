"""
File: factorial.py
This script calculates the factorial of a given non-negative integer.
No external libraries are required.
Usage:
    python factorial.py
Example Output:
    Enter a non-negative integer: 5
    Factorial of 5 is 120
"""

def factorial(n):
    """
    Calculate the factorial of a non-negative integer n using an iterative approach.
    
    Args:
        n (int): Non-negative integer whose factorial is to be computed.
    Returns:
        int: Factorial of n.
        
    Raises:
        ValueError: If n is negative.
    """
    if n < 0:
        raise ValueError("n must be a non-negative integer.")
    result = 1
    
    # We multiply result by each integer from 1 to n
    for i in range(1, n+1):
        result *= i
    return result

def main():
    # Prompt user for input
    num = int(input("Enter a non-negative integer: "))
    # Calculate factorial
    fact = factorial(num)
    # Output the result
    print(f"Factorial of {num} is {fact}")

if __name__ == "__main__":
    main()
