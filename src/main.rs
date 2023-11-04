use std::fs;
pub const PASSWORDS_FILE: &str = "passwords.txt";

fn main() {
    // get args
    let args: Vec<String> = std::env::args().collect();

    let password_key = &args[1];
    assert!(password_key.len() > 0);

    let home_dir = std::env::var("HOME").unwrap();
    let password_manager_dir = home_dir + "/.password_manager";
    let passwords_file = password_manager_dir + "/" + PASSWORDS_FILE;

    if !std::path::Path::new(&passwords_file).exists() {
        create_password_file();
    }

    let passwords = fs::read_to_string(passwords_file).expect("Unable to read file");
}

fn create_password_file() {
    let home_dir = std::env::var("HOME").unwrap();
    let password_manager_dir = home_dir + "/.password_manager";
    fs::create_dir_all(&password_manager_dir).expect("Unable to create directory");
    fs::File::create(password_manager_dir + "/" + PASSWORDS_FILE).expect("Unable to create file");
}
