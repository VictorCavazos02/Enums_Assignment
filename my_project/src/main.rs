use std::io::{self, Write};
use std::process::Command;

enum FileOperation {
    List(String),           // Directory path
    Display(String),        // File path
    Create(String, String), // File path and content
    Remove(String),         // File path
    Pwd,                    // Print working directory
}

fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::List(dir) => {
            let result = Command::new("ls")
                .arg(&dir)
                .status();

            match result {
                Ok(status) if status.success() => {}
                _ => eprintln!("Failed to list directory."),
            }
        }

        FileOperation::Display(file) => {
            let result = Command::new("cat")
                .arg(&file)
                .status();

            match result {
                Ok(status) if status.success() => {}
                _ => eprintln!("Failed to display file."),
            }
        }

        FileOperation::Create(file, content) => {
            let command = format!("echo '{}' > {}", content, file);

            let result = Command::new("sh")
                .arg("-c")
                .arg(&command)
                .status();

            match result {
                Ok(status) if status.success() => {
                    println!("File '{}' created successfully.", file);
                }
                _ => eprintln!("Failed to create file."),
            }
        }

        FileOperation::Remove(file) => {
            let result = Command::new("rm")
                .arg(&file)
                .status();

            match result {
                Ok(status) if status.success() => {
                    println!("File '{}' removed successfully.", file);
                }
                _ => eprintln!("Failed to remove file."),
            }
        }

        FileOperation::Pwd => {
            let result = Command::new("pwd").status();

            match result {
                Ok(status) if status.success() => {}
                _ => eprintln!("Failed to get working directory."),
            }
        }
    }
}

fn get_input(prompt: &str) -> String {
    let mut input = String::new();

    print!("{}", prompt);
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

fn print_menu() {
    println!("\nFile Operations Menu:");
    println!("1. List files in a directory");
    println!("2. Display file contents");
    println!("3. Create a new file");
    println!("4. Remove a file");
    println!("5. Print working directory");
    println!("0. Exit");
}

fn main() {
    println!("Welcome to the File Operations Program!");

    loop {
        print_menu();

        let choice = get_input("\nEnter your choice (0-5): ");

        match choice.as_str() {
            "1" => {
                let dir = get_input("Enter directory path: ");
                let op = FileOperation::List(dir);
                perform_operation(op);
            }

            "2" => {
                let file = get_input("Enter file path: ");
                let op = FileOperation::Display(file);
                perform_operation(op);
            }

            "3" => {
                let file = get_input("Enter file path: ");
                let content = get_input("Enter content: ");
                let op = FileOperation::Create(file, content);
                perform_operation(op);
            }

            "4" => {
                let file = get_input("Enter file path: ");
                let op = FileOperation::Remove(file);
                perform_operation(op);
            }

            "5" => {
                let op = FileOperation::Pwd;
                perform_operation(op);
            }

            "0" => {
                println!("Goodbye!");
                break;
            }

            _ => {
                println!("Invalid option. Please enter a number between 0 and 5.");
            }
        }
    }
}