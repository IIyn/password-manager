use crate::password_objects::password::Password;

pub struct Passwords {
    passwords: Vec<Password>,
}

impl Passwords {
    pub fn new() -> Self {
        Self {
            passwords: Vec::new(),
        }
    }

    pub fn from_string(&mut self, string: &str) {
        for line in string.lines() {
            let mut split = line.split(' ');
            let name = split.next().unwrap();
            let value = split.next().unwrap();
            self.add(Password::new(name.to_owned(), value.to_owned()));
        }
    }

    pub fn add(&mut self, password: Password) {
        self.passwords.push(password);
    }

    pub fn get_password(&self, name: &str) -> Option<&Password> {
        self.passwords.iter().find(|p| p.get_name() == name)
    }

    // pub fn get_passwords(&self, name: &str) -> Option<&Password> {
    //     for password in &self.passwords {
    //         if password.get_name() == name {
    //             return Some(password);
    //         }
    //     }
    //     None
    // }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for password in &self.passwords {
            result.push_str(&password.get_name());
            result.push(' ');
            result.push_str(&password.get_value());
            result.push('\n');
        }
        result
    }

    pub fn get_passwords(&self) -> String {
        let mut result = String::new();
        for password in &self.passwords {
            result.push_str(&password.get_name());
            result.push('\n');
        }
        result
    }
}
