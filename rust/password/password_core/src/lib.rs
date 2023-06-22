use ring::digest;

pub struct PasswordEntry {
    username: String,
    password: String,
    service: String,
}

pub struct PasswordStore {
    passwords: Vec<PasswordEntry>,
}

impl PasswordEntry {
    pub fn new(uname: String, pass: String, ser: String) -> Self {

        let mut digest = digest::digest(&digest::SHA256, pass.as_bytes());
        let pass_hash = format!("{:?}", digest);

        digest = digest::digest(&digest::SHA256, uname.as_bytes());
        let uname_hash = format!("{:?}", digest);

        digest = digest::digest(&digest::SHA256, ser.as_bytes());
        let ser_hash = format!("{:?}", digest);

        Self{
            username: uname_hash,
            password: pass_hash,
            service: ser_hash,
        }
    }
}

impl PasswordStore {
    pub fn new() -> Self {
        PasswordStore {
            passwords: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, e: PasswordEntry) -> bool {
        let result = self.passwords.push(e);
        
        match result {
            Ok(()) => {
                true
            },
            Err(err) => {
                false
            },
        }
    }

    pub fn read(serv: String) -> PasswordEntry {
        println!("Read");
        PasswordEntry::new("a".to_string(), "a".to_string(), "a".to_string())
    }

    pub fn delete(serv: String) -> bool {
        println!("Delete");
        true
    }

    pub fn update(uname: String, pass: String, ser: String) -> bool {
        println!("Update");
        true
    }
}
