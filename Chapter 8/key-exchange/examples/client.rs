extern crate ring;
extern crate untrusted;

use std::net::TcpStream;
use std::io::{BufRead, BufReader, Write};

use ring::{agreement, rand};
use untrusted::Input;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8888").expect("Could not connect to server");

    let rng = rand::SystemRandom::new();
    let client_private_key = agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng).expect("Failed to generate key");
    let mut client_public_key = [0u8; agreement::PUBLIC_KEY_MAX_LEN];
    let client_public_key = &mut client_public_key[..client_private_key.public_key_len()];
    client_private_key.compute_public_key(client_public_key).expect("Failed to generate key");

    stream.write(client_public_key).expect("Failed to write to server");

    let mut buffer: Vec<u8> = Vec::new();
    let mut reader = BufReader::new(&stream);
    reader.read_until(b'\n', &mut buffer).expect("Could not read into buffer");
    let peer_public_key = Input::from(&buffer);

    println!("Received: {:?}", peer_public_key);

    let res = agreement::agree_ephemeral(client_private_key, &agreement::X25519, peer_public_key, ring::error::Unspecified,
                           |key_material| {
        let mut key = Vec::new();
        key.extend_from_slice(key_material);
        Ok(key)
    });

    println!("Generated: {:?}", res.unwrap());
}
