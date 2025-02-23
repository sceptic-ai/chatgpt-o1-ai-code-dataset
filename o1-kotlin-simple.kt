/*
File: LargestElement.kt
This Kotlin program finds the largest integer in a list.
Usage:
    kotlinc LargestElement.kt -include-runtime -d LargestElement.jar
    java -jar LargestElement.jar
Example Output:
    The largest element is 99
*/

fun findLargestElement(numbers: List<Int>): Int {
    require(numbers.isNotEmpty()) { "List cannot be empty." }
    var maxVal = numbers[0]
    for (num in numbers) {
        if (num > maxVal) {
            maxVal = num
        }
    }
    return maxVal
}

fun main() {
    val listOfNumbers = listOf(10, 99, 23, 45)
    val largest = findLargestElement(listOfNumbers)
    println("The largest element is $largest")
}
