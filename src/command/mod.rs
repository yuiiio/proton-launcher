use std::process::Command;

pub fn execute(command: &String, args: &[String], dir: &String) {
    Command::new(command)
        .current_dir(dir)
        .args(args)
        .spawn()
        .expect("Command couldn't be executed!");
}

pub mod help;
