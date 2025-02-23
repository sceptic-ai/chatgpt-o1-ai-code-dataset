# File: bubble_sort.rb
# This script implements the Bubble Sort algorithm to sort an array of integers.
# Usage:
#   ruby bubble_sort.rb
# Example Output:
#   Original array: [64, 34, 25, 12, 22, 11, 90]
#   Sorted array:   [11, 12, 22, 25, 34, 64, 90]

def bubble_sort(array)
  n = array.length
  loop do
    swapped = false
    (n-1).times do |i|
      if array[i] > array[i+1]
        # Swap elements
        array[i], array[i+1] = array[i+1], array[i]
        swapped = true
      end
    end
    break unless swapped
  end
  array
end

# Example usage
data = [64, 34, 25, 12, 22, 11, 90]
puts "Original array: #{data}"
sorted_data = bubble_sort(data)
puts "Sorted array:   #{sorted_data}"
