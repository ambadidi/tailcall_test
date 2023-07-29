use chrono::{Duration, Utc};
use std::env;
use std::io;
use std::io::Write;
use uuid::Uuid;

#[derive(Debug, Clone)]
struct Task {
    name: String,
    status: String,
    date: i64,
    // id: Uuid,
    id_string: String,
}
impl Task {
    fn new(name: String, id_string: String) -> Self {
        Self {
            name: name,
            status: String::from("Undone"),
            date: Utc::now().timestamp(),
            // id: Uuid::new_v4(),
            id_string: id_string,
        }
    }
}
#[derive(Debug, Clone)]
struct ListTask {
    list: Vec<Task>,
}
impl ListTask {
    fn new() -> Self {
        Self { list: Vec::new() }
    }
    fn add_to_list(&mut self, task: Task) {
        self.list.push(task);
    }
}
fn main() {
    // let args: Vec<String> = env::args().collect();
    // let command = &args[1];
    let mut list_task = ListTask::new();

    loop {
        println!("What is your Task operation?\n[Add]\n[Update]\n[List]\n[Delete]\n");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                // Trim the input to remove the trailing newline character
                let trimmed_input = input.trim();

                if trimmed_input == "Add" {
                    // println!("test Add");
                    print!("Enter the name of the new Task: ");
                    io::stdout().flush().unwrap();
                    let mut task_name = String::new();

                    match io::stdin().read_line(&mut task_name) {
                        Ok(_) => {
                            let trimmed_task_name = task_name.trim();

                            let task = Task::new(
                                trimmed_task_name.to_string(),
                                Uuid::new_v4().to_string().as_str()[0..5].to_string(),
                            );
                            list_task.add_to_list(task.clone());
                            println!("New Task {}, Status: {}", task.name, task.status);
                        }
                        Err(error) => println!("Error reading input: {}", error),
                    }
                    //
                } else if trimmed_input == "Update" {
                    println!("test Update");
                } else if trimmed_input == "List" {
                    println!("List of Tasks:");
                    for task in &list_task.list {
                        println!(
                            "Id: {} => Task name: {} => STATUS: {}",
                            task.id_string, task.name, task.status
                        );
                    }
                } else if trimmed_input == "Delete" {
                    print!("Provide the id to  Delete: ");
                    io::stdout().flush().unwrap();
                    let mut input = String::new();

                    // Read the line of input from the user
                    match io::stdin().read_line(&mut input) {
                        Ok(_) => {
                            // Trim the input to remove the trailing newline character
                            let trimmed_input = input.trim();

                            // Do something with the input
                            println!("You entered: {}", trimmed_input);
                        }
                        Err(error) => println!("Error reading input: {}", error),
                    }
                    // delte from list
                    // sync
                } else {
                    println!("Wrong Operstion");
                }
            }
            Err(error) => println!("Error reading input: {}", error),
        }
    }
}
