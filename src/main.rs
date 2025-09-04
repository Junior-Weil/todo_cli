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
    let mut task = load_task();

    // main loop, keep showing the menu until a user chooses to exit
}

