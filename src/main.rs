use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, Write};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String, // What the task is
    done: bool,          // Whether it is complete
}

fn main() {
    // Load tasks from file if exists, otherwise store in empty vec
    let mut tasks = load_tasks();

    // main loop, keep showing the menu until a user chooses to exit
    loop {
        println!("\n--- TO-DO LIST ---");

        // Print each task with its index number (starting from 1)
        for (i, task) in tasks.iter().enumerate() {
            println!(
                "{}: [{}] {}",
                i + 1,
                if task.done { "x" } else { " " },
                task.description
            );
        }

        // Menu Options
        println!("\nOptions:");
        println!("1. Add task");
        println!("2. Mark task done");
        println!("3. Save & exit");

        // Ask the user what they want to do
        let choice = get_input("Enter choice: ");

        // Match on the user's input
        match choice.trim() {
            "1" => {
                let desc = get_input("Enter task description: ");
                tasks.push(Task {
                    description: desc,
                    done: false,
                });
            }
            "2" => {
                let index = get_input("Enter task number: ");
                if let Ok(num) = index.trim().parse::<usize>() {
                    // Check Valid Num
                    if num > 0 && num <= tasks.len() {
                        tasks[num - 1].done = true;
                        println!("Marked as done!");
                    } else {
                        println!("Invalid task number.");
                    }
                }
            }
            "3" => {
                save_tasks(&tasks);
                println!("Tasks saved, Goodbye!");
                break;
            }
            _ => println!("Invalid choice."),
        }
    }
}

// A helper function to ask the user for input and retunr it as a String
fn get_input(prompt: &str) -> String {
    // print prompt text with a new line
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim_end().to_string()
}

// Load tasks form "tasks.json" if it exists
fn load_tasks() -> Vec<Task> {
    let path = "tasks.json";
    if let Ok(file) = File::open(path) {
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}

// Save the current task list to "task.json"
fn save_tasks(tasks: &Vec<Task>) {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("tasks.json")
        .unwrap();

    serde_json::to_writer_pretty(file, tasks).unwrap();
}
