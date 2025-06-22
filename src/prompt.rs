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

pub fn prompt_select_other(prompt_text: &str, options: &[String]) -> String {
    let mut full_prompt = prompt_text.to_owned()
        + "please select one or more of the following (comma seperated) or type 'other':\r\n";
    for (i, option) in options.iter().enumerate() {
        full_prompt += &format!("[{i}]: {}\r\n", option);
    }
    loop {
        let response = prompt(&full_prompt);
        // if response is a number check if we can parse it as an index
        if response == "other" {
            return prompt("Please specify your option");
        } else if let Ok(index) = response.parse::<usize>() {
            if index < options.len() {
                return options[index].to_string();
            } else {
                println!("Index out of range. Please select a valid index.");
            }
        } else if response.contains(',') {
            // Multiple selection,
            // Split by comma, pick each option from the options list by this index, then join them with '&'
            let indices: Vec<usize> = response
                .split(',')
                .map(str::trim)
                .filter_map(|s| s.parse::<usize>().ok())
                .collect();
            if indices.iter().all(|&i| i < options.len()) {
                return indices
                    .iter()
                    .map(|&i| options[i].to_string())
                    .collect::<Vec<String>>()
                    .join(" & ");
            } else {
                println!("One or more indices are out of range. Please select valid indices.");
            }
        }
        println!("Input cannot be empty. Please select an option.");
    }
}
