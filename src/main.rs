//use handlebars::TemplateError;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{self, Error, ErrorKind};
use std::process::{exit, Command};
use std::time::Duration;

enum CommandResult {
    Success,
    Failure(i32),
}

fn run_command_with_progress(command: &str, args: &[&str]) -> io::Result<CommandResult> {
    let mut cmd = Command::new(command);
    cmd.args(args);

    let pb = ProgressBar::new_spinner();
    let style_result = ProgressStyle::default_spinner()
        .tick_chars("/|\\- ")
        .template("{spinner:.green} {msg}");
    let style = match style_result {
        Ok(s) => s,
        Err(e) => return Err(Error::new(ErrorKind::Other, e.to_string())),
    };
    pb.set_style(style);
    pb.enable_steady_tick(Duration::from_millis(100));
    println!("Inside command with progress");
    pb.set_message(format!("Running command: {} {}", command, args.join(" ")));

    let output = cmd.output()?;
    if output.status.success() {
        pb.finish_with_message("Command completed successfully");
        Ok(CommandResult::Success)
    } else {
        let exit_code = output.status.code().unwrap_or(1);
        pb.finish_with_message(format!("Command failed with exit code: {}", exit_code));
        Ok(CommandResult::Failure(exit_code))
    }
}

fn run_command_with_output(command: &str, args: &[&str]) -> io::Result<String> {
    let output = Command::new(command).args(args).output()?;
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
        Ok(stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        Err(Error::new(ErrorKind::Other, stderr))
    }
}

fn run_unix_command_with_progress(command: &str, args: &[&str]) -> io::Result<()> {
    match run_command_with_progress(command, args)? {
        CommandResult::Success => Ok(()),
        CommandResult::Failure(exit_code) => exit(exit_code),
    }
}

fn run_unix_command_with_output(command: &str, args: &[&str]) -> io::Result<String> {
    run_command_with_output(command, args)
}

fn run_bash_script_with_progress(script: &str) -> io::Result<()> {
    match run_command_with_progress("bash", &["-c", script])? {
        CommandResult::Success => Ok(()),
        CommandResult::Failure(exit_code) => exit(exit_code),
    }
}

fn run_bash_script_with_output(script: &str) -> io::Result<String> {
    run_command_with_output("bash", &["-c", script])
}

use rustyline::error::ReadlineError; // Add this import
use rustyline::Editor;
use std::fs;
//use std::io;
use std::path::Path;

// ... (Same handle_* functions as before) ...

pub fn start_shell() {
    println!("Simple Rust Shell");
    println!("Type 'exit' to quit.");

    let mut rl = Editor::<rustyline::DefaultHelper, ()>::new();

    rl.set_word_completer(Some(Box::new(|line, _pos| {
        let mut completions = vec![];

        // Add available scripts from the 'scripts' folder
        let scripts_folder = Path::new("./scripts");
        if let Ok(entries) = fs::read_dir(scripts_folder) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(name) = entry.file_name().to_str() {
                        completions.push(name.to_string());
                    }
                }
            }
        }

        // Add available system commands by reading the PATH environment variable
        if let Some(path) = std::env::var_os("PATH") {
            let paths = std::env::split_paths(&path);
            for path in paths {
                if let Ok(entries) = fs::read_dir(path) {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            if let Some(name) = entry.file_name().to_str() {
                                completions.push(name.to_string());
                            }
                        }
                    }
                }
            }
        }

        completions
            .iter()
            .filter(|c| c.starts_with(line))
            .cloned()
            .collect()
    })));

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if line.trim() == "exit" {
                    break;
                } else {
                    handle_command(&line);
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("Exiting...");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

fn main() {
    match run_unix_command_with_progress("ls", &["-l"]) {
        Ok(_) => println!("Command completed"),
        Err(err) => eprintln!("Command failed: {}", err),
    }

    match run_bash_script_with_progress("echo Hello, world!") {
        Ok(_) => println!("Script executed"),
        Err(err) => eprintln!("Script failed: {}", err),
    }

    match run_unix_command_with_output("ls", &["-l"]) {
        Ok(output) => println!("Command output:\n{}", output),
        Err(err) => eprintln!("Command failed: {}", err),
    }

    match run_bash_script_with_output("echo Hello, world!") {
        Ok(output) => println!("Script output:\n{}", output),
        Err(err) => eprintln!("Script failed: {}", err),
    }

    //
    match run_unix_command_with_progress("sleep", &["10"]) {
        Ok(_) => println!("Command completed"),
        Err(err) => eprintln!("Command failed: {}", err),
    }
}
