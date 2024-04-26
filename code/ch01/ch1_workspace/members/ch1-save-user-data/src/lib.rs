use std::str;

#[derive(Debug)]
pub struct User {
    pub id: u8,
    pub secret: String,
}

pub fn store_secrets(user: &User, buffer: &mut [u8]) {
    let _secret = user.secret.clone();

    // assume we're writing to a database
    println!("{:?}: {}", user, str::from_utf8(&buffer).unwrap());
}
