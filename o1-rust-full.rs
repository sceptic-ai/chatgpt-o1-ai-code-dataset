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
/*
====================================
Quicksort Algorithm (In-place)
====================================
Amaç:
    Bir diziyi (slice) yerinde quicksort algoritması kullanarak
    sıralamak. Pivot olarak son elemanı seçiyoruz.
Gereksinimler:
    - Yalnızca Rust standard library kullanılır.
Çalıştırma Talimatları:
    1. Kodu `quicksort.rs` olarak kaydedin.
    2. `cargo run` komutuyla çalıştırın.
Beklenen Çıktı:
    Sıralanmış dizi ekrana yazılır.
*/

fn main() {
    let mut data = vec![29, 10, 14, 37, 13, 14, 1, 100, 72];
    println!("Orijinal dizi: {:?}", data);
    
    quicksort(&mut data);
    println!("Sıralanmış dizi: {:?}", data);
}

/// In-place quicksort fonksiyonu.
///
/// # Parametreler
/// * `arr` - Sıralanacak verileri taşıyan mutable slice.
/// 
/// # Açıklama
/// 1. `quicksort` fonksiyonu `arr` boyutu 2 veya daha az ise zaten sıralı kabul eder.
/// 2. Eğer boyut daha büyükse, pivot seçilir ve `partition` fonksiyonu ile pivotun
///    doğru yerine yerleşmesi sağlanır. Sonrasında pivotun sol ve sağ parçaları
///    kendi içinde recursive olarak sıralanır.
fn quicksort(arr: &mut [i32]) {
    let len = arr.len();
    if len < 2 {
        // Dizi 0 veya 1 elemanlıysa zaten sıralıdır.
        return;
    }
    // Diziyi partition fonksiyonu ile böl, dönen pivot indeksi ile iki tarafta recursive çağrı yap.
    let pivot_index = partition(arr);
    // pivot_index konumu artık doğru sıradadır.
    
    // Soldaki alt dilimi (pivot hariç) quicksort ile sırala.
    quicksort(&mut arr[0..pivot_index]);
    // Sağdaki alt dilimi (pivot hariç) quicksort ile sırala.
    quicksort(&mut arr[pivot_index+1..]);
}

/// `partition` fonksiyonu:
/// * Pivot olarak slice'ın son elemanını kullanır.
/// * Pivotun "doğru" pozisyonunu bulur ve onu oraya yerleştirir.
/// * Pivotun solundaki elemanlar pivotdan küçük, sağındaki elemanlar pivotdan büyük olacak şekilde verileri düzenler.
///
/// # Döndürdüğü Değer
/// * Pivotun yerleştirildiği (yeni) index.
fn partition(arr: &mut [i32]) -> usize {
    let len = arr.len();
    let pivot = arr[len - 1];
    let mut i = 0; // i, pivotdan küçük elemanların yerleştirileceği indexi tutar.

    // Son eleman pivot, o yüzden sondan bir önceki elemana kadar inceliyoruz.
    for j in 0..(len - 1) {
        if arr[j] < pivot {
            arr.swap(i, j); 
            i += 1;
        }
    }
    // Pivotu final yerine taşı (i indeksine).
    arr.swap(i, len - 1);
    i
}
/*
=====================================
Merge Sort Algorithm (Recursive)
=====================================
Amaç:
    Bir dilimi (slice) merge sort algoritması kullanarak sıralamak.
Gereksinimler:
    - Yalnızca Rust standard library kullanılır.
Çalıştırma Talimatları:
    1. Kodu `mergesort.rs` olarak kaydedin.
    2. `cargo run` komutuyla çalıştırın.
Beklenen Çıktı:
    Sıralanmış dizi ekrana yazılır.
*/

fn main() {
    let mut data = vec![5, 2, 8, 14, 1, 9, 3];
    println!("Orijinal veri: {:?}", data);
    
    mergesort(&mut data);
    println!("Sıralanmış veri: {:?}", data);
}

