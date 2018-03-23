use std::net::TcpStream;
use std::str;
use std::io::{self, BufRead, BufReader, Write};
use std::time::Duration;
use std::net::SocketAddr;

fn main() {
    let remote: SocketAddr = "127.0.0.1:8888".parse().unwrap();
    let mut stream = TcpStream::connect_timeout(&remote, Duration::from_secs(1)).expect("Could not connect to server");
    stream.set_read_timeout(Some(Duration::from_secs(3))).expect("Could not set a read timeout");
    loop {
        let mut input = String::new();
        let mut buffer: Vec<u8> = Vec::new();
        io::stdin().read_line(&mut input).expect("Failed to read from stdin");
        stream.write(input.as_bytes()).expect("Failed to write to server");

        let mut reader = BufReader::new(&stream);

        reader.read_until(b'\n', &mut buffer).expect("Could not read into buffer");
        print!("{}", str::from_utf8(&buffer).expect("Could not write buffer as string"));
    }
}
