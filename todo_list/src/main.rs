use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Write};

// Function to add a task to the list
fn add_task(tasks: &mut Vec<String>) {
    println!("Enter a task:");
    let mut task = String::new();
    io::stdin().read_line(&mut task).expect("Failed to read input");
    tasks.push(task.trim().to_string()); // Add the trimmed task to the list
}

// Function to display all tasks
fn display_tasks(tasks: &Vec<String>) {
    if tasks.is_empty() {
        println!("No tasks found!");
    } else {
        println!("Tasks:");
        for (i, task) in tasks.iter().enumerate() {
            println!("{}: {}", i + 1, task);
        }
    }
}

// Function to save tasks to a file
fn save_tasks(tasks: &Vec<String>) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("tasks.txt")
        .expect("Unable to open file");

    for task in tasks {
        writeln!(file, "{}", task).expect("Failed to write to file");
    }

    println!("Tasks saved to tasks.txt");
}

// Function to load tasks from a file
fn load_tasks() -> Vec<String> {
    let file = OpenOptions::new().read(true).open("tasks.txt");

    let mut tasks = Vec::new();

    if let Ok(file) = file {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(task) = line {
                tasks.push(task);
            }
        }
    }

    tasks
}

fn main() {
    let mut tasks = load_tasks(); // Load tasks from the file

    loop {
        println!("Options:");
        println!("1. Add a task");
        println!("2. Display tasks");
        println!("3. Save tasks");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim() {
            "1" => add_task(&mut tasks),
            "2" => display_tasks(&tasks),
            "3" => save_tasks(&tasks),
            "4" => {
                save_tasks(&tasks);
                break;
            }
            _ => println!("Invalid choice! Please try again."),
        }
    }
}
