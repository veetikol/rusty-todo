use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use chrono::Local;

struct Item {
    id: u32,
    content: String,
    date: String,
    status: String,
}

impl Item {
    fn new(id: &u32, content: &str, date: &str, status: &str) -> Self {
        Self {
            id: *id,
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
        
        let id = get_next_id();
        let text = &args[2];
        let item = Item::new(&id,text, &formatted_date, "undone");

        let item_data = format!("{}\n", item.display());
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open("todo.txt")
            .expect("Unable to open or create todo.txt");
        writeln!(file, "{}", item.display()).expect("Unable to write to file");

        println!("Added {}", item.display())
    }
}
