use ch1_save_user_data::{User, store_secrets};

fn main() {
    let buffer = &mut[0u8; 1024];
    let u1 = User {
        id: 1,
        secret: String::from("Pa55w0rd!"),
    };
    let u2 = User {
        id: 2,
        secret: String::from("correct horse battery staple"),
    };

    store_secrets(&u1, buffer);
    store_secrets(&u2, buffer);
}
