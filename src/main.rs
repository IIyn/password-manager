mod password_objects;
mod save;
use std::io;
use {password_objects::password, password_objects::passwords};
use {save::file::read_password_file, save::file::write_password_file};

fn main() {
    // get args
    let args: Vec<String> = std::env::args().collect();
    assert!(args.len() > 1);

    process_args(args);
}

fn process_args(args: Vec<String>) {
    let mut test_passwords = read_password_file();

    if args[1] == "--show" || args[1] == "-s" {
        println!(
            "List of passwords saved :\n{}",
            test_passwords.get_passwords()
        );
    } else if args[1] == "--add-password" || args[1] == "-a" {
        println!("Enter the name of the password : ");
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");
        println!("Enter the password : ");
        let password = rpassword::read_password().unwrap();
        let password_to_insert =
            password::Password::new(name.trim().to_string(), password.trim().to_string());
        test_passwords.add(password_to_insert);
        write_password_file(test_passwords);
    } else if args[1] == "--generate-password" || args[1] == "-g" {
        println!("Enter the name of the password : ");
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");
        let password_to_insert =
            password::Password::new(name.trim().to_string(), password::generate_password());
        println!("Password generated successfully !");
        test_passwords.add(password_to_insert);
        write_password_file(test_passwords);
    } else if args[1] == "--get-password" && !args[2].is_empty() {
        let password_to_get = test_passwords.get_password(&args[2].to_string());
        if password_to_get.is_some() {
            println!("Password : {}", password_to_get.unwrap().get_value());
        } else {
            println!("Password not found");
        }
    } else {
        print!("Unknown command : ");
        for i in 1..args.len() {
            print!("{} ", args[i]);
        }
        println!();
    }
}
