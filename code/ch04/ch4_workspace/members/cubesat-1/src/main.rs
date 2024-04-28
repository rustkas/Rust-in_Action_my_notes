use cubesat_1::{CubeSat, GroundStation, Mailbox};

fn main() {
    let mut sat_a = CubeSat {
        id: 100,
        mailbox: Mailbox { messages: vec![] },
    };
    let base = GroundStation {};
    base.send(&mut sat_a, "hello".to_string());
    let msg = sat_a.recv();
    match msg {
        Some(msg) => println!("sat_a received: {msg}"),
        None => println!("No messages")
    }
    
}
