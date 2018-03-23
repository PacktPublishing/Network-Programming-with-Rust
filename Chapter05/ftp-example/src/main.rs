extern crate ftp;

use std::str;
use std::io::Cursor;
use ftp::{FtpStream, FtpError};

fn run_ftp(addr: &str, user: &str, pass: &str) -> Result<(), FtpError> {
    let mut ftp_stream = FtpStream::connect((addr, 21))?;
    ftp_stream.login(user, pass)?;
    println!("current dir: {}", ftp_stream.pwd()?);

    ftp_stream.simple_retr("5MB.zip")?;
    println!("Downloaded file");

    let data = "A random string to write to a file";
    let mut reader = Cursor::new(data);
    ftp_stream.cwd("/upload")?;
    ftp_stream.put("my_file.txt", &mut reader)?;

    ftp_stream.quit()
}

fn main() {
    run_ftp("speedtest.tele2.net", "anonymous", "anonymous").unwrap();
}
