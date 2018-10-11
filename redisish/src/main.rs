#[macro_use]
extern crate nom;

mod command;
mod execute;
mod parse;

use execute::*;
use parse::*;

use std::io;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7000")?;
    let mut store = Store::new();

    // Accept connections and process them serially
    for stream in listener.incoming() {
        let mut stream = stream?;
        let maybe_cmd = {
            let mut reader = BufReader::new(&stream);
            let mut line = String::new();
            let _ = reader.read_line(&mut line)?;
            parse(line)
        };
        match maybe_cmd {
            Ok(cmd) => match exec(cmd, &mut store) {
                Ok(res) => {
                    stream.write(res.as_bytes()).unwrap();
                }
                Err(e) => {
                    stream
                        .write(format!("Execution error: {}", e).as_bytes())
                        .unwrap();
                }
            },
            Err(e) => {
                stream
                    .write(format!("Parsing error: {}", e).as_bytes())
                    .unwrap();
            }
        }
    }
    Ok(())
}
