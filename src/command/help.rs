pub fn print() {
    println!("Proton Launcher v{}", env!("CARGO_PKG_VERSION"));
    println!("=============================");
    println!("Usage:");
    println!("proton-launcher [path to windows application]");
    println!("OR");
    println!("proton-launcher winetricks-install [winetricks package name]");
    println!("");
    println!("IMPORTANT:");
    println!("If you want to install a package/dependency/whatever over winetricks you have to move at least into the proton prefix of your application!");
    println!("Example: You move into ~/WinApps/Battlenet");
    println!("");
    println!("Bugs or feature ideas? Create an issue on GitHub!");
}