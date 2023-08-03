
# Rust Command Execution and Shell Interaction [![Rust](https://github.com/stormogulen/CommandRunner/actions/workflows/rust.yml/badge.svg)](https://github.com/stormogulen/CommandRunner/actions/workflows/rust.yml)

This Rust program showcases different ways of executing system commands, running shell scripts, and interacting with a simple shell interface. It utilizes external crates such as `indicatif`, `std::process`, and `rustyline` to provide various functionalities.

### Program Description

This program demonstrates the following features:

- Running system commands with progress indication using indicatif crate.

- Running Bash scripts with progress indication.

- Capturing and displaying the output of system commands and Bash scripts.

- Providing a simple interactive shell interface.

- Autocompletion of available scripts and system commands in the shell.

### Features

- Command Execution with Progress: The program uses the indicatif crate to display progress and status messages during command execution.

- Bash Script Execution: The program demonstrates how to run Bash scripts using the system command.

- Capturing Output: The program captures and displays the output (both stdout and stderr) of executed commands and scripts.

- Interactive Shell: The program includes a basic interactive shell interface that allows you to execute commands and scripts interactively.

- TODO: Autocompletion: The shell supports autocompletion of available scripts from a specified folder and system commands from the PATH.
