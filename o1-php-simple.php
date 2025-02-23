<?php
/**
 * File: read_csv.php
 * This PHP script reads data from a CSV file and prints it in a tabular format.
 * Usage:
 *   php read_csv.php
 * Example Output:
 *   Reading sample.csv:
 *   John,30
 *   Alice,25
 */

// Path to your CSV file
$csvFile = __DIR__ . '/sample.csv';

/**
 * Reads a CSV file and returns an array of rows (each row is an array of fields).
 *
 * @param string $filename Path to the CSV file.
 * @return array An array of rows from the CSV.
 */
function readCsv($filename) {
    $rows = array();
    if (($handle = fopen($filename, 'r')) !== false) {
        while (($data = fgetcsv($handle, 1000, ',')) !== false) {
            $rows[] = $data;
        }
        fclose($handle);
    }
    return $rows;
}

// Example usage
echo "Reading sample.csv:\n";
$csvData = readCsv($csvFile);
foreach ($csvData as $row) {
    echo implode(",", $row) . "\n";
}
