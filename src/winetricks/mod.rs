use std::env;

use super::command;
use super::environment;
use super::proton;

pub fn do_the_trick(package: String) {
    // Get the path of the actual Wine prefix
    let current_path = env::current_dir().unwrap();
    let assumed_prefix = proton::get_prefix_path(&current_path.to_str().unwrap().to_string());

    let prefix_path = assumed_prefix + "/pfx/drive_c";

    // Install the package
    environment::set_variable("WINEPREFIX".to_string(), prefix_path.clone());

    command::execute(&"winetricks".to_string(), &[
        package,
    ]);
}
