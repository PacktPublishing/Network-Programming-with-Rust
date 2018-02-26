extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct RequestParser;

fn main() {
    let get = RequestParser::parse(Rule::ident_list, "GET /foobar/ HTTP/1.1\r\n")
        .unwrap_or_else(|e| panic!("{}", e));
    for pair in get {
        println!("Rule: {:?}", pair.as_rule());
        println!("Span: {:?}", pair.clone().into_span());
        println!("Text: {}", pair.clone().into_span().as_str());
    }

    let _ = RequestParser::parse(Rule::ident_list, "WRONG /foobar/ HTTP/1.1\r\n")
        .unwrap_or_else(|e| panic!("{}", e));
}
