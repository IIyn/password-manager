use crate::passwords::Passwords;
use std::fs;

const PASSWORDS_FILE: &str = "passwords.txt";

fn create_password_file() {
    let home_dir = std::env::var("HOME").unwrap();
    let password_manager_dir = home_dir + "/.password_manager";
    fs::create_dir_all(&password_manager_dir).expect("Unable to create directory");
    fs::File::create(password_manager_dir + "/" + PASSWORDS_FILE).expect("Unable to create file");
}

pub fn write_password_file(passwords: Passwords) {
    let home_dir = std::env::var("HOME").unwrap();
    let password_manager_dir = home_dir + "/.password_manager";
    let passwords_file = password_manager_dir + "/" + PASSWORDS_FILE;
    if !std::path::Path::new(&passwords_file).exists() {
        create_password_file();
    }
    let home_dir = std::env::var("HOME").unwrap();
    let password_manager_dir = home_dir + "/.password_manager";
    let passwords_file = password_manager_dir + "/" + PASSWORDS_FILE;
    let encrypted_passwords = encrypt_passwords(&passwords.to_string());
    fs::write(passwords_file, encrypted_passwords).expect("Unable to write file");
}

pub fn read_password_file() -> Passwords {
    let home_dir = std::env::var("HOME").unwrap();
    let password_manager_dir = home_dir + "/.password_manager";
    let passwords_file = password_manager_dir + "/" + PASSWORDS_FILE;
    if !std::path::Path::new(&passwords_file).exists() {
        return Passwords::new();
    }
    let encrypted_passwords = fs::read_to_string(passwords_file).expect("Unable to read file");
    if encrypted_passwords.is_empty() {
        return Passwords::new();
    }
    let mut passwords = Passwords::new();
    let decrypted_passwords = decrypt_passwords(&encrypted_passwords);
    passwords.from_string(&decrypted_passwords);
    passwords
}

fn encrypt_passwords(text: &str) -> String {
    let mut encrypted_text = String::new();
    for c in text.chars() {
        encrypted_text.push((c as u8 + 1) as char);
    }
    encrypted_text
}

fn decrypt_passwords(text: &str) -> String {
    let mut decrypted_text = String::new();
    for c in text.chars() {
        decrypted_text.push((c as u8 - 1) as char);
    }
    decrypted_text
}
