use command::*;
use std::collections::HashMap;

/// In-memory server state.
pub struct Store(HashMap<String, Vec<String>>);

impl Store {
    /// Create an empty server state.
    pub fn new() -> Self {
        Store(HashMap::new())
    }
}

/// Process a command and say what should be returned to the user.
pub fn exec(cmd: Command, store: &mut Store) -> Result<String, String> {
    println!("Got a command: {:?}", cmd);
    let Store(s) = store;
    match cmd {
        Command::Publish { list, value } => {
            let x = s.entry(list).or_insert(vec![]);
            x.push(value);
            Ok("Added".into())
        }
        Command::Retrieve { list } => match s.get_mut(&list) {
            None => Err(format!("Stack {} does not exist", list)),
            Some(v) => match v.pop() {
                Some(x) => Ok(x),
                None => Err(format!("No elements in the stack {}", list)),
            },
        },
    }
}
