# todo-rust

todo-rust is a simple and lightweight command-line application for managing tasks, written in Rust. It allows you to efficiently add, list, mark as done, delete, and clear tasks.

## Features

- **Add Tasks**: Create tasks with a unique ID and timestamp.
- **Mark as Done**: Mark tasks as completed.
- **Delete Tasks**: Remove tasks by their ID.
- **Display Tasks**: View all tasks, with completed tasks styled differently.
- **Clear List**: Remove all tasks from the list.
- **Lightweight**: Minimal dependencies and easy to use.

## Installation

1. Ensure you have [Rust](https://www.rust-lang.org/) installed on your system.
2. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/todo-rust.git
   ```
3. Navigate to the project directory:
   ```bash
   cd todo-rust
   ```
4. Build the project:
   ```bash
   cargo build --release
   ```
5. Run the application:
   ```bash
   ./target/release/todo-rust
   ```

After these steps you might want to add the executable to your $PATH

## Usage

Run the application with one of the following commands:

- **Add a task**:
  ```bash
  ./todo-rust add "Your task description"
  ```
- **Mark a task as done**:
  ```bash
  ./todo-rust markdone <task_id>
  ```
- **Delete a task**:
  ```bash
  ./todo-rust delete <task_id>
  ```
- **List all tasks**:
  ```bash
  ./todo-rust list
  ```
- **Clear the task list**:
  ```bash
  ./todo-rust clearlist
  ```
- **View help**:
  ```bash
  ./todo-rust help
  ```
- **View application info**:
  ```bash
  ./todo-rust info
  ```

## Example

Usage example:

1. Add a task:
   ```bash
   ./todo-rust add "Buy groceries"
   ```
2. List tasks:
   ```bash
   ./todo-rust list
   ```
   Output:
   ```
   Your todo list:
   1. Buy groceries, 2025-04-01 12:00:00, undone
   ```
3. Mark the task as done:
   ```bash
   ./todo-rust markdone 1
   ```
4. List tasks again:
   ```bash
   ./todo-rust list
   ```
   Output:
   ```
   Your todo list:
   ~1: Buy groceries - 2025-04-01 12:00:00 - done~
   ```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
