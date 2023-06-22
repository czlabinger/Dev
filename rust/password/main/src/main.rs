use password_core::PasswordEntry;
use password_core::PasswordStore;

fn main() {
    let mut store = PasswordStore::new();
    let e = PasswordEntry::new("Uname".to_string(), "Password".to_string(), "Service".to_string());
    store.add_entry(e);
}
