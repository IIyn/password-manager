use crate::passwords::Passwords;
use bcrypt::{hash, verify};
use rand::Rng;
use std::fs;

const PASSWORDS_FILE: &str = ".MOBJuelXwhUDRsP";
pub const MASTER_PASSWORD_FILE: &str = ".XwrxWOpRgHZywtx";

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
    let mut rng = rand::thread_rng();
    let mut i = 0;
    while i < 150 {
        let random_number = rng.gen_range(0..94);
        let random_char = (random_number + 33) as u8 as char;
        // add to beginning of string
        encrypted_text.insert(0, random_char);
        i += 1;
    }
    for c in text.chars() {
        encrypted_text.push((c as u8 + 132) as char);
    }
    i = 0;
    while i < 150 {
        let random_number = rng.gen_range(0..94);
        let random_char = (random_number + 33) as u8 as char;
        encrypted_text.push(random_char);
        i += 1;
    }
    encrypted_text
}

fn decrypt_passwords(text: &str) -> String {
    let mut decrypted_text = String::new();

    for c in text[150..text.len() - 150].chars() {
        decrypted_text.push((c as u8 - 132) as char);
    }
    decrypted_text
}

pub fn hash_master_password(password: String) {
    let hashed = hash(password, 4).expect("Unable to hash password");
    let master_password_file =
        std::env::var("HOME").unwrap() + "/.password_manager/" + MASTER_PASSWORD_FILE;
    if !std::path::Path::new(&master_password_file).exists() {
        fs::File::create(&master_password_file).expect("Unable to create file");
    }
    fs::write(master_password_file, hashed).expect("Unable to write file");
}

pub fn verify_master_password(password: String) -> bool {
    let master_password_file =
        std::env::var("HOME").unwrap() + "/.password_manager/" + MASTER_PASSWORD_FILE;
    let hashed = fs::read_to_string(master_password_file).expect("Unable to read file");
    let is_valid = verify(password, &hashed).expect("Unable to verify password");
    is_valid
}
