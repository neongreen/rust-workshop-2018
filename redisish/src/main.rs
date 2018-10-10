#[macro_use]
extern crate nom;

use nom::space1;

use std::collections::HashMap;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
use std::net::{TcpListener, TcpStream};

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7000")?;
    let mut store = HashMap::new();

    // Accept connections and process them serially
    for stream in listener.incoming() {
        let mut stream = stream?;
        let cmd = {
            let mut reader = BufReader::new(&stream);
            let mut line = String::new();
            let _ = reader.read_line(&mut line)?;
            parse(line.as_str())?
        };
        // Execute a command and go home.
        //
        // TODO: handle several commands in the same connection.
        exec(cmd, &mut store, &mut stream);
    }
    Ok(())
}

#[derive(Debug, Clone)]
enum Command {
    /// Insert the specified value at the head of the list stored at key. If
    /// key does not exist, it is created as empty list before performing
    /// the push operation.
    ///
    /// Example: `PUBLISH numbers one`. Note the lack of quotes.
    Publish { list: String, value: String },

    /// Remove and return the first element of the list stored at key.
    ///
    /// Example: `RETRIEVE numbers`.
    Retrieve { list: String },
}

// Parsing

/// Parse a Redisish command. The command has to end with a newline.
///
/// TODO: would be nice if the command didn't have to with a newline.
fn parse(cmd: &str) -> io::Result<Command> {
    match commandP(cmd) {
        Ok(("", parsed_cmd)) => Ok(parsed_cmd),
        Ok((rest, _)) => Err(Error::new(
            ErrorKind::Other,
            format!("Unparsed input: {}", rest),
        )),
        Err(err) => Err(Error::new(
            ErrorKind::Other,
            format!("Parse error: {}", err),
        )),
    }
}

named!(publishP<&str, Command>,
  do_parse!(
           tag!("PUBLISH") >> space1 >>
    list:  take_until!(" ") >>
           space1 >>
    value: take_until_and_consume!("\n") >>
    (Command::Publish { list: list.into(), value: value.into() })
  )
);

named!(retrieveP<&str, Command>,
  do_parse!(
           tag!("RETRIEVE") >> space1 >>
    list:  take_until_and_consume!("\n") >>
    (Command::Retrieve { list: list.into() })
  )
);

named!(commandP<&str, Command>,
  alt!(publishP | retrieveP)
);

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
