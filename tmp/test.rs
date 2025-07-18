use std::process::Command;

fn execute_command(cmd: &str) {
    // Vulnerable: command is constructed from a variable
    Command::new(cmd).spawn().expect("Failed to start command");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_command() {
        // This should be ignored by the Semgrep rule
        let command = "echo";
        execute_command(command);
    }
}

#[test]
fn another_test() {
    // This should also be ignored
    let cmd = "ls";
    Command::new(cmd).spawn().unwrap();
}

fn main() {
    let user_input = "ls"; // In a real scenario, this could come from user input
    execute_command(user_input);
}