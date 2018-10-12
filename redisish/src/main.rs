#[macro_use]
extern crate nom;

mod command;
mod execute;
mod parse;

use execute::*;
use parse::*;

use std::io;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7000")?;
    let store = Arc::new(Mutex::new(Store::new()));
    for stream in listener.incoming() {
        let mut stream = stream?;
        let store = store.clone();
        thread::spawn(move || {
            handle_client(&mut stream, store);
        });
    }
    Ok(())
}

fn handle_client(stream: &mut TcpStream, store: Arc<Mutex<Store>>) {
    let maybe_cmd = {
        let mut reader = BufReader::new(stream.by_ref());
        let mut line = String::new();
        let _ = reader.read_line(&mut line);
        parse(line)
    };
    let mut store = store.lock().unwrap();
    match maybe_cmd {
        Ok(cmd) => match exec(cmd, &mut store) {
            Ok(res) => {
                stream.write(format!("{}\n", res).as_bytes()).unwrap();
            }
            Err(e) => {
                stream
                    .write(format!("Execution error: {}\n", e).as_bytes())
                    .unwrap();
            }
        },
        Err(e) => {
            stream
                .write(format!("Parsing error: {}\n", e).as_bytes())
                .unwrap();
        }
    }
}
