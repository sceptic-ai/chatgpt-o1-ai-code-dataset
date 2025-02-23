<?php
/**
 * File: mysqli_crud.php
 * This PHP script demonstrates basic CRUD (Create, Read, Update, Delete) operations
 * using the MySQLi extension with a MySQL database.
 * 
 * Usage:
 *   1. Configure the $servername, $username, $password, and $dbname variables.
 *   2. Run `php mysqli_crud.php`.
 * Example Output:
 *   Connected successfully.
 *   Table persons created successfully (if not exists).
 *   Inserted record John Doe, age 29
 *   Inserted record Jane Smith, age 34
 *   Reading records from database:
 *   ID: 1, Name: John Doe, Age: 29
 *   ID: 2, Name: Jane Smith, Age: 34
 *   Updated record ID=1 to new age=30
 *   Deleted record ID=2
 */

// Database configuration
$servername = "localhost";
$username = "root";
$password = "";
$dbname = "test_db";

// Create connection
$conn = new mysqli($servername, $username, $password);

// Check connection
if ($conn->connect_error) {
    die("Connection failed: " . $conn->connect_error);
}
echo "Connected successfully.\n";

// Create database if it doesn't exist
$conn->query("CREATE DATABASE IF NOT EXISTS $dbname");

// Select database
$conn->select_db($dbname);

// Create table if it doesn't exist
$createTableSQL = "
CREATE TABLE IF NOT EXISTS persons (
  id INT AUTO_INCREMENT PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  age INT NOT NULL
)";
if ($conn->query($createTableSQL) === true) {
    echo "Table persons created successfully (if not exists).\n";
}

// Insert records
insertRecord($conn, "John Doe", 29);
insertRecord($conn, "Jane Smith", 34);

// Read and display records
readRecords($conn);

// Update a record
updateRecord($conn, 1, 30);

// Delete a record
deleteRecord($conn, 2);

$conn->close();

/**
 * Insert a new record into the 'persons' table.
 *
 * @param mysqli $connection The MySQLi connection object.
 * @param string $name The person's name.
 * @param int $age The person's age.
 * @return void
 */
function insertRecord($connection, $name, $age) {
    $sql = "INSERT INTO persons (name, age) VALUES (?, ?)";
    $stmt = $connection->prepare($sql);
    $stmt->bind_param("si", $name, $age);
    if ($stmt->execute()) {
        echo "Inserted record $name, age $age\n";
    }
    $stmt->close();
}

/**
 * Read and display all records from the 'persons' table.
 *
 * @param mysqli $connection The MySQLi connection object.
 * @return void
 */
function readRecords($connection) {
    $sql = "SELECT id, name, age FROM persons";
    $result = $connection->query($sql);

    echo "Reading records from database:\n";
    if ($result->num_rows > 0) {
        while ($row = $result->fetch_assoc()) {
            echo "ID: {$row['id']}, Name: {$row['name']}, Age: {$row['age']}\n";
        }
    } else {
        echo "No records found.\n";
    }
}

/**
 * Update a record's age in the 'persons' table by ID.
 *
 * @param mysqli $connection The MySQLi connection object.
 * @param int $id The ID of the record to update.
 * @param int $newAge The new age to be set.
 * @return void
 */
function updateRecord($connection, $id, $newAge) {
    $sql = "UPDATE persons SET age=? WHERE id=?";
    $stmt = $connection->prepare($sql);
    $stmt->bind_param("ii", $newAge, $id);
    if ($stmt->execute()) {
        echo "Updated record ID=$id to new age=$newAge\n";
    }
    $stmt->close();
}

/**
 * Delete a record from the 'persons' table by ID.
 *
 * @param mysqli $connection The MySQLi connection object.
 * @param int $id The ID of the record to delete.
 * @return void
 */
function deleteRecord($connection, $id) {
    $sql = "DELETE FROM persons WHERE id=?";
    $stmt = $connection->prepare($sql);
    $stmt->bind_param("i", $id);
    if ($stmt->execute()) {
        echo "Deleted record ID=$id\n";
    }
    $stmt->close();
}
