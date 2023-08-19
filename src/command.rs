use std::str::FromStr;

use crate::resp::Message;

pub fn process_request(request_message: &str) -> Message {
    match Message::from_str(request_message) {
        Ok(message) => match message {
            Message::SimpleString(command) => match command.to_uppercase().as_str() {
                "PING" => Message::SimpleString(String::from("PONG")),
                _ => Message::SimpleError(String::from(format!("Unknown command `{}`", command))),
            },
            Message::SimpleError(_error) => Message::SimpleError(String::from(
                "Error as a command to druk-redis? It's too dangerous joke, guy...",
            )),
            Message::BulkString(_) => {
                Message::SimpleError(String::from("It's not an actual command!"))
            }
            Message::Array(command_args) => {
                if let Some(command) = command_args.first() {
                    return match command {
                        Message::SimpleString(command_repr) | Message::BulkString(command_repr) => {
                            match command_repr.to_uppercase().as_str() {
                                "ECHO" => Message::SimpleString(
                                    command_args[1..]
                                        .iter()
                                        .map(|cmd| String::from(cmd))
                                        .collect::<Vec<String>>()
                                        .concat(),
                                ),
                                _ => Message::SimpleError(String::from(format!(
                                    "Unknown command `{}`",
                                    String::from(command)
                                ))),
                            }
                        }
                        Message::SimpleError(_) => todo!(),
                        Message::Array(_) => todo!(),
                    };
                } else {
                    return Message::SimpleError(String::from(format!("Empty command!",)));
                };
            }
        },
        Err(err) => Message::SimpleError(String::from(err)),
    }
}
