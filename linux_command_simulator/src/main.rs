use std::fs::OpenOptions;
use std::io::{self, Write};
use std::process::Command;
use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};
use std::os::unix::process::ExitStatusExt; // Import the trait for Unix platforms

struct CommandResult {
    command: String,
    output: String,
}

fn main() {
    println!("Name: Jose F. Gonzalez Jr.");
    println!("Linux Command Simulator");
    println!("Type your commands below (type 'exit' to stop):");

    let (tx, rx): (Sender<CommandResult>, Receiver<CommandResult>) = mpsc::channel();
    let mut handles = vec![];

    loop {
        // Read command from user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let command = input.trim();

        if command.eq_ignore_ascii_case("exit") {
            break;
        }

        let command = command.to_string();
        let tx = tx.clone();

        // Spawn a thread to execute the command
        let handle = thread::spawn(move || {
            let output = execute_command(&command);
            tx.send(output).expect("Failed to send command result");
        });

        handles.push(handle);
    }

    // Wait for all threads to finish execution
    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    // Collect all command results
    let mut results = vec![];
    for result in rx.try_iter() {
        results.push(result);
    }

    // Save the results to a file
    save_results_to_file(&results);

    // Display all command results to the user
    println!("\nCommand Execution Summary:");
    for result in results {
        println!("\nCommand: {}", result.command);
    }
}

fn execute_command(command: &str) -> CommandResult {
    let parts: Vec<&str> = command.split_whitespace().collect();

    if parts.is_empty() {
        return CommandResult {
            command: command.to_string(),
            output: String::from("Invalid command"),
        };
    }

    let mut cmd = Command::new(parts[0]);
    if parts.len() > 1 {
        cmd.args(&parts[1..]);
    }

    // Execute the command and capture the output
    let output = cmd.output().unwrap_or_else(|_| {
        std::process::Output {
            stdout: Vec::new(),
            stderr: Vec::new(),
            status: std::process::ExitStatus::from_raw(0), // This requires the `ExitStatusExt` trait
        }
    });

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();

    CommandResult {
        command: command.to_string(),
        output: stdout,
    }
}

fn save_results_to_file(results: &[CommandResult]) {
    let file_path = "command_results.txt";
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(file_path);

    match file {
        Ok(mut file) => {
            for result in results {
                if let Err(e) = writeln!(file, "Command: {}", result.command) {
                    eprintln!("Couldn't write to file: {}", e);
                }
                if let Err(e) = writeln!(file, "Output: {}", result.output) {
                    eprintln!("Couldn't write to file: {}", e);
                }
            }
        }
        Err(e) => eprintln!("Failed to open file: {}", e),
    }
}