extern crate ring;
extern crate untrusted;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};

use ring::{agreement, rand};
use untrusted::Input;
use ring::error::Unspecified;

fn handle_client(mut stream: TcpStream) -> Result<(), Unspecified> {
    let rng = rand::SystemRandom::new();

    let server_private_key = agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng)?;
    let mut server_public_key = [0u8; agreement::PUBLIC_KEY_MAX_LEN];
    let server_public_key = &mut server_public_key[..server_private_key.public_key_len()];
    server_private_key.compute_public_key(server_public_key)?;

    let mut peer_public_key_buf = [0u8; 32];
    stream.read(&mut peer_public_key_buf).expect("Failed to read");
    let peer_public_key = Input::from(&peer_public_key_buf);

    println!("Received: {:?}", peer_public_key);

    stream.write(&server_public_key).expect("Failed to send server public key");

    let res = agreement::agree_ephemeral(server_private_key, &agreement::X25519, peer_public_key, ring::error::Unspecified,
                           |key_material| {
        let mut key = Vec::new();
        key.extend_from_slice(key_material);
        Ok(key)
    });

    println!("Generated: {:?}", res.unwrap());

    Ok(())
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8888").expect("Could not bind");
    for stream in listener.incoming() {
        match stream {
            Err(e) => { eprintln!("failed: {}", e) }
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }
}
