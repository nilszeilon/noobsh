use std::io::{self, Write};
use std::process::{Command, Stdio};

//TODO autocomplete things most common phrases/commands etc
//TODO add some color to things
//TODO read file formats like csv for ease of reading
//TODO add support for python notebook ish things
//TODO alot of error handling! and make it more robust
fn main() {
    let mut current_dir = std::env::current_dir().unwrap();
    let mut previous_dir = current_dir.clone();
    let home_dir = &std::env::var("HOME")
                            .map(std::path::PathBuf::from)
                            .unwrap_or(current_dir.clone());

    loop {
        let mut current_dir = std::env::current_dir().unwrap();
        print!("{}> ", current_dir.display());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let commands = input.trim().split("|").map(str::trim);

        let mut prev_output: Option<std::process::Child> = None;

        for command in commands {
            let parts: Vec<&str> = command.split_whitespace().collect();

            //TODO lets abstrct the command away
            //TODO add a command that lets you save a commadn block and reuse
            match parts[0] {
                "cd" => {
                   let path = match parts.get(1) {
                        Some(&path) if path == "-" => previous_dir.clone(),
                        Some(&path) if path == "~" => home_dir.clone(),
                        Some(&path) => std::path::Path::new(path).to_owned(),
                        None => home_dir.clone(),
                    };

                    match std::env::set_current_dir(&path) {
                        Ok(()) =>{previous_dir = current_dir; current_dir = path.to_owned()},
                        Err(e) => eprintln!("cd: {}", e),
                    }
                }
                _ => {
                    let mut child = Command::new(parts[0])
                        .args(&parts[1..])
                        .stdin(prev_output.take().map_or(Stdio::inherit(), |output: std::process::Child| {
                            Stdio::from(output.stdout.unwrap())
                        }))
                        .stdout(Stdio::piped())
                        .spawn()
                        .expect("failed to execute process");

                    if let Some(mut prev) = prev_output {
                        prev.wait().expect("failed to execute process");
                    }

                    prev_output = Some(child);
                }
            }
        }

        if let Some(output) = prev_output {
            let output = output.wait_with_output().expect("failed to execute process");
            io::stdout().write_all(&output.stdout).unwrap();
        }
    }
}

