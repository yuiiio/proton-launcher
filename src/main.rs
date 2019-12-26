mod command;
mod environment;
mod proton;
mod winetricks;

use std::collections::HashMap;
use std::process;

fn main() {
    println!("ProtonLauncher v{}", env!("CARGO_PKG_VERSION"));

    let first_argument = environment::get_argument(1);
    if first_argument == "winetricks-install" {
        let package = environment::get_argument(2);
        winetricks::do_the_trick(package);
        process::exit(0);
    } else if first_argument == "--help" {
        command::help::print();
        process::exit(0);
    }

    let application_path = environment::resolve_path(first_argument);

    // Prepare the environment
    let mut env_vars: HashMap<&'static str, &'static str> = HashMap::new();
    env_vars.insert("PROTON_NO_ESYNC", "1");
    env_vars.insert("PROTON_DUMP_DEBUG_COMMANDS", "1");

    for (name, value) in env_vars {
        // Set all environment variables
        environment::set_variable(name.to_string(), value.to_string());
    }

    // Get the path to the latest proton version
    let latest_proton_version = proton::get_latest_version();
    println!("Found Proton: {}", latest_proton_version);

    // Set the environment variable
    if proton::check_if_in_prefix(application_path.clone()) {
        // Get the prefix path
        let prefix_path = proton::get_prefix_path(&application_path);

        environment::set_variable("STEAM_COMPAT_DATA_PATH".to_string(), prefix_path);
    } else {
        // Create a new directory for the compat data
        let application_name = proton::get_application_name(application_path.clone());
        let compat_directory = proton::create_compat_data_dir(application_name);

        environment::set_variable("STEAM_COMPAT_DATA_PATH".to_string(), compat_directory);
    }

    // Start the application
    command::execute(
        &latest_proton_version,
        &["run".to_string(), application_path.clone()],
    );
}
