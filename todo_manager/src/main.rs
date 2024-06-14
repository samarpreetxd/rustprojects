use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

#[derive(Debug)]
struct Task {
    description: String,
    completed: bool,
}

impl Task {
    fn new(description: &str) -> Task {
        Task {
            description: description.to_string(),
            completed: false,
        }
    }

    fn from_file_line(line: &str) -> Task {
        let mut parts = line.splitn(2, '\t');
        let completed = parts.next().unwrap() == "true";
        let description = parts.next().unwrap_or("").to_string();
        Task { description, completed }
    }

    fn to_file_line(&self) -> String {
        format!("{}\t{}\n", self.completed, self.description)
    }
}

fn read_tasks() -> io::Result<Vec<Task>> {
    let file = File::open("tasks.txt");
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut tasks = Vec::new();
            for line in reader.lines() {
                let line = line?;
                let task = Task::from_file_line(&line);
                tasks.push(task);
            }
            Ok(tasks)
        }
        Err(_) => Ok(Vec::new()),
    }
}

fn write_tasks(tasks: &[Task]) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("tasks.txt")?;
    
    for task in tasks {
        file.write_all(task.to_file_line().as_bytes())?;
    }
    
    Ok(())
}

fn add_task(tasks: &mut Vec<Task>, description: &str) {
    let new_task = Task::new(description);
    tasks.push(new_task);
}

fn remove_task(tasks: &mut Vec<Task>, index: usize) {
    if index < tasks.len() {
        tasks.remove(index);
    } else {
        println!("Error: No task found at index {}", index);
    }
}

fn complete_task(tasks: &mut Vec<Task>, index: usize) {
    if index < tasks.len() {
        tasks[index].completed = true;
    } else {
        println!("Error: No task found at index {}", index);
    }
}

fn list_tasks(tasks: &[Task]) {
    for (index, task) in tasks.iter().enumerate() {
        let status = if task.completed { "[X]" } else { "[ ]" };
        println!("{} {} - {}", index, status, task.description);
    }
}

fn main() {
    let mut tasks = match read_tasks() {
        Ok(tasks) => tasks,
        Err(e) => {
            eprintln!("Error reading tasks: {}", e);
            return;
        }
    };

    loop {
        println!("\nCommands:");
        println!("  - add <description>: Add a new task");
        println!("  - remove <index>: Remove task at index");
        println!("  - complete <index>: Mark task at index as complete");
        println!("  - list: List all tasks");
        println!("  - exit: Exit the program");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let tokens: Vec<&str> = input.trim().splitn(2, ' ').collect();
        let command = tokens[0].to_lowercase();

        match command.as_str() {
            "add" => {
                if let Some(description) = tokens.get(1) {
                    add_task(&mut tasks, description);
                    println!("Task added successfully!");
                } else {
                    println!("Error: Missing task description");
                }
            }
            "remove" => {
                if let Some(index_str) = tokens.get(1) {
                    if let Ok(index) = index_str.parse::<usize>() {
                        remove_task(&mut tasks, index);
                    } else {
                        println!("Error: Invalid index");
                    }
                } else {
                    println!("Error: Missing index");
                }
            }
            "complete" => {
                if let Some(index_str) = tokens.get(1) {
                    if let Ok(index) = index_str.parse::<usize>() {
                        complete_task(&mut tasks, index);
                    } else {
                        println!("Error: Invalid index");
                    }
                } else {
                    println!("Error: Missing index");
                }
            }
            "list" => {
                list_tasks(&tasks);
            }
            "exit" => {
                println!("Exiting program...");
                if let Err(e) = write_tasks(&tasks) {
                    eprintln!("Error writing tasks: {}", e);
                }
                break;
            }
            _ => {
                println!("Invalid command");
            }
        }
    }
}
