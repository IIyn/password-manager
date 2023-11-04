struct Password {
    value: String,
    name: String,
}

struct Passwords {
    passwords: Vec<Password>,
}

impl Passwords {
    fn new() -> Passwords {
        Passwords {
            passwords: Vec::new(),
        }
    }

    fn add(&mut self, password: Password) {
        self.passwords.push(password);
    }

    fn get(&self, name: &str) -> Option<&Password> {
        for password in &self.passwords {
            if password.name == name {
                return Some(password);
            }
        }
        None
    }

    fn to_string(&self) -> String {
        let mut result = String::new();
        for password in &self.passwords {
            result.push_str(&password.name);
            result.push('\n');
        }
        result
    }
}
