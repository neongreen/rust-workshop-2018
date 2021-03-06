use command::Command;
use nom::{rest_s, space1};

/// Parse a command.
///
/// ```
/// # use redisish::parse::*;
/// # use redisish::command::*;
///
/// assert_eq!(parse("PUBLISH numbers one".into()),
///     Ok(Command::Publish{list: "numbers".into(), value: "one".into()}));
///
/// assert_eq!(parse("RETRIEVE numbers".into()),
///     Ok(Command::Retrieve{list: "numbers".into()}));
/// ```
pub fn parse(cmd: String) -> Result<Command, String> {
    match commandP(cmd.trim()) {
        Ok((_rest, val)) => Ok(val),
        Err(err) => Err(format!("{}", err)),
    }
}

named!(publishP<&str, Command>,
    do_parse!(
               tag!("PUBLISH") >>
               space1 >>
        list:  take_until!(" ") >>
               space1 >>
        value: rest_s >>
        (Command::Publish { list: list.into(), value: value.into() })
    )
);

named!(retrieveP<&str, Command>,
    do_parse!(
               tag!("RETRIEVE") >>
               space1 >>
        list:  rest_s >>
        (Command::Retrieve { list: list.into() })
    )
);

named!(commandP<&str, Command>,
    alt!(publishP | retrieveP)
);
