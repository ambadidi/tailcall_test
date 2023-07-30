use chrono::{Duration, Utc};
// use std::env;
use std::io;
use std::io::Write;
use uuid::Uuid;
use std::fs::write;
use serde::Serialize;
use serde_json::to_string_pretty;
use std::fs::read_to_string;
use std::error::Error;
use serde_json::from_str;
use serde::Deserialize;
#[derive(Serialize, Clone,Deserialize)]
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
#[derive(Serialize, Clone,Deserialize)]
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
    let mut list_task; 
    // let mut list_task1; [1,2]
    list_task =  read_tasks_from_file("data.json").unwrap();
    let file_path = "data.json";
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
                            match write_list_task_to_file(&list_task, file_path) {
                                Ok(_) => println!("Data written to {} successfully.", file_path),
                                Err(e) => eprintln!("Error writing data to {}: {}", file_path, e),
                            }
                        }
                        Err(error) => println!("Error reading input: {}", error),
                    }
                    //
                } else if trimmed_input == "Update" {
                    println!("test Update");
                    print!("Provide the id to Update: ");
                    io::stdout().flush().unwrap();
                    let mut id_to_update = String::new();

                    match io::stdin().read_line(&mut id_to_update) {
                        Ok(_) => {
                            
                            let trimmed_id_to_update = id_to_update.trim();

                            // Do something with the input
                            let index_to_update = search_by_id(&list_task, trimmed_id_to_update.to_string());
                            if index_to_update == -1 {
                                println!("NO TASK with THIS id");
                            } else {
                                print!("Update name: ");
                                io::stdout().flush().unwrap();
                                let mut name_to_update = String::new();
                                match io::stdin().read_line(&mut name_to_update) {
                                    Ok(_) => {
                                        
                                        let trimmed_name_to_update = name_to_update.trim();
                                        list_task.list[index_to_update as usize].name = trimmed_name_to_update.to_string();
                                    }
                                    Err(error) => println!("Error reading input: {}", error),
                                }
                                println!("Task Updated!");
                                match write_list_task_to_file(&list_task, file_path) {
                                    Ok(_) => println!("Data written to {} successfully.", file_path),
                                    Err(e) => eprintln!("Error writing data to {}: {}", file_path, e),
                                }
                            }
                            
                        }
                        Err(error) => println!("Error reading id to Update: {}", error),
                    }
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
                    let mut id_to_delete = String::new();

                    match io::stdin().read_line(&mut id_to_delete) {
                        Ok(_) => {
                            
                            let trimmed_id_to_delete = id_to_delete.trim();

                            // Do something with the input
                            let index_to_delete = search_by_id(&list_task, trimmed_id_to_delete.to_string());
                            if index_to_delete == -1 {
                                println!("NO TASK with THIS id");
                            } else {
                                list_task.list.remove(index_to_delete as usize);
                                println!("Task deleted!");
                                match write_list_task_to_file(&list_task, file_path) {
                                    Ok(_) => println!("Data written to {} successfully.", file_path),
                                    Err(e) => eprintln!("Error writing data to {}: {}", file_path, e),
                                }
                            }
                            
                        }
                        Err(error) => println!("Error reading id to Delete: {}", error),
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

fn search_by_id(list: &ListTask,id: String) -> i32 {
    if !list.list.is_empty() {
        for (i, task) in list.list.iter().enumerate() {
            if task.id_string == id {
                return i as i32;
            }
        }
    } 
    return -1;
}

fn write_list_task_to_file(list_task: &ListTask, file_path: &str) -> Result<(), Box<dyn Error>> {
    
    let json_string = to_string_pretty(list_task)?;

    
    write(file_path, json_string)?;

    Ok(())
}





fn read_tasks_from_file(file_path: &str) -> Result<ListTask, Box<dyn Error>> {
    // Read the JSON data from the file as a string
    let json_string = read_to_string(file_path)?;
    // println!("dd {}",json_string);
    if json_string.trim().is_empty() {
        
        return Ok(ListTask::new()); // Return an empty Vec<Task>
    }
    // Deserialize the JSON string into a Vec<Task>
    let tasks:ListTask = from_str(&json_string)?;
   // println!("here {}", tasks.len());
    Ok(tasks)
}