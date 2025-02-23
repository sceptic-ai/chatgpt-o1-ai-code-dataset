/*
File: reverseString.js
This script defines a function to reverse a given string.
No external libraries are needed.
Usage:
    node reverseString.js
Example Output:
    Original: Hello World
    Reversed: dlroW olleH
*/

/**
 * Reverses the characters in a given string.
 * 
 * @param {string} str - The string to be reversed.
 * @return {string} - The reversed string.
 */
function reverseString(str) {
  // Split the string into an array of characters,
  // reverse the array, then join the characters back into a string.
  return str.split('').reverse().join('');
}

// Example usage
const original = "Hello World";
const reversed = reverseString(original);

// Print results
console.log("Original:", original);
console.log("Reversed:", reversed);