/// Merge sort fonksiyonu.
///
/// # Mantık
/// 1. Dizi iki parçaya bölünür.
/// 2. Her parça ayrı ayrı merge sort edilir (recursively).
/// 3. İki parça birleştirilir (merge).
fn mergesort(arr: &mut [i32]) {
    let n = arr.len();
    // Dizi tek elemanlı veya boşsa sıralamaya gerek yok.
    if n <= 1 {
        return;
    }
    
    // Orta noktayı bul.
    let mid = n / 2;
    // İki alt dilim oluştur: sol ve sağ.
    // Rust'ta slice'ları bölerek borç alıyoruz; mutable referanslar
    // eşzamanlı olarak farklı parçalara verilince UB olmaz.
    mergesort(&mut arr[..mid]);
    mergesort(&mut arr[mid..]);
    
    // Geçici vector ile birleştirme işlemi (merge).
    let mut temp = Vec::with_capacity(n);
    
    let (mut i, mut j) = (0, mid);
    
    // İki dilimi de dolaşarak küçük elemanı temp'e at.
    while i < mid && j < n {
        if arr[i] <= arr[j] {
            temp.push(arr[i]);
            i += 1;
        } else {
            temp.push(arr[j]);
            j += 1;
        }
    }
    
    // Kalan elemanlar varsa ekle.
    temp.extend_from_slice(&arr[i..mid]);
    temp.extend_from_slice(&arr[j..n]);
    
    // temp içeriğini tekrar arr'a kopyala.
    arr.copy_from_slice(&temp);
}
/*
==========================================
JSON Processing with Serde
==========================================
Amaç:
    Serde kullanarak bir JSON dosyasını parse etmek (okumak) ve
    güncellenmiş hâlini yeniden JSON olarak kaydetmek.
Kullanılacak Krate:
    1. Serde ve serde_derive (otomatik türetme için).
    2. Serde_json (JSON işlemleri için).
Cargo.toml Örnek:
    [dependencies]
    serde = { version = "1.0", features = ["derive"] }
    serde_json = "1.0"
Çalıştırma Talimatları:
    1. `cargo run` ile çalıştırın.
    2. `data.json` isminde basit bir JSON dosyası oluşturabilirsiniz.
         Örnek:
         {
           "name": "Alice",
           "age": 30
         }
Beklenen Çıktı:
    Okunan verinin güncellenmiş hâli `updated_data.json` dosyasına kaydedilir.
*/

use serde::{Deserialize, Serialize};
use std::fs;
use std::error::Error;

/// Kişi verisini tutan struct.
/// Derive ile Serialize ve Deserialize otomatik eklenir.
#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    // JSON dosyasını okuyalım.
    let data = fs::read_to_string("data.json")?;
    
    // JSON'u `Person` tipine parse edelim.
    let mut person: Person = serde_json::from_str(&data)?;
    println!("Okunan kişi: {:?}", person);
    
    // Yaşı 1 artırıyoruz.
    person.age += 1;
    
    // Yeni veriyi JSON'a çevir.
    let updated_json = serde_json::to_string_pretty(&person)?;
    
    // Yeni JSON'u kaydet.
    fs::write("updated_data.json", updated_json)?;
    
    println!("Veri güncellendi ve 'updated_data.json' dosyasına yazıldı.");
    Ok(())
}
/*
========================================================
Trait ve Generics ile Özel Stack Veri Yapısı
========================================================
Amaç:
    - Generic bir Stack veri yapısı oluşturmak.
    - Bir trait tanımlamak ve bu trait’i struct üzerinde uygulamak.
Gereksinimler:
    - Yalnızca Rust standard library.
Çalıştırma Talimatları:
    1. `cargo run` komutu ile çalıştırabilirsiniz.
    2. Kod dosyası ismi `generic_stack.rs` olabilir.
Beklenen Çıktı:
    Stack ile push/pop yapılan işlemler ekranda gösterilir.
*/

fn main() {
    // i32 tipinde elemanlar tutan stack oluştur.
    let mut int_stack = GenericStack::new();
    int_stack.push(10);
    int_stack.push(20);
    int_stack.push(30);
    
    println!("Pop: {:?}", int_stack.pop()); // 30
    
    // &dyn StackTrait trait nesnesi üzerinden işlem yapalım.
    let stack_ref: &dyn StackTrait<i32> = &int_stack;
    println!("Top elemanı: {:?}", stack_ref.peek()); // 20
    
    println!("Stack uzunluğu: {}", stack_ref.size()); // 2
}

/// Bir stack üzerinde temel fonksiyonlar sağlayan bir trait.
trait StackTrait<T> {
    fn push(&mut self, item: T);
    fn pop(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
    fn size(&self) -> usize;
}

/// Generic bir stack yapısı. `Vec` kullanarak implement ediyoruz.
struct GenericStack<T> {
    items: Vec<T>,
}

impl<T> GenericStack<T> {
    /// Yeni bir `GenericStack` döndüren yardımcı fonksiyon.
    fn new() -> Self {
        GenericStack { items: Vec::new() }
    }
}

/// `StackTrait` traitini `GenericStack` üzerinde implemente ediyoruz.
impl<T> StackTrait<T> for GenericStack<T> {
    /// Stack'e eleman eklemek için push.
    fn push(&mut self, item: T) {
        self.items.push(item);
    }
    
    /// Stack'ten eleman çıkarmak için pop.
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    
    /// Stack'in en üstündeki (en son eklenen) elemana sadece bakmak.
    fn peek(&self) -> Option<&T> {
        self.items.last()
    }
    
