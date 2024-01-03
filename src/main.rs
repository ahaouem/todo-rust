use todo_lib::{add_task, complete_task, delete_task, display_tasks, edit, read_input, Task};

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    let mut id = 0;

    let mut priorities: Vec<String> =
        vec!["low".to_string(), "medium".to_string(), "high".to_string()];

    loop {
        println!("\nMenu:");
        println!("1. Add task");
        println!("2. Display tasks");
        println!("3. Complete task");
        println!("4. Edit");
        println!("5. Delete task");
        println!("6. Quit");

        let choice = read_input("Choose an option: ");

        match choice.trim().to_lowercase().as_str() {
            "1" | "add task" | "add" => {
                id += 1;
                add_task(&mut tasks, id, &priorities);
            }
            "2" | "display tasks" | "display" => {
                display_tasks(&tasks);
            }
            "3" | "complete task" | "complete" => {
                complete_task(&mut tasks);
            }
            "4" | "edit" => {
                edit(&mut tasks, &mut priorities);
            }
            "5" | "delete task" | "delete" => {
                delete_task(&mut tasks);
            }
            "6" | "quit" => {
                break;
            }
            _ => {
                println!("Invalid option. Please choose again.");
            }
        }
    }
}
