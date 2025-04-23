use std::env;
use std::fs::{self, OpenOptions, File};
use std::io::{BufRead, BufReader, Write};
use serde::{Deserialize, Serialize};
use chrono::Local;

#[derive(Serialize, Deserialize)]
enum Status {
    Undone,
    Done,
}

#[derive(Serialize, Deserialize)]
struct Item {
    id: u32,
    content: String,
    date: String,
    status: String,
}

fn load_todo_list() -> Vec<Item> {
    let file = OpenOptions::new()
        .read(true)
        .open("todo.json")
        .unwrap_or_else(|_| {
            File::create("todo.json").expect("Unable to create file");
            File::open("todo.json").expect("Unable to open file")
        });
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|line| serde_json::from_str(&line.unwrap_or_default()).ok())
        .collect::<Vec<Item>>()
}

fn save_todo_list(items: &[Item]) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("todo.json")
        .expect("Unable to opne todo.json");
    for item in items {
        writeln!(file, "{}", serde_json::to_string(item).expect("Unable to serialize item"))
            .expect("Unable to write file");
    }
}

fn reset_id() {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("id_counter.txt")
        .expect("Unable to open id_counter");
    file.set_len(0).expect("Unable to clear file");
}

fn get_next_id() -> u32 {
    let id_file = "id_counter.txt";

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
        eprintln!("Invalid command, type 'help' for available commands.");
        return;
    }

    let command = &args[1];
    let mut todo_list = load_todo_list();

    if command == "add" {
        if args.len() < 3 {
            eprintln!("Please provide an item to add");
            return;
        }
        
        let text = &args[2];
        let item = Item {
            id: get_next_id(),
            content: text.to_string(),
            date: formatted_time,
            status: "undone".to_string(),
        };

        todo_list.push(item);
        save_todo_list(&todo_list);

    } else if command == "markdone" {
        if args.len() < 3 {
            eprintln!("Please provide an item id to mark done");
            return;
        }

        let input = &args[2];
        let id: u32 = input.parse().expect("Failed to parse string to u32");
        if let Some(item) = todo_list.iter_mut().find(|item| item.id == id) {
            item.status = "done".to_string();
            println!("Item id {} marked done.",  id);
        } else {
            eprintln!("Item id {} does not exist", id);
        }
        save_todo_list(&todo_list);

    } else if command == "list" {
        println!("Your todo list:");
        for item in &todo_list {
            if item.status == "done" {
                println!("\x1b[9m{}: {} - {} - {}~\x1b[0m", item.id, item.content, item.date, item.status);
            } else {
                println!("{}. {}, {}, {}", item.id, item.content, item.date, item.status);
            }
        }

    } else if command == "delete" {
        if args.len() < 3 {
            eprintln!("Please provide an item id to delete");
            return;
        }

        let input = &args[2];
        let id: u32 = input.parse().expect("Failed to parse string to u32");
        if let Some(pos) = todo_list.iter().position(|item| item.id == id) {
            todo_list.remove(pos);
            println!("Item id {} deleted.", id);
        } else {
            eprintln!("Item id: {} doesn't exist", id);
        }
        save_todo_list(&todo_list);

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
        reset_id();
        save_todo_list(&todo_list);
        println!("Todo list cleared.")
    } else if command == "cleardone" {
        todo_list.retain(|item| item.status == "undone");
        save_todo_list(&todo_list);
        println!("Cleared done items");  
    } else if command == "info" {
        println!("todo-rust 0.1 by Veeti Kolanen :)");
        println!("manage a list of things to do.");
        println!("type help for available commands!");
    } else {
        eprintln!("Invalid command, type 'help' for available commands.");
    }

}
