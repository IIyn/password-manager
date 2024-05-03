mod password_objects;
mod save;
use arboard::Clipboard;
use password_objects::{password, passwords};
use save::file::{
    hash_master_password, read_password_file, verify_master_password, write_password_file,
    MASTER_PASSWORD_FILE,
};
use std::{fs, io};

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

fn read_password() -> String {
    rpassword::read_password().unwrap()
}

fn ask_master_password() -> String {
    println!("Enter your master password : ");
    read_password()
}

fn verify_and_run<F: FnOnce(Option<&String>)>(callback: F, arg: Option<&String>) {
    if verify_master_password(ask_master_password()) {
        callback(arg);
    } else {
        println!("Wrong password")
    }
}

fn show_passwords() {
    let passwords_in_file = read_password_file();
    println!(
        "List of passwords saved :\n{}",
        passwords_in_file.get_passwords()
    );
}

fn add_password() {
    let mut passwords_in_file = read_password_file();
    println!("Enter the name of the password : ");
    let name = read_input();
    println!("Enter the password : ");
    let password = read_password();
    let password_to_insert =
        password::Password::new(name.trim().to_string(), password.trim().to_string());
    passwords_in_file.add(password_to_insert);
    write_password_file(passwords_in_file);
}

fn generate_password() {
    let mut passwords_in_file = read_password_file();
    println!("Enter the name of the password : ");
    let name = read_input();
    let password_to_insert =
        password::Password::new(name.trim().to_string(), password::generate_password());
    println!("Password generated successfully !");
    passwords_in_file.add(password_to_insert);
    write_password_file(passwords_in_file);
}

fn copy_password_clipboard(argument: &String) {
    let passwords_in_file = read_password_file();
    let password_to_get = passwords_in_file.get_password(&argument.to_string());
    if password_to_get.is_some() {
        let mut clipboard = Clipboard::new().unwrap();
        clipboard
            .set_text(password_to_get.unwrap().get_value())
            .unwrap();
        println!("Password copied to clipboard !");
    } else {
        println!("Password not found");
    }
}

fn copy_password(argument: &String) {
    let passwords_in_file: passwords::Passwords = read_password_file();
    let password_to_get: Option<&password::Password> =
        passwords_in_file.get_password(&argument.to_string());
    if password_to_get.is_some() {
        println!("Password : {}", password_to_get.unwrap().get_value());
    } else {
        println!("Password not found");
    }
}

fn edit_password() {
    let mut passwords_in_file: passwords::Passwords = read_password_file();
    println!("Enter the name of the password : ");
    let name: String = read_input();
    let password_to_edit: Option<&password::Password> =
        passwords_in_file.get_password(&name.trim().to_string());
    if password_to_edit.is_some() {
        println!("Enter the new password : ");
        let new_password: String = read_password();
        passwords_in_file.set_password(&name, new_password);
        write_password_file(passwords_in_file);
    } else {
        println!("Password not found");
    }
}

fn process_args(args: Vec<String>) {
    let password_path: String =
        std::env::var("HOME").unwrap() + "/.password_manager/" + MASTER_PASSWORD_FILE;
    if !std::path::Path::new(&password_path).exists() {
        println!("No password file found, write your master password : ");
        let master_password: String = read_password();
        hash_master_password(master_password);
    } else if fs::read_to_string(&password_path).unwrap().is_empty() {
        println!("No password file found, write your master password : ");
        let master_password: String = read_password();
        hash_master_password(master_password);
    } else {
        if args[1] == "--show" || args[1] == "-s" {
            verify_and_run(|_arg| show_passwords(), None);
        } else if args[1] == "--add-password" || args[1] == "-a" {
            verify_and_run(|_arg| add_password(), None);
        } else if args[1] == "--generate-password" || args[1] == "-g" {
            verify_and_run(|_arg| generate_password(), None);
        } else if args[1] == "--copy-password" || args[1] == "-c" {
            if args[2].is_empty() {
                println!("No password name given. Use --copy-password <password_name>");
                return;
            }

            if args[3].is_empty() {
                verify_and_run(
                    |_arg: Option<&String>| copy_password_clipboard(_arg.unwrap()),
                    Some(&args[2]),
                );
            } else if args[3] == "--no-clipboard" || args[3] == "-nc" {
                verify_and_run(
                    |_arg: Option<&String>| copy_password(_arg.unwrap()),
                    Some(&args[2]),
                );
            } else {
                println!("Unknown command : {}", args[3]);
            }
        } else if args[1] == "--edit-password" || args[1] == "-e" {
            verify_and_run(|_arg| edit_password(), None);
        } else if args[1] == "--help" || args[1] == "-h" {
            println!(
                "
            If using cargo to run the project, add -- before the arguments, like this :
            cargo run -- --show or cargo run -- --s
            --show, -s : Show all passwords saved
            --add-password, -a : Add a password
            --generate-password, -g : Generate a password
            --copy-password, -c : Copy a password to clipboard + --no-clipboard, -nc : to just show the password
            --help, -h : Show this help
            "
            );
        } else {
            print!("Unknown command : ");
            for i in 1..args.len() {
                print!("{} ", args[i]);
            }
            println!("Use --help to see all commands");
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert!(args.len() > 1);
    process_args(args);
}
