use rand::Rng;
pub struct Password {
    name: String,
    value: String,
}

impl Password {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }

    // pub fn set_name(&mut self, name: String) {
    //     self.name = name;
    // }

    // pub fn to_string(&self) -> String {
    //     format!("{}: {}", self.name, self.value)
    // }
}

// generate password with random characters like a-z, A-Z, 0-9, !@#$%^&*()_+{}|:"<>?~`
pub fn generate_password() -> String {
    let mut password = String::new();
    let mut rng = rand::thread_rng();
    let mut i = 0;
    while i < 20 {
        let random_number = rng.gen_range(0..94);
        let random_char = (random_number + 33) as u8 as char;
        password.push(random_char);
        i += 1;
    }
    password
}
