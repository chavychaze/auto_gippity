use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::{
    io::{stdin, stdout},
    os::macos::raw::stat,
};

#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    AICall,
    UnitTests,
    Issue,
}

impl PrintCommand {
    pub fn print_agent_message(&self, agent_pos: &str, agent_statement: &str) {
        let mut stdout: std::io::Stdout = stdout();

        // Decide on the print color
        let statement_color: Color = match self {
            Self::AICall => Color::Cyan,
            Self::UnitTests => Color::Magenta,
            Self::Issue => Color::Red,
        };

        // Print the agent statement in a specific color
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("Agent: {}: ", agent_pos);

        // Make selected color
        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{}", agent_statement);

        // Reset a color
        stdout.execute(ResetColor).unwrap();
    }
}

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
    stdin()
        .read_line(&mut user_response)
        .expect("Failed to read response");

    // Trim whitespace and return
    return user_response.trim().to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_print_agent_msg() {
        PrintCommand::AICall
            .print_agent_message("Managing Agent", "Testing testing, processing something");
    }
}
