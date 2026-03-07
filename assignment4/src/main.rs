use std::io::{self, Write};
use std::process::Command;

enum FileOperation {
    List(String),
    Display(String),
    Create(String, String),
    Remove(String),
    Pwd,
    Exit,
}

fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::List(directory_path) => {
            let result = Command::new("ls").arg(&directory_path).output();

            match result {
                Ok(output) => {
                    if output.status.success() {
                        print!("{}", String::from_utf8_lossy(&output.stdout));
                    } else {
                        eprintln!("Failed to list files.");
                        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                    }
                }
                Err(e) => eprintln!("Error executing ls: {}", e),
            }
        }

        FileOperation::Display(file_path) => {
            let result = Command::new("cat").arg(&file_path).output();

            match result {
                Ok(output) => {
                    if output.status.success() {
                        print!("{}", String::from_utf8_lossy(&output.stdout));
                    } else {
                        eprintln!("Failed to display file contents.");
                        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                    }
                }
                Err(e) => eprintln!("Error executing cat: {}", e),
            }
        }

        FileOperation::Create(file_path, content) => {
            let command = format!("echo '{}' > {}", content, file_path);
            let result = Command::new("sh")
                .arg("-c")
                .arg(&command)
                .output();

            match result {
                Ok(output) => {
                    if output.status.success() {
                        println!("File '{}' created successfully.", file_path);
                    } else {
                        eprintln!("Failed to create file.");
                        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                    }
                }
                Err(e) => eprintln!("Error creating file: {}", e),
            }
        }

        FileOperation::Remove(file_path) => {
            let result = Command::new("rm").arg(&file_path).output();

            match result {
                Ok(output) => {
                    if output.status.success() {
                        println!("File '{}' removed successfully.", file_path);
                    } else {
                        eprintln!("Failed to remove file.");
                        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                    }
                }
                Err(e) => eprintln!("Error executing rm: {}", e),
            }
        }

        FileOperation::Pwd => {
            let result = Command::new("pwd").output();

            match result {
                Ok(output) => {
                    if output.status.success() {
                        print!("Current working directory: ");
                        print!("{}", String::from_utf8_lossy(&output.stdout));
                    } else {
                        eprintln!("Failed to get working directory.");
                    }
                }
                Err(e) => eprintln!("Error executing pwd: {}", e),
            }
        }

        FileOperation::Exit => {
            println!("Goodbye!");
        }
    }
}

fn read_input(prompt: &str) -> String {
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

        let choice = read_input("Enter your choice (0-5): ");

        let operation = match choice.as_str() {
            "1" => {
                let dir = read_input("Enter directory path: ");
                FileOperation::List(dir)
            }
            "2" => {
                let file = read_input("Enter file path: ");
                FileOperation::Display(file)
            }
            "3" => {
                let file = read_input("Enter file path: ");
                let content = read_input("Enter content: ");
                FileOperation::Create(file, content)
            }
            "4" => {
                let file = read_input("Enter file path: ");
                FileOperation::Remove(file)
            }
            "5" => FileOperation::Pwd,
            "0" => FileOperation::Exit,
            _ => {
                println!("Invalid option. Please try again.");
                continue;
            }
        };

        if let FileOperation::Exit = operation {
            perform_operation(operation);
            break;
        } else {
            perform_operation(operation);
        }
    }
}