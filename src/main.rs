use std::env;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use chrono::Local;

#[derive(Serialize, Deserialize)]
struct Item {
    id: u32,
    content: String,
    date: String,
    status: String,
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

fn mark_done(id: u32) {
    let file = OpenOptions::new()
        .read(true)
        .open("todo.json")
        .expect("Unable to open todo.txt");
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = reader.lines()
        .map(|line| line.expect("Unable to read line"))
        .collect();
    
    let mut updated_lines: Vec<String> = Vec::new();
    let mut item_found = false;

    for line in lines {
        let mut item: Item = serde_json::from_str(&line).expect("Unable to parse JSON");
        if item.id == id {
            item.status = "done".to_string();
            item_found = true;
        }
        updated_lines.push(serde_json::to_string(&item).expect("Unable to serialize JSON"))
    }

    if !item_found {
        eprintln!("Item id: {} doesn't exist", id);
        return;
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("todo.json")
        .expect("Unable to open todo.json for writing");
    for line in updated_lines {
        writeln!(file, "{}", line).expect("Unable to write to file");
    }
    
    println!("Item id {} marked done.", id);

}

fn add_to_file(item: Item) -> Result<()> {
    let j = serde_json::to_string(&item)?;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("todo.json")
        .expect("Unable to open or create todo.json");
    writeln!(file, "{}", j).expect("Unable to write to file");
    Ok(())
}

fn main() {
    let now = Local::now();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Invalid command, type 'help' for available commands.");
        return;
    }

    let command = &args[1];

    if command == "add" {
        if args.len() < 3 {
            eprintln!("Please provide an item to add");
            return;
        }
        
        let text = &args[2];
        let item = Item {
            id: get_next_id(),
            content: text.to_string(),
            date: now.to_string(),
            status: "undone".to_string(),
        };

        add_to_file(item);
    } else if command == "markdone" {
        if args.len() < 3 {
            eprintln!("Please provide an item id to mark done");
            return;
        }

        let input = &args[2];
        let id: u32 = input.parse().expect("Failed to parse string to u32");
        mark_done(id);
    }

}
