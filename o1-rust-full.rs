/*
==============================
Factorial Example (Simple)
==============================
Purpose:
    Demonstrate a basic recursive factorial function in Rust.
Crates/Dependencies:
    None (uses only the Rust standard library).
Instructions to Run:
    1. Save this code in a file named `factorial.rs`.
    2. Run with `cargo run`.
Example Output:
    5! = 120
*/

fn main() {
    // We'll compute the factorial of a few numbers for demonstration.
    let number = 5;
    // Calculate factorial using our custom function.
    let result = factorial(number);
    // Print the result to the console.
    println!("{}! = {}", number, result);
}

/// Computes the factorial of a non-negative integer `n`.
/// The factorial of n (written as n!) is the product of all
/// positive integers up to n. For instance, 5! = 1 * 2 * 3 * 4 * 5 = 120.
///
/// # Arguments
///
/// * `n` - A non-negative integer.
///
/// # Returns
///
/// * The factorial value of `n`.
fn factorial(n: u32) -> u32 {
    // If n is 0 or 1, return 1.
    // This is the base case for the recursion.
    if n <= 1 {
        1
    } else {
        // Otherwise, multiply n by the factorial of (n-1).
        // Recursively calls the factorial function.
        n * factorial(n - 1)
    }
}
/*
=============================================
Finding the Largest Element in a Vector
=============================================
Purpose:
    Demonstrate how to traverse a vector and
    track the maximum value in Rust.
Crates/Dependencies:
    None (uses only the Rust standard library).
Instructions to Run:
    1. Save this code in a file named `largest_element.rs`.
    2. Run with `cargo run`.
Example Output:
    The largest element is: 9
*/

fn main() {
    // A sample vector of integers.
    let numbers = vec![3, 7, 2, 9, 5, 1];
    
    // Use our function to find the largest element in the vector.
    if let Some(max_value) = find_largest(&numbers) {
        println!("The largest element is: {}", max_value);
    } else {
        println!("The vector is empty!");
    }
}

/// Returns the largest element in a vector of integers, or `None` if the vector is empty.
///
/// # Arguments
///
/// * `values` - A reference to a vector of i32 values.
///
/// # Returns
///
/// * `Some(i32)` if the vector is non-empty, containing the largest value.
/// * `None` if the vector is empty.
fn find_largest(values: &[i32]) -> Option<i32> {
    // Early return if the slice is empty.
    if values.is_empty() {
        return None;
    }
    
    // Initialize the max to the first element.
    let mut max_val = values[0];
    
    // Iterate over the slice to find the largest element.
    // We borrow the values by reference since we only need to read them.
    for &value in values.iter().skip(1) {
        if value > max_val {
            max_val = value;
        }
    }
    
    Some(max_val)
}
/*
=========================================
Breadth-First Search (Graph Traversal)
=========================================
Purpose:
    Demonstrate BFS on a directed graph using Rust’s ownership
    and borrowing features. This code includes building a graph
    and traversing it from a given start node.
Crates/Dependencies:
    None (uses only the Rust standard library).
Instructions to Run:
    1. Save this code in a file named `bfs.rs`.
    2. Run with `cargo run`.
Example Output:
    Visited: A, B, C, D, E
*/

use std::collections::{HashMap, VecDeque};

fn main() {
    // Define a small directed graph using a hash map.
    // Each node maps to a vector of adjacent nodes.
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    
    // Populate the graph manually.
    graph.insert("A", vec!["B", "C"]);
    graph.insert("B", vec!["D"]);
    graph.insert("C", vec!["D", "E"]);
    graph.insert("D", vec!["E"]);
    graph.insert("E", vec![]);
    
    // Perform BFS starting from node "A".
    let visited_nodes = bfs(&graph, "A");
    println!("BFS Order: {:?}", visited_nodes);
}

