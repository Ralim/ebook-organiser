use std::io::Write;

pub fn prompt(prompt: &str) -> String {
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    while buffer.is_empty() {
        print!("{}> ", prompt);
        std::io::stdout().flush().expect("Failed to flush stdout");
        stdin.read_line(&mut buffer).expect("Failed to read line");
        buffer = buffer.trim().to_string();
    }
    buffer
}

pub fn prompt_bool(prompt_text: &str) -> bool {
    loop {
        let response = prompt(prompt_text);
        match response.to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Please answer 'y' or 'n'."),
        }
    }
}
