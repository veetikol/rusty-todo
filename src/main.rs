use std::env;
use std::fs;
use chrono::Local;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref ITEM_ID_COUNTER: Mutex<u32> = Mutex::new(0);
}

struct Item {
    id: u32,
    content: String,
    date: String,
    status: String,
}

impl Item {
    fn new(icontent: &str, date: &str, status: &str) -> Self {
        let mut counter = ITEM_ID_COUNTER.lock().unwrap();
        *counter += 1;
        let id = *counter;

        Self {
            id,
            content: content.to_string(),
            date: date.to_string(),
            status: status.to_string()
        }
    }

    fn display(&self) -> String {
        format!(
            "{}: {} (added on {}, status: {})",
            self.id, self.content, self.date, self.status
        )
    }
}

fn main() {
    let now = Local::now();
    let formatted_date = now.format("%Y-%m-%d %H:%M:%S").to_string();
    let valid_commands= ["add", "markdone", "delete"];

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
        let item = Item::new(text, &formatted_date, "undone");

        let item_data = format!("{}\n", item.display());
        fs::write("todo.txt", item_data).expect("Unable to write to file");
        
        println!("Added {}" item.display())
    }
}