/// Performs a breadth-first search on a directed graph starting from `start`.
///
/// # Arguments
///
/// * `graph` - A reference to a HashMap representing the adjacency list of the graph.
/// * `start` - The starting node for the BFS.
///
/// # Returns
///
/// * A vector of visited nodes in BFS order.
fn bfs(graph: &HashMap<&str, Vec<&str>>, start: &str) -> Vec<String> {
    // A queue to manage the BFS frontier.
    let mut queue: VecDeque<&str> = VecDeque::new();
    // A vector to keep track of visited nodes.
    let mut visited: Vec<String> = Vec::new();

    // Insert the start node into the queue.
    queue.push_back(start);

    // Process the queue until it's empty.
    while let Some(current) = queue.pop_front() {
        // Check if we've already visited this node.
        if visited.contains(&current.to_string()) {
            // If so, skip processing this node.
            continue;
        }
        // Mark the current node as visited.
        visited.push(current.to_string());

        // Retrieve the adjacency list of the current node, if any.
        // If current node doesn't exist in the graph, this returns an empty slice.
        if let Some(neighbors) = graph.get(current) {
            // Push each neighbor into the queue to visit later.
            for neighbor in neighbors {
                if !visited.contains(&neighbor.to_string()) {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    visited
}
/*
==================================================
File I/O Example with Basic Error Handling
==================================================
Purpose:
    Demonstrate reading from and writing to a file,
    leveraging Rust's `Result` for error handling.
Crates/Dependencies:
    None (uses Rust standard library - std::fs).
Instructions to Run:
    1. Create a `Cargo.toml` if not already available.
    2. Save this code in a file named `file_io.rs`.
    3. Provide an input file named `input.txt` in the same directory.
    4. Run with `cargo run`.
Example Output:
    If `input.txt` contains "Hello, world!", then:
    Read from file: Hello, world!
    Done writing to output.txt
*/

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Call our read_file function to read data from "input.txt".
    let content = read_file("input.txt")?;
    println!("Read from file: {}", content);
    
    // Write the content to "output.txt".
    write_file("output.txt", &content)?;
    println!("Done writing to output.txt");
    
    // Return Ok if everything went well.
    Ok(())
}

/// Reads the entire content of a file into a String.
/// Returns a Result that, on success, contains the file content.
///
/// # Arguments
///
/// * `filename` - The path to the file as a string slice.
///
/// # Errors
///
/// * If the file cannot be opened or read, an error is returned.
fn read_file(filename: &str) -> Result<String, Box<dyn Error>> {
    // Open the file in read-only mode.
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    
    // Collect all lines into a single string.
    let mut content = String::new();
    for line in reader.lines() {
        // Each line could result in an error, so we handle it with `?`.
        content.push_str(&line?);
        content.push('\n');
    }
    
    Ok(content)
}

/// Writes a given string to a file. Overwrites any existing content.
///
/// # Arguments
///
/// * `filename` - The path to the file as a string slice.
/// * `content` - The text content to write.
///
/// # Errors
///
/// * If the file cannot be created or written to, an error is returned.
fn write_file(filename: &str, content: &str) -> Result<(), Box<dyn Error>> {
    // Use OpenOptions to create a new file or truncate if it exists.
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename)?;
    
    // Write the content to the file.
    file.write_all(content.as_bytes())?;
    
    Ok(())
}
/*
=======================================
Concurrency Example: Multi-threaded Sum
=======================================
Purpose:
    Showcase Rust's std::thread and message passing using channels
    to sum parts of a large data set in parallel.
Crates/Dependencies:
    None (uses only Rust standard library).
Instructions to Run:
    1. Save this code in a file named `concurrent_sum.rs`.
    2. Run with `cargo run`.
Example Output:
    The total sum is: 4950  (if the numbers 1..=100 are used)
*/

use std::thread;
use std::sync::mpsc;

fn main() {
    // We'll sum the integers from 1 to 100 in parallel.
    let numbers: Vec<u32> = (1..=100).collect();
    
    // Create a channel for threads to send partial sums back to main.
    let (tx, rx) = mpsc::channel();
    
    // Split the data into chunks for parallel processing.
    // For example, 2 chunks for demonstration; you can use more.
    let chunk_size = numbers.len() / 2;
    let chunks = numbers.chunks(chunk_size);
    
    for chunk in chunks {
        let thread_tx = tx.clone();
        // Clone the slice into a new Vec so each thread owns its data.
        let data_chunk = chunk.to_vec();
        
        // Spawn a new thread for each chunk of data.
        thread::spawn(move || {
            let partial_sum: u32 = data_chunk.iter().sum();
            // Send the partial sum through the channel.
            thread_tx.send(partial_sum).expect("Failed to send data");
        });
    }
    
    // Drop the original transmitter to avoid deadlock once all threads have been spawned.
    drop(tx);
    
    // Collect partial sums and compute the final total.
    let mut total_sum = 0;
    for received in rx {
        total_sum += received;
    }
    
    println!("The total sum is: {}", total_sum);
}
/*
=================================
Unsafe Rust Example (Low-Level)
=================================
Purpose:
    Demonstrate a minimal use of an `unsafe` block to manipulate
    raw pointers. Explains why the unsafe block is used and how
    we maintain safety guarantees.
Crates/Dependencies:
    None (uses only the Rust standard library).
Instructions to Run:
    1. Save this code as `unsafe_example.rs`.
    2. Run with `cargo run`.
Example Output:
    The number is: 10
Notes on Safety:
    We use unsafe here to illustrate pointer dereferencing. In real
    applications, prefer higher-level abstractions for safety.
*/

fn main() {
    // Create a mutable integer on the stack.
    let mut num: i32 = 10;
    
    // Get a raw pointer to num.
    let ptr = &mut num as *mut i32;
    
    // Use an unsafe block to dereference the raw pointer.
    // We assert that the pointer is valid within this scope.
    unsafe {
        // This is considered unsafe because raw pointers
        // do not guarantee memory safety. We rely on
        // compile-time and run-time reasoning that `ptr`
        // still points to valid data.
        *ptr += 0; // No real change, just a demonstration.
    }
    
    // Demonstrate that we can still access `num` safely here.
    println!("The number is: {}", num);
}
/*
==========================================
Web Server Using Actix (Hello World)
==========================================
Purpose:
    Set up a basic Actix web server that returns a "Hello, World!"
    response. Demonstrates route definition and server startup.
Crates/Dependencies:
    1. Add actix-web to Cargo.toml:
       [dependencies]
       actix-web = "4"
    2. Uses std::io::Result from Rust's standard library.
Instructions to Run:
    1. Save this code as `actix_server.rs`.
    2. In the same directory, run `cargo run`.
    3. The server starts on http://127.0.0.1:8080/
Example Usage:
    Visit http://127.0.0.1:8080/ in your web browser to see "Hello, World!".
*/

use actix_web::{get, App, HttpServer, Responder, HttpResponse};

#[get("/")]
async fn hello() -> impl Responder {
    // Return a simple response body.
    HttpResponse::Ok().body("Hello, World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start the Actix HTTP server on localhost at port 8080.
    HttpServer::new(|| {
        // Create the app with the route we defined above.
        App::new()
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
/*
===================================================
CSV Parsing and Processing Example
===================================================
Purpose:
    Demonstrate reading and processing CSV data using the `csv` crate.
Crates/Dependencies:
    1. Add csv to Cargo.toml:
       [dependencies]
       csv = "1"
    2. Also uses the Rust standard library for I/O.
Instructions to Run:
    1. Save as `csv_parser.rs`.
    2. Create a `data.csv` file with some rows, e.g.:
         name,score
         Alice,50
         Bob,75
         Charlie,90
    3. Run with `cargo run`.
Example Output:
    If `data.csv` is as above, it might print:
    Processed row: (Alice, 50)
    Processed row: (Bob, 75)
    Processed row: (Charlie, 90)
*/

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use csv::ReaderBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    // Open the CSV file. We wrap it in a BufReader for efficient reading.
    let file = File::open("data.csv")?;
    let buffered = BufReader::new(file);
    
    // Create a CSV reader with flexible configuration.
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(buffered);
    
    // Use a for loop to read each record.
    for result in rdr.records() {
        // A record can result in an error, so we use `?` to propagate it.
        let record = result?;
        
        // Here we assume each CSV record has two fields: name and score.
        let name = &record[0];
        let score: i32 = record[1].parse()?;
        
        println!("Processed row: ({}, {})", name, score);
    }
    
    Ok(())
}
/*
=======================================================
Command-Line Tool Using Clap
=======================================================
Purpose:
    Demonstrate creating a CLI application in Rust that
    takes arguments and performs a file copy.
Crates/Dependencies:
    1. Add clap to Cargo.toml:
       [dependencies]
       clap = { version = "4", features = ["derive"] }
    2. Uses Rust standard library for file operations.
Instructions to Run:
    1. Save as `cli_copy.rs`.
    2. Run: `cargo run -- --source <path_to_source> --destination <path_to_destination>`
Example Usage:
    cargo run -- --source input.txt --destination output.txt
    This will copy the contents of input.txt into output.txt.
*/

use clap::Parser;
use std::fs;
use std::error::Error;

#[derive(Parser, Debug)]
#[command(name = "clippy_copy", version = "1.0", author = "Rustacean")]
#[command(about = "A simple CLI to copy a file's contents to another file")]
struct Cli {
    /// Path to the source file
    #[arg(short, long)]
    source: String,
    
    /// Path to the destination file
    #[arg(short, long)]
    destination: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the command-line arguments using Clap.
    let cli = Cli::parse();
    
    // Perform the file copy using Rust's std::fs::copy function.
    fs::copy(&cli.source, &cli.destination)?;
    
    println!("Copied from {} to {}", cli.source, cli.destination);
    Ok(())
}
/*
===============================================================
Multi-threaded Web Scraper Example (Complex)
===============================================================
Purpose:
    Demonstrate a more advanced Rust program that:
      1. Spawns multiple threads to scrape a list of URLs.
      2. Uses channels for coordination and concurrency.
      3. Fetches HTML content and prints a summary.
Crates/Dependencies:
    1. reqwest for HTTP requests:
       [dependencies]
       reqwest = "0.11"
    2. tokio for async runtime (if using async reqwest functions):
       [dependencies]
       tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
    3. This example uses threads + blocking I/O. For purely async,
       you'd adapt this to async/await syntax.
Instructions to Run:
    1. Save this code in `multithreaded_scraper.rs`.
    2. Run `cargo run`.
Notes:
    This scraper doesn't parse or extract links. It simply fetches
    the HTML content and prints the status code and length.
*/

use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use reqwest::blocking::Client;

fn main() {
    // List of URLs to scrape. In real usage, consider reading from a file or CLI.
    let urls = vec![
        "https://www.rust-lang.org",
        "https://www.example.com",
        "https://httpbin.org/get",
    ];

    // Create a channel to receive the scraping results from threads.
    let (tx, rx) = mpsc::channel();

    // We can reuse a single reqwest Client across threads.
    // This is more efficient than creating one client per request.
    let client = Client::new();

    // For each URL, spawn a new thread to fetch the content.
    for url in urls {
        let thread_tx = tx.clone();
        let thread_client = client.clone();
        let thread_url = url.to_string();

        thread::spawn(move || {
            // Perform the HTTP GET request.
            let response_result = thread_client.get(&thread_url).send();

            // Prepare a message to send back through the channel.
            let message = match response_result {
                Ok(response) => {
                    // If successful, read the text (or partial) to get its length.
                    // This blocking call can fail, so we handle that with match.
                    match response.text() {
                        Ok(text) => format!(
                            "[SUCCESS] Fetched: {} | Length: {} chars",
                            thread_url,
                            text.len()
                        ),
                        Err(e) => format!("[ERROR] Failed to read response text: {}", e),
                    }
                }
                Err(e) => format!("[ERROR] Failed to fetch {}: {}", thread_url, e),
            };

            // Send the result back to the main thread.
            thread_tx.send(message).expect("Failed to send data");
        });
    }

    // Drop the original transmitter to ensure the channel closes when threads finish.
    drop(tx);

    // Collect results from all threads.
    while let Ok(result_message) = rx.recv() {
        println!("{}", result_message);
    }

    // Give threads a little time if needed (or could join threads explicitly).
    // This is not strictly required, but sometimes helpful in demonstration code.
    thread::sleep(Duration::from_millis(500));
}
