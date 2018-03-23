extern crate pom;
use pom::DataInput;
use pom::parser::{sym, one_of, seq};
use pom::parser::*;

use std::str;

fn space() -> Parser<'static, u8, ()> {
    one_of(b" \t\r\n").repeat(0..).discard()
}

fn string() -> Parser<'static, u8, String> {
    one_of(b"abcdefghijklmnopqrstuvwxyz").repeat(0..).convert(String::from_utf8)
}

fn main() {
    let get = b"GET /home/ HTTP/1.1\r\n";
    let mut input = DataInput::new(get);
    let parser = (seq(b"GET") | seq(b"POST")) * space() * sym(b'/') * string() * sym(b'/') * space() * seq(b"HTTP/1.1");
    let output = parser.parse(&mut input);
    println!("{:?}", str::from_utf8(&output.unwrap()));
}
