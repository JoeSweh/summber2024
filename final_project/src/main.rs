use dotenv::dotenv;
use std::env;
use serde::{Deserialize, Serialize};
use ureq;
use std::io::{self, Write};

// Structs for API request and response
#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct CodeCompletionRequest {
    messages: Vec<Message>,
    max_tokens: u32,
}

#[derive(Deserialize)]
struct CodeCompletionResponse {
    choices: Vec<CompletionChoice>,
}

#[derive(Deserialize)]
struct CompletionChoice {
    message: Message,
}

fn send_api_request<T: Serialize, U: for<'de> Deserialize<'de>>(request: &T) -> Result<U, String> {
    let api_endpoint = env::var("API_ENDPOINT").map_err(|e| e.to_string())?;
    let api_key = env::var("API_KEY").map_err(|e| e.to_string())?;

    let request_json = serde_json::to_string(request).map_err(|e| e.to_string())?;

    let response = ureq::post(&api_endpoint)
        .set("Content-Type", "application/json")
        .set("api-key", &api_key)
        .send_string(&request_json);

    match response {
        Ok(resp) => {
            let status = resp.status();
            let response_text = match resp.into_string() {
                Ok(text) => text,
                Err(_) => "Unable to read response".to_string(),
            };
        

            if status == 200 {
                let parsed_response: U = serde_json::from_str(&response_text).map_err(|e| e.to_string())?;
                Ok(parsed_response)
            } else {
                Err(format!("Request failed with status {}: {}", status, response_text))
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

// Function for code completion

fn code_completion() -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter your partial code (type 'END' on a new line when finished):");

    let mut code = String::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        if line.trim() == "END" {
            break;
        }
        code.push_str(&line);
    }

    let request = CodeCompletionRequest {
        messages: vec![
            Message {
                role: "user".to_string(),
                content: code.trim().to_string(),
            },
        ],
        max_tokens: 800,
    };

    match send_api_request::<CodeCompletionRequest, CodeCompletionResponse>(&request) {
        Ok(response) => {
            if let Some(choice) = response.choices.first() {
                println!("Completion suggestion:");
                println!("{}", choice.message.content);
            } else {
                println!("No suggestions were returned by the API.");
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}
// Function for code explanation
fn code_explanation() -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter the code snippet you want explained (type 'END' on a new line when finished):");
    let mut code = String::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        if line.trim() == "END" {
            break;
        }
        code.push_str(&line);
    }

    let request = CodeCompletionRequest {
        messages: vec![
            Message {
                role: "user".to_string(),
                content: format!("Explain the following code: {}", code.trim()),
            },
        ],
        max_tokens: 400, // Adjust based on API requirements
    };

    match send_api_request::<CodeCompletionRequest, CodeCompletionResponse>(&request) {
        Ok(response) => {
            if let Some(choice) = response.choices.first() {
                println!("Explanation:");
                println!("{}", choice.message.content);
            } else {
                println!("No explanation provided.");
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}

// Function for refactoring suggestions
fn refactoring_suggestions() -> Result<(), String> {
    println!("Enter the code block you want refactored (type 'END' on a new line when finished):");
    let mut code = String::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).map_err(|e| e.to_string())?;
        if line.trim() == "END" {
            break;
        }
        code.push_str(&line);
    }

    let request = CodeCompletionRequest {
        messages: vec![
            Message {
                role: "user".to_string(),
                content: format!("Refactor the following code: {}", code.trim()),
            },
        ],
        max_tokens: 400, // Adjust based on API requirements
    };

    match send_api_request::<CodeCompletionRequest, CodeCompletionResponse>(&request) {
        Ok(response) => {
            if let Some(choice) = response.choices.first() {
                println!("Refactoring suggestion:");
                println!("{}", choice.message.content);
            } else {
                println!("No refactoring suggestions provided.");
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}

// Main function to run the program
fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    loop {
        println!("AI Code Assistant");
        println!("1. Code Completion");
        println!("2. Code Explanation");
        println!("3. Refactoring Suggestions");
        println!("4. Exit");
        print!("Choose an option: ");
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" => {
                if let Err(e) = code_completion() {
                    println!("Error during code completion: {}", e);
                }
            }
            "2" => {
                if let Err(e) = code_explanation() {
                    println!("Error during code explanation: {}", e);
                }
            }
            "3" => {
                if let Err(e) = refactoring_suggestions() {
                    println!("Error during refactoring suggestions: {}", e);
                }
            }
            "4" => break,
            _ => println!("Invalid option, please try again."),
        }
    }
    Ok(())
}