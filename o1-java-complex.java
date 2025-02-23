/*
File: CrudOperations.java
This class demonstrates basic CRUD (Create, Read, Update, Delete) operations using JDBC with SQLite.
Usage:
    javac CrudOperations.java
    java CrudOperations
Dependencies:
    SQLite JDBC driver (sqlite-jdbc-<version>.jar)
Example Output:
    Database created or opened successfully.
    Table created successfully.
    Record inserted: (John Doe, 29)
    Record inserted: (Jane Smith, 34)
    Reading records from the database:
    ID: 1, Name: John Doe, Age: 29
    ID: 2, Name: Jane Smith, Age: 34
    Record updated: ID=1 now has age=30
    Record with ID=2 deleted.
*/

import java.sql.*;

public class CrudOperations {

    // SQLite connection URL
    private static final String URL = "jdbc:sqlite:sample.db";

    public static void main(String[] args) {
        try {
            // 1. Establish connection
            Connection conn = DriverManager.getConnection(URL);
            System.out.println("Database created or opened successfully.");

            // 2. Create table if it does not exist
            createTable(conn);

            // 3. Insert sample records
            insertRecord(conn, "John Doe", 29);
            insertRecord(conn, "Jane Smith", 34);

            // 4. Read and print records
            readRecords(conn);

            // 5. Update a record
            updateRecord(conn, 1, 30);

            // 6. Delete a record
            deleteRecord(conn, 2);

            conn.close();
        } catch (SQLException e) {
            e.printStackTrace();
        }
    }

    private static void createTable(Connection conn) throws SQLException {
        // The SQL statement to create a table if it doesn't already exist
        String sql = "CREATE TABLE IF NOT EXISTS persons ("
                   + "id INTEGER PRIMARY KEY AUTOINCREMENT, "
                   + "name TEXT NOT NULL, "
                   + "age INTEGER NOT NULL"
                   + ")";
        try (Statement stmt = conn.createStatement()) {
            stmt.execute(sql);
            System.out.println("Table created successfully.");
        }
    }

    private static void insertRecord(Connection conn, String name, int age) throws SQLException {
        String sql = "INSERT INTO persons (name, age) VALUES (?, ?)";
        try (PreparedStatement pstmt = conn.prepareStatement(sql)) {
            pstmt.setString(1, name);
            pstmt.setInt(2, age);
            pstmt.executeUpdate();
            System.out.println("Record inserted: (" + name + ", " + age + ")");
        }
    }

    private static void readRecords(Connection conn) throws SQLException {
        String sql = "SELECT id, name, age FROM persons";
        try (Statement stmt = conn.createStatement();
             ResultSet rs = stmt.executeQuery(sql)) {
            System.out.println("Reading records from the database:");
            while (rs.next()) {
                System.out.println("ID: " + rs.getInt("id") 
                    + ", Name: " + rs.getString("name")
                    + ", Age: " + rs.getInt("age"));
            }
        }
    }

    private static void updateRecord(Connection conn, int id, int newAge) throws SQLException {
        String sql = "UPDATE persons SET age=? WHERE id=?";
        try (PreparedStatement pstmt = conn.prepareStatement(sql)) {
            pstmt.setInt(1, newAge);
            pstmt.setInt(2, id);
            pstmt.executeUpdate();
            System.out.println("Record updated: ID=" + id + " now has age=" + newAge);
        }
    }

    private static void deleteRecord(Connection conn, int id) throws SQLException {
        String sql = "DELETE FROM persons WHERE id=?";
        try (PreparedStatement pstmt = conn.prepareStatement(sql)) {
            pstmt.setInt(1, id);
            pstmt.executeUpdate();
            System.out.println("Record with ID=" + id + " deleted.");
        }
    }
}
