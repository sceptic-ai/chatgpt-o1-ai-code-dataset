// File: todo_cli.rs
// This Rust program implements a very basic command-line Todo application.
// It allows adding tasks, listing them, and marking tasks as done.
//
// Usage:
//   cargo run -- [command] [arguments]
// Example:
//   cargo run -- add "Buy groceries"
//   cargo run -- list
//
// Example Output:
//   Added task 1: "Buy groceries"
//   1: [ ] Buy groceries
//   Completed task 1: "Buy groceries"
//
use std::env;
use std::fs::{OpenOptions, read_to_string};
use std::io::{Write, Error};

#[derive(Debug)]
struct Task {
    id: usize,
    title: String,
    done: bool,
}

fn main() -> Result<(), Error> {
    // Retrieve arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: todo_cli <command> [arguments]");
        return Ok(());
    }
    
    let command = &args[1];
    let todo_file = "todo.txt";

    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                eprintln!("Usage: todo_cli add <task_description>");
                return Ok(());
            }
            let task_description = &args[2..].join(" ");
            add_task(todo_file, task_description)?;
        }
        "list" => {
            list_tasks(todo_file)?;
        }
        "done" => {
            if args.len() < 3 {
                eprintln!("Usage: todo_cli done <task_id>");
                return Ok(());
            }
            let task_id: usize = args[2].parse().expect("Please provide a valid task id.");
            mark_done(todo_file, task_id)?;
        }
        _ => {
            eprintln!("Unknown command. Available commands: add, list, done");
        }
    }
    Ok(())
}

/// Add a new task to the file.
fn add_task(file_path: &str, description: &str) -> Result<(), Error> {
    let mut tasks = load_tasks(file_path)?;
    let new_id = tasks.len() + 1;
    tasks.push(Task {
        id: new_id,
        title: description.to_string(),
        done: false,
    });
    save_tasks(file_path, &tasks)?;
    println!("Added task {}: \"{}\"", new_id, description);
    Ok(())
}

/// List all tasks from the file.
fn list_tasks(file_path: &str) -> Result<(), Error> {
    let tasks = load_tasks(file_path)?;
    for task in tasks {
        let status = if task.done { "[x]" } else { "[ ]" };
        println!("{}: {} {}", task.id, status, task.title);
    }
    Ok(())
}

/// Mark a task as done by its ID.
fn mark_done(file_path: &str, task_id: usize) -> Result<(), Error> {
    let mut tasks = load_tasks(file_path)?;
    for task in tasks.iter_mut() {
        if task.id == task_id {
            task.done = true;
            println!("Completed task {}: \"{}\"", task.id, task.title);
        }
    }
    save_tasks(file_path, &tasks)?;
    Ok(())
}

/// Load tasks from file and parse them into a vector of Task structs.
fn load_tasks(file_path: &str) -> Result<Vec<Task>, Error> {
    let contents = match read_to_string(file_path) {
        Ok(c) => c,
        Err(_) => String::new(),
    };
    let mut tasks = Vec::new();
    for line in contents.lines() {
        // Expected format: id|title|done
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() == 3 {
            let id: usize = parts[0].parse().unwrap_or(0);
            let title = parts[1].to_string();
            let done: bool = parts[2].parse().unwrap_or(false);
            tasks.push(Task { id, title, done });
        }
    }
    Ok(tasks)
}

/// Save tasks back to the file in a simple text format.
fn save_tasks(file_path: &str, tasks: &[Task]) -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_path)?;
    
    for task in tasks {
        let line = format!("{}|{}|{}\n", task.id, task.title, task.done);
        file.write_all(line.as_bytes())?;
    }
    Ok(())
}
