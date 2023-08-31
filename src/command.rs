use std::collections::HashMap;
use std::str::FromStr;

use crate::resp::Message;

pub fn process_request(
    request_message: &str,
    redis_global_storage: &mut tokio::sync::MutexGuard<'_, HashMap<String, String>>,
) -> Message {
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
                                "ECHO" => {
                                    let args = command_args[1..]
                                        .iter()
                                        .map(|cmd| {
                                            return match cmd {
                                                Message::BulkString(value)
                                                | Message::SimpleString(value) => Some(value),
                                                _ => None,
                                            };
                                        })
                                        .collect::<Vec<Option<&String>>>();
                                    if args.contains(&None) {
                                        return Message::SimpleError(String::from(
                                            "Wrong format args for ECHO command",
                                        ));
                                    } else {
                                        return Message::SimpleString(
                                            args.iter()
                                                .map(|arg_option| arg_option.unwrap().as_str())
                                                .collect::<Vec<&str>>()
                                                .concat(),
                                        );
                                    }
                                }
                                "PING" => Message::SimpleString(String::from("PONG")),
                                "SET" => {
                                    let key_str = match command_args.get(1).unwrap() {
                                        Message::SimpleString(_string) => _string,
                                        Message::BulkString(_string) => _string,
                                        _ => {
                                            return Message::SimpleError(String::from(
                                                "BIG ERROR IN KEY",
                                            ));
                                        }
                                    };
                                    let value_str = match command_args.get(2).unwrap() {
                                        Message::SimpleString(_string) => _string,
                                        Message::BulkString(_string) => _string,
                                        _ => {
                                            return Message::SimpleError(String::from(
                                                "BIG ERROR IN VALUE",
                                            ));
                                        }
                                    };

                                    redis_global_storage
                                        .insert(key_str.to_string(), value_str.to_string());
                                    println!("{:?}", redis_global_storage);
                                    Message::SimpleString(String::from("OK"))
                                }
                                "GET" => {
                                    let key_str = match command_args.get(1).unwrap() {
                                        Message::SimpleString(_string) => _string,
                                        Message::BulkString(_string) => _string,
                                        _ => {
                                            return Message::SimpleError(String::from(
                                                "BIG ERROR IN KEY",
                                            ));
                                        }
                                    };
                                    println!("{:?}", redis_global_storage);
                                    println!("{}", key_str);

                                    let value = redis_global_storage.get(key_str).unwrap();

                                    Message::SimpleString(String::from(value.to_string()))
                                }

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