    /// Stack'in kaç eleman barındırdığını döndürür.
    fn size(&self) -> usize {
        self.items.len()
    }
}
/*
====================================================
Custom Macro Example
====================================================
Amaç:
    Rust makrolarının (declarative macro) basit kullanımını göstermek.
    Bu örnekte bir `vec_of_strings!` makrosu tanımlıyoruz.
Gereksinimler:
    - Yalnızca Rust standard library.
Çalıştırma Talimatları:
    1. Dosya adı `custom_macro.rs` olabilir.
    2. `cargo run` komutu ile çalıştırabilirsiniz.
Beklenen Çıktı:
    vec_of_strings makrosuyla oluşturulan vektördeki elemanlar ekrana yazılır.
*/

/// Declarative macro tanımı. 
/// `vec_of_strings!` makrosu virgülle ayrılmış string literal'lerden bir `Vec<String>` üretir.
#[macro_export]
macro_rules! vec_of_strings {
    // ( $( ... ),* ) => birden fazla argüman
    // $()* => 0 veya daha fazla tekrar
    // $x:expr => bir ifade
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                // Her ifade bir `String`e dönüştürülerek vektöre eklenir.
                temp_vec.push($x.to_string());
            )*
            temp_vec
        }
    };
}

fn main() {
    // Makroyu kullanarak bir string vektörü oluşturalım.
    let fruits = vec_of_strings!("Apple", "Banana", "Cherry");
    println!("Oluşturulan meyve listesi: {:?}", fruits);
}
/*
============================================================
Rocket Tabanlı REST API (CRUD Örneği)
============================================================
Amaç:
    - Rocket kullanarak basit bir REST API oluşturma.
    - GET, POST, PUT, DELETE örnekleri gösterilir.
Gereksinimler:
    1. `Rocket` krate'ini kullanır. Cargo.toml'da:
       [dependencies]
       rocket = "0.5.0-rc.3"  // Sürüm fark edebilir
    2. JSON dönüşümü için `serde` + `serde_json` kullanır.
Çalıştırma Talimatları:
    1. `cargo run` ile projeyi başlatın.
    2. Örnek istekler (terminal veya Postman üzerinden):
       - GET http://127.0.0.1:8000/items
       - POST http://127.0.0.1:8000/items  (JSON body: {"id":1,"name":"Item1"})
       - PUT http://127.0.0.1:8000/items/1 (JSON body: {"id":1,"name":"UpdatedItem"})
       - DELETE http://127.0.0.1:8000/items/1
*/

#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Serialize, Deserialize};
use std::sync::Mutex;
use rocket::State;
use rocket::http::Status;

/// API içinde kullanacağımız basit veri modeli:
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
struct Item {
    id: u32,
    name: String,
}

/// Uygulama durum verisi: item'ları tutan bir vektör.
/// Mutex ile koruyoruz ki aynı anda birden fazla istek geldiğinde veri
/// tutarlılığını koruyalım.
struct AppState {
    items: Mutex<Vec<Item>>,
}

#[get("/items")]
fn get_items(state: &State<AppState>) -> Json<Vec<Item>> {
    // Mutex içindeki item vektörüne thread-safe erişim.
    let items = state.items.lock().unwrap();
    // Clone ederek JSON'a göndersin, ownership sorunlarını aşalım.
    Json(items.clone())
}

#[post("/items", data = "<item>")]
fn create_item(state: &State<AppState>, item: Json<Item>) -> Status {
    let mut items = state.items.lock().unwrap();
    // Yeni item ekle.
    items.push(item.into_inner());
    // 201 Created statüsü yollayabiliriz.
    Status::Created
}

#[put("/items/<id>", data = "<updated>")]
fn update_item(state: &State<AppState>, id: u32, updated: Json<Item>) -> Status {
    let mut items = state.items.lock().unwrap();
    // Id'si eşleşen item'ı bulalım.
    if let Some(existing_item) = items.iter_mut().find(|it| it.id == id) {
        existing_item.name = updated.name.clone();
        Status::Ok
    } else {
        // Bulunamazsa 404 Not Found
        Status::NotFound
    }
}

#[delete("/items/<id>")]
fn delete_item(state: &State<AppState>, id: u32) -> Status {
    let mut items = state.items.lock().unwrap();
    let initial_len = items.len();
    // Verilen id'ye sahip item'ları filtre dışı bırakarak silme işlemi.
    items.retain(|it| it.id != id);
    
    if items.len() < initial_len {
        Status::Ok
    } else {
        Status::NotFound
    }
}

#[launch]
fn rocket() -> _ {
    // Başlangıçta boş bir item listesi ile state'i oluşturuyoruz.
    rocket::build()
        .manage(AppState {
            items: Mutex::new(vec![]),
        })
        .mount("/", routes![get_items, create_item, update_item, delete_item])
}
