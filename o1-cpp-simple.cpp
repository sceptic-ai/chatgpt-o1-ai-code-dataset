/*
File: BubbleSort.cpp
This program implements the Bubble Sort algorithm to sort an integer array.
Usage:
    g++ BubbleSort.cpp -o BubbleSort
    ./BubbleSort
Example Output:
    Original array: 64 34 25 12 22 11 90
    Sorted array: 11 12 22 25 34 64 90
*/

#include <iostream>
#include <vector>
using namespace std;

/**
 * Sorts an array of integers using the Bubble Sort algorithm.
 *
 * @param arr A reference to a vector of integers to be sorted.
 */
void bubbleSort(vector<int>& arr) {
    int n = arr.size();
    bool swapped;

    // Repeatedly step through the array
    for (int i = 0; i < n - 1; i++) {
        swapped = false;
        // Compare adjacent elements
        for (int j = 0; j < n - i - 1; j++) {
            // If current element is greater, swap
            if (arr[j] > arr[j + 1]) {
                int temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
                swapped = true;
            }
        }
        // If no swap happened in this pass, the array is already sorted
        if (!swapped) break;
    }
}

int main() {
    vector<int> data = {64, 34, 25, 12, 22, 11, 90};

    cout << "Original array: ";
    for (int num : data) {
        cout << num << " ";
    }
    cout << endl;

    bubbleSort(data);

    cout << "Sorted array: ";
    for (int num : data) {
        cout << num << " ";
    }
    cout << endl;

    return 0;
}
