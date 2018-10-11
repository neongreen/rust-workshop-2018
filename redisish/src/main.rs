#[macro_use]
extern crate nom;

mod command;
mod parse;

use command::*;
use parse::*;

use std::collections::HashMap;
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7000")?;
    let mut store = HashMap::new();

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
            Ok(cmd) => exec(cmd, &mut store, &mut stream),
            Err(e) => {
                stream
                    .write(format!("Parsing error: {}", e).as_bytes())
                    .unwrap();
            }
        }
    }
    Ok(())
}

// Execution

type Store = HashMap<String, Vec<String>>;

/// Process a Redisish command.
fn exec(cmd: Command, store: &mut Store, stream: &mut TcpStream) {
    println!("Got a command: {:?}", cmd);
    let mut ret = |x: String| -> () {
        stream.write(x.as_bytes()).unwrap();
    };
    match cmd {
        Command::Publish { list, value } => {
            let x = store.entry(list).or_insert(vec![]);
            x.push(value);
            ret("OK\n".into());
        }
        Command::Retrieve { list } => match store.get_mut(&list) {
            None => ret(format!("ERR: Stack {} does not exist\n", list)),
            Some(v) => match v.pop() {
                Some(x) => ret(x),
                None => ret(format!("ERR: No elements in the stack {}\n", list)),
            },
        },
    }
}
