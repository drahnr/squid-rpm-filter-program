use regex::Regex;

#[macro_use]
extern crate lazy_static;

use std::io::{BufReader,BufRead,stdin};
use std::iter::Iterator;

lazy_static!{
    static ref RE: Regex = 
        Regex::new(r#"([^/]+\.rpm)"#).unwrap()
    ;
}

fn main() {

    let mut reader = BufReader::new(stdin());


    let mut line = String::new();
    while reader.read_line(&mut line).is_ok() {
        let url : String = line.chars()
            .take_while(|c| { *c != ' ' } )
            .collect::<String>();

        RE.captures(url.as_str())
            .and_then(|captures| {
                captures.get(1)
            }).and_then(|cap|{
                println!("OK store-id={}", cap.as_str());
                Some(())
            })
            .unwrap_or_else(|| {
                println!("ERR");
            });
    }
}
