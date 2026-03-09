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
        FileOperation::List(directory_path) => {
            let output = Command::new("ls")
                .arg(&directory_path)
                .output();

            match output {
                Ok(result) => {
                    if result.status.success() {
                        println!("{}", String::from_utf8_lossy(&result.stdout));
                    } else {
                        eprintln!("Failed to list files.");
                    }
                }
                Err(_) => eprintln!("Failed to execute ls."),
            }
        }

        FileOperation::Display(file_path) => {
            let output = Command::new("cat")
                .arg(&file_path)
                .output();

            match output {
                Ok(result) => {
                    if result.status.success() {
                        println!("{}", String::from_utf8_lossy(&result.stdout));
                    } else {
                        eprintln!("Failed to display file contents.");
                    }
                }
                Err(_) => eprintln!("Failed to execute cat."),
            }
        }

        FileOperation::Create(file_path, content) => {
            let command = format!("echo '{}' > {}", content, file_path);

            let output = Command::new("sh")
                .arg("-c")
                .arg(&command)
                .output();

            match output {
                Ok(result) => {
                    if result.status.success() {
                        println!("File '{}' created successfully.", file_path);
                    } else {
                        eprintln!("Failed to create file.");
                    }
                }
                Err(_) => eprintln!("Failed to execute file creation command."),
            }
        }

        FileOperation::Remove(file_path) => {
            let output = Command::new("rm")
                .arg(&file_path)
                .output();

            match output {
                Ok(result) => {
                    if result.status.success() {
                        println!("File '{}' removed successfully.", file_path);
                    } else {
                        eprintln!("Failed to remove file.");
                    }
                }
                Err(_) => eprintln!("Failed to execute rm."),
            }
        }

        FileOperation::Pwd => {
            let output = Command::new("pwd").output();

            match output {
                Ok(result) => {
                    if result.status.success() {
                        print!("Current working directory: ");
                        println!("{}", String::from_utf8_lossy(&result.stdout));
                    } else {
                        eprintln!("Failed to get working directory.");
                    }
                }
                Err(_) => eprintln!("Failed to execute pwd."),
            }
        }
    }
}

fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    println!("Welcome to the File Operations Program!");

    loop {
        println!("\nFile Operations Menu:");
        println!("1. List files in a directory");
        println!("2. Display file contents");
        println!("3. Create a new file");
        println!("4. Remove a file");
        println!("5. Print working directory");
        println!("0. Exit");

        let choice = get_input("\nEnter your choice (0-5): ");

        match choice.as_str() {
            "1" => {
                let dir = get_input("Enter directory path: ");
                let operation = FileOperation::List(dir);
                perform_operation(operation);
            }

            "2" => {
                let file = get_input("Enter file path: ");
                let operation = FileOperation::Display(file);
                perform_operation(operation);
            }

            "3" => {
                let file = get_input("Enter file path: ");
                let content = get_input("Enter content: ");
                let operation = FileOperation::Create(file, content);
                perform_operation(operation);
            }

            "4" => {
                let file = get_input("Enter file path: ");
                let operation = FileOperation::Remove(file);
                perform_operation(operation);
            }

            "5" => {
                let operation = FileOperation::Pwd;
                perform_operation(operation);
            }

            "0" => {
                println!("Goodbye!");
                break;
            }

            _ => {
                println!("Invalid choice. Please enter a number from 0 to 5.");
            }
        }
    }
}