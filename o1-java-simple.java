/*
File: FindMax.java
This Java class contains a method to find the maximum integer in an array.
No additional libraries are required (only standard Java).
Usage:
    javac FindMax.java
    java FindMax
Example Output:
    The maximum value in the array is: 99
*/

public class FindMax {

    /**
     * Finds the maximum value in an integer array.
     *
     * @param arr The input array of integers.
     * @return The maximum integer found in the array.
     * @throws IllegalArgumentException if the array is null or empty.
     */
    public static int findMaxValue(int[] arr) {
        if (arr == null || arr.length == 0) {
            throw new IllegalArgumentException("Array must not be null or empty.");
        }

        // Initialize max with the first element of the array
        int max = arr[0];

        // Iterate through the array, updating max if a bigger value is found
        for (int value : arr) {
            if (value > max) {
                max = value;
            }
        }

        return max;
    }

    public static void main(String[] args) {
        int[] numbers = {10, 99, 23, 87, 12};
        int maxValue = findMaxValue(numbers);
        System.out.println("The maximum value in the array is: " + maxValue);
    }
}
