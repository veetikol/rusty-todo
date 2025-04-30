use std::env;
use std::fs::{self, OpenOptions, File};
use std::io::{BufRead, BufReader, Write};
use serde::{Deserialize, Serialize};
use chrono::Local;
use colored::*;

const TODO_FILE: &str = "todo.json";
const ID_COUNTER_FILE: &str = "id_counter.txt";

#[derive(Serialize, Deserialize)]
struct Item {
    id: u32,
    content: String,
    date: String,
    status: String,
}

fn load_todo_list(todo_file: &str) -> Vec<Item> {
    let file = OpenOptions::new()
        .read(true)
        .open(todo_file)
        .unwrap_or_else(|_| {
            File::create(todo_file).expect("Unable to create file");
            File::open(todo_file).expect("Unable to open file")
        });
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|line| serde_json::from_str(&line.unwrap_or_default()).ok())
        .collect::<Vec<Item>>()
}

fn save_todo_list(items: &[Item], todo_file: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(todo_file)
        .expect("Unable to opne todo.json");
    for item in items {
        writeln!(file, "{}", serde_json::to_string(item).expect("Unable to serialize item"))
            .expect("Unable to write file");
    }
}

fn reset_id(id_file: &str) {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(id_file)
        .expect("Unable to open id_counter");
    file.set_len(0).expect("Unable to clear file");
}

fn get_next_id(id_file: &str) -> u32 {
    let mut id = match fs::read_to_string(id_file) {
        Ok(content) => content.trim().parse::<u32>().unwrap_or(0),
        Err(_) => 0,
    };

    id += 1;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(id_file)
        .expect("unable to open id file");
    writeln!(file, "{}", id).expect("Unable to write to ID file");

    id
}

fn main() {
    let now = Local::now();
    let formatted_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("{}", "Invalid command, type 'help' for available commands.".red());
        return;
    }

    let command = &args[1];
    let mut todo_list = load_todo_list(TODO_FILE);

    if command == "add" {
        if args.len() < 3 {
            eprintln!("{}", "Please provide an item to add".red());
            return;
        }
        
        let text = &args[2];
        let item = Item {
            id: get_next_id(ID_COUNTER_FILE),
            content: text.to_string(),
            date: formatted_time,
            status: "undone".to_string(),
        };

        println!("{} {} {} {}", "Added task".green(), text, "with id:".green(), item.id.to_string().blue());
        todo_list.push(item);
        save_todo_list(&todo_list, TODO_FILE);
        

    } else if command == "markdone" {
        if args.len() < 3 {
            eprintln!("{}", "Please provide an item id to mark done".red());
            return;
        }

        let input = &args[2];
        let id: u32 = input.parse().expect("Failed to parse string to u32");
        if let Some(item) = todo_list.iter_mut().find(|item| item.id == id) {
            item.status = "done".to_string();
            println!("{} {} {}", "Item id".green(),  id, "marked done!".green());
        } else {
            eprintln!("{} {} {}", "Item id".red(), id, "does not exist".red());
        }
        save_todo_list(&todo_list, TODO_FILE);

    } else if command == "list" {
        if todo_list.len() == 0 {
            println!("{}", "Your todo list is empty".blue())
        } else {
            let title = "Your todo list:".blue();
            println!("{}", title);
            for item in &todo_list {
                if item.status == "done" {
                    println!("\x1b[9m{}: {} - {} - {}\x1b[0m", item.id, item.content, item.date, item.status);
                } else {
                    println!("{}. {}, {}, {}", item.id, item.content.bold(), item.date, item.status.red());
                }
            }
        }
        

    } else if command == "delete" {
        if args.len() < 3 {
            eprintln!("{}", "Please provide an item id to delete".red());
            return;
        }

        let input = &args[2];
        let id: u32 = input.parse().expect("Failed to parse string to u32");
        if let Some(pos) = todo_list.iter().position(|item| item.id == id) {
            todo_list.remove(pos);
            println!("{} {} {}", "Item id".green(), id, "deleted".green());
        } else {
            eprintln!("Item id: {} doesn't exist", id);
        }
        save_todo_list(&todo_list, TODO_FILE);

    } else if command == "help" {
        println!("Available commands:");
        println!("add <item> - Add a new item to the todo list");
        println!("markdone <id> - Mark an item as done");
        println!("delete <id> - Delete an item from the todo list");
        println!("clearlist - Wipe the todo list");
        println!("cleardone - Remove done items");
        println!("list - Display the todo list");
        println!("info - basic application info");
    } else if command == "clearlist" {
        todo_list.clear();
        reset_id(ID_COUNTER_FILE);
        save_todo_list(&todo_list, TODO_FILE);
        println!("{}", "Todo list cleared".green());
    } else if command == "cleardone" {
        todo_list.retain(|item| item.status == "undone");
        save_todo_list(&todo_list, TODO_FILE);
        println!("{}", "Cleared done items".green());  
    } else if command == "info" {
        println!("todo-rust 0.1 by Veeti Kolanen :)");
        println!("manage a list of things to do.");
        println!("type help for available commands!");
    } else {
        eprintln!("{}", "Invalid command, type 'help' for available commands.".red());
    }

}
