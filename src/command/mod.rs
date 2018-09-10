use std::process::Command;

pub fn execute(command: &String, args: &[String]) {
    Command::new(command)
        .args(args)
        .spawn()
        .expect("Command couldn't be executed!");
}

pub mod help;