use crossterm::{ style::{ Color, ResetColor, SetForegroundColor }, ExecutableCommand };
use std::io::{ stdin, stdout };

// Get User request
pub fn get_user_response(question: &str) -> String {
    let mut stdout: std::io::Stdout = stdout();

    // Print a question in a specific color
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("");
    println!("{}", question);

    // Reset a Color
    stdout.execute(ResetColor).unwrap();

    // Read user input
    let mut user_response: String = String::new();
    stdin().read_line(&mut user_response).expect("Failed to read response");

    // Trim whitespace and return
    return user_response.trim().to_string();
}
