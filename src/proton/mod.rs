use std::fs;
use std::process;
use super::environment;

// Get the latest version of the installed Proton versions
pub fn get_latest_version() -> String {
    let files: fs::ReadDir = match fs::read_dir(environment::get_variable("HOME") + "/.local/share/Steam/steamapps/common") {
        Ok(val) => {
            val
        },
        Err(err) => {
            println!("{}", err);
            println!("Couldn't read the directory");
            process::exit(1);
        }
    };

    let mut proton_directories: Vec<String> = Vec::new();

    for file in files {
        match file.as_ref() {
            Ok(file_ref) => {
                if String::from(file_ref.file_name().to_string_lossy()).contains("Proton") {
                    proton_directories.push(String::from(file_ref.path().to_string_lossy()));
                }
            },
            Err(err) => {
                println!("{}", err);
            }
        }
    }

    if proton_directories.len() == 0 {
        println!("Please install Proton!");
        process::exit(1);
    }

    return proton_directories[proton_directories.len()-1].clone() + "/proton";
}

// Create a directory for the WINE/Proton prefix (internally called "compat data"-directory)
pub fn create_compat_data_dir(application_name: String) -> String {
    let directory_path = environment::get_variable("HOME") + "/WinApps/" + &*application_name;
    fs::create_dir_all(&directory_path)
        .expect("The directory for the prefix couldn't be created!");

    return directory_path;
}

// Assume the name of an application from its path
pub fn get_application_name(application_path: String) -> String {
    let splitted_application_path: Vec<&str> = application_path.split("/").collect();

    let application_name = splitted_application_path[splitted_application_path.len()-1].clone().replace(".exe", "");

    return String::from(application_name);
}

// Assume the prefix path from the path of an executable
pub fn get_prefix_path(application_path: &String) -> String {
    let splitted_application_path: Vec<&str> = application_path.split("/").collect();

    let mut counter: usize = 0;

    for path_part in splitted_application_path.clone() {
        counter += 1;
        if path_part == "WinApps" {
            break;
        }
    }

    if splitted_application_path.len() <= counter {
        println!("The application isn't located in a prefix!");
        return "".to_string();
    }

    // Prepare the prefix path
    let prefix_path = environment::get_variable("HOME") + "/WinApps/" + splitted_application_path[counter];

    println!("The application is located in a prefix: {}", &prefix_path);

    return prefix_path;
}

// Check if the file which should be executed is located in an existing WINE/Proton prefix
pub fn check_if_in_prefix(application_path: String) -> bool {
    // Prepare the path to the prefixes
    let prefix_directory = environment::get_variable("HOME") + "/WinApps/";

    return application_path.contains(&*prefix_directory);
}