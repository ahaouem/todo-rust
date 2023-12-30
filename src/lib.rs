use chrono::{NaiveDate, ParseError};
use std::io::{self, Write};

pub struct Task {
    pub id: i8,
    pub title: String,
    pub description: String,
    pub due_date: NaiveDate,
    pub priority: String,
    pub completed: bool,
}

impl Task {
    fn new(
        id: i8,
        title: String,
        description: String,
        due_date: NaiveDate,
        priority: String,
    ) -> Task {
        Task {
            id,
            title,
            description,
            due_date,
            priority,
            completed: false,
        }
    }
}

pub fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn parse_date(date_str: &str) -> Result<NaiveDate, ParseError> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
}

pub fn add_task(tasks: &mut Vec<Task>, id: i8, priorities: &[String]) {
    let title = read_input("Task title: ");
    let description = read_input("Task description: ");
    let mut priority: String;

    loop {
        priority = read_input(&format!("Priority ({}) : ", priorities.join("/")));

        if priorities.contains(&priority.to_lowercase()) {
            break;
        } else {
            println!(
                "Invalid priority. Please choose from: {}",
                priorities.join(", ")
            );
        }
    }

    let due_date: NaiveDate;

    loop {
        let due_date_str = read_input("Due date (YYYY-MM-DD): ");
        match parse_date(&due_date_str) {
            Ok(date) => {
                if date < chrono::Utc::now().naive_utc().date() {
                    println!("Due date cannot be in the past. Please enter a future date.");
                } else {
                    due_date = date;
                    break;
                }
            }
            Err(_) => {
                println!("Invalid date format. Please use YYYY-MM-DD.");
            }
        }
    }

    let task = Task::new(id, title, description, due_date, priority);
    tasks.push(task);
    println!("Task added!");
}

pub fn display_tasks(tasks: &[Task]) {
    println!("\nTask list:");
    for task in tasks {
        let completed = if task.completed { "Yes" } else { "No" };
        println!(
            "ID: {}. Title: {}, Description: {}, Priority: {}, Due date: {}, Completed: {}",
            task.id, task.title, task.description, task.priority, task.due_date, completed
        );
    }
}

pub fn complete_task(tasks: &mut Vec<Task>) {
    let task_id = read_input("Enter the task ID to mark as completed: ");
    let task_id = match task_id.trim().parse::<i8>() {
        Ok(id) => id,
        Err(_) => {
            println!("Invalid task ID.");
            return;
        }
    };

    for task in tasks {
        if task.id == task_id {
            task.completed = true;
            println!("Task marked as completed!");
            return;
        }
    }

    println!("Task not found.");
}

pub fn edit(tasks: &mut Vec<Task>, priorities: &mut Vec<String>) {
    println!("What do you want to edit?");
    println!("1. Task");
    println!("2. Priorities");

    let choice = read_input("Choose an option: ");
    match choice.trim().parse::<u32>() {
        Ok(1) => {
            let id_str = read_input("Enter the ID of the task to edit: ");
            match id_str.trim().parse::<i8>() {
                Ok(id) => {
                    let task = tasks.iter_mut().find(|task| task.id == id);
                    match task {
                        Some(task) => {
                            edit_task_details(task, priorities);
                        }
                        None => {
                            println!("Task not found.");
                        }
                    }
                }
                Err(_) => {
                    println!("Invalid ID. Please enter a number.");
                }
            }
        }
        Ok(2) => {
            edit_priorities(priorities);
        }
        _ => {
            println!("Invalid option. Please choose again.");
        }
    }
}

pub fn edit_task_details(task: &mut Task, priorities: &Vec<String>) {
    let title = read_input("New task title: ");
    let description = read_input("New task description: ");
    let priority = read_input(&format!("Priority ({}) : ", priorities.join("/")));
    let due_date_str = read_input("New due date (YYYY-MM-DD): ");

    match parse_date(&due_date_str) {
        Ok(due_date) => {
            task.title = title;
            task.description = description;
            task.priority = priority;
            task.due_date = due_date;
            println!("Task details edited successfully!");
        }
        Err(_) => {
            println!("Invalid date format. Please use YYYY-MM-DD.");
        }
    }
}

pub fn edit_priorities(priorities: &mut Vec<String>) {
    println!("Please specify the priority you wish to edit. You may choose, for instance, 1 and 2 (e.g., '1 2').");
    for (index, priority) in priorities.iter().enumerate() {
        println!("{}. {}", index + 1, priority);
    }

    let choices_str = read_input("Choose an option: ");
    let choices: Vec<usize> = choices_str
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    let mut new_names: Vec<String> = Vec::new();

    for &choice in &choices {
        if choice > 0 && choice <= priorities.len() {
            let new_name = read_input(&format!(
                "Provide a new name for priority '{}': ",
                priorities[choice - 1]
            ));
            new_names.push(new_name);
        } else {
            println!("Invalid choice: {}. Ignoring.", choice);
        }
    }

    for (index, choice) in choices.into_iter().enumerate() {
        if choice > 0 && choice <= priorities.len() {
            priorities[choice - 1] = new_names[index].clone();
        }
    }

    println!("Priorities names have been changed!");
}

pub fn delete_task(tasks: &mut Vec<Task>) {
    let id_str = read_input("Enter the ID of the task to delete: ");
    match id_str.trim().parse::<i8>() {
        Ok(id) => {
            let index = tasks.iter().position(|task| task.id == id);
            match index {
                Some(index) => {
                    tasks.remove(index);
                    println!("Task deleted!");
                }
                None => {
                    println!("Task not found.");
                }
            }
        }
        Err(_) => {
            println!("Invalid ID. Please enter a number.");
        }
    }
}
