use std::env;
use std::fs;
use std::process;

pub fn set_variable(name: String, content: String) {
    env::set_var(name, content);
}

pub fn get_variable(name: &'static str) -> String {
    return match env::var(name) {
        Ok(val) => {
            val
        },
        Err(err) => {
            println!("{}", err);
            return String::from("");
        }
    };
}

pub fn get_argument(index: usize) -> String {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() <= 1 || arguments.len() <= index {
        println!("Please provide the path to the application!");
        println!("For additional help: proton-launcher --help");
        process::exit(1);
    }

    return arguments[index].clone();
}

pub fn resolve_path(path: String) -> String {
    let converted_path = match fs::canonicalize(path) {
        Ok(val) => {
            val
        },
        Err(error) => {
            println!("{}", error);
            return "".to_string();
        }
    };

    return converted_path.as_path().to_str().unwrap().to_string();
}