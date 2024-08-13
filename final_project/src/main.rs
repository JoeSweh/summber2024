use dotenv::dotenv;
use std::env;
use serde::{Deserialize, Serialize};
use ureq;
use std::io::{self, Write};

// Structs for different request types
#[derive(Serialize)]
struct CodeCompletionRequest {
    prompt: String,
    max_tokens: u32,
}

#[derive(Deserialize)]
struct CodeCompletionResponse {
    choices: Vec<CompletionChoice>,
}

#[derive(Deserialize)]
struct CompletionChoice {
    text: String,
}

#[derive(Serialize)]
struct CodeExplanationRequest {
    code: String,
}

#[derive(Deserialize)]
struct CodeExplanationResponse {
    explanation: String,
}

#[derive(Serialize)]
struct RefactoringRequest {
    code: String,
}

#[derive(Deserialize)]
struct RefactoringResponse {
    suggestions: String,
}

fn code_completion() -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter your partial code (type 'END' on a new line when finished):");
    
    let mut code = String::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;  // Handle error with `?` instead of .unwrap()
        if line.trim() == "END" {
            break;
        }
        code.push_str(&line);
    }

    let request = CodeCompletionRequest {
        prompt: code,
        max_tokens: 100,
    };

    match send_code_completion_request(&request) {
        Ok(response) => {
            println!("Completion suggestion:");
            println!("{}", response.choices[0].text);
        }
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}

fn code_explanation() -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter the code you want explained (type 'END' on a new line when finished):");
    
    let mut code = String::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;  // Handle error with `?` instead of .unwrap()
        if line.trim() == "END" {
            break;
        }
        code.push_str(&line);
    }

    let request = CodeExplanationRequest {
        code: code,
    };

    match send_code_explanation_request(&request) {
        Ok(response) => {
            println!("Code explanation:");
            println!("{}", response.explanation);
        }
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}

fn refactoring_suggestions() -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter the code you want refactored (type 'END' on a new line when finished):");
    
    let mut code = String::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;  // Handle error with `?` instead of .unwrap()
        if line.trim() == "END" {
            break;
        }
        code.push_str(&line);
    }

    let request = RefactoringRequest {
        code: code,
    };

    match send_refactoring_request(&request) {
        Ok(response) => {
            println!("Refactoring suggestions:");
            println!("{}", response.suggestions);
        }
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}

// Function for sending code completion API requests
fn send_code_completion_request(request: &CodeCompletionRequest) -> Result<CodeCompletionResponse, Box<dyn std::error::Error>> {
    let api_key = env::var("API_KEY")?;
    let api_endpoint = env::var("API_ENDPOINT")?;

    let response: CodeCompletionResponse = ureq::post(&format!("{}/completions", api_endpoint))
        .set("Authorization", &format!("Bearer {}", api_key))
        .send_json(serde_json::to_value(request)?)?
        .into_json()?;

    Ok(response)
}

// Function for sending code explanation API requests
fn send_code_explanation_request(request: &CodeExplanationRequest) -> Result<CodeExplanationResponse, Box<dyn std::error::Error>> {
    let api_key = env::var("API_KEY")?;
    let api_endpoint = env::var("API_ENDPOINT")?;

    let response: CodeExplanationResponse = ureq::post(&format!("{}/explain", api_endpoint))
        .set("Authorization", &format!("Bearer {}", api_key))
        .send_json(serde_json::to_value(request)?)?
        .into_json()?;

    Ok(response)
}

// Function for sending refactoring API requests
fn send_refactoring_request(request: &RefactoringRequest) -> Result<RefactoringResponse, Box<dyn std::error::Error>> {
    let api_key = env::var("API_KEY")?;
    let api_endpoint = env::var("API_ENDPOINT")?;

    let response: RefactoringResponse = ureq::post(&format!("{}/refactor", api_endpoint))
        .set("Authorization", &format!("Bearer {}", api_key))
        .send_json(serde_json::to_value(request)?)?
        .into_json()?;

    Ok(response)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();  // Load environment variables
    
    loop {
        println!("AI Code Assistant");
        println!("1. Code Completion");
        println!("2. Code Explanation");
        println!("3. Refactoring Suggestions");
        println!("4. Exit");
        print!("Choose an option: ");
        
        io::stdout().flush()?;  // Handle error with `?` instead of .unwrap()

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" => code_completion()?,
            "2" => code_explanation()?,
            "3" => refactoring_suggestions()?,
            "4" => break,
            _ => println!("Invalid option, please try again."),
        }
    }
    Ok(())
}