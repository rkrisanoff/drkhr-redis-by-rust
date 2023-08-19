use core::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Message {
    SimpleString(String),
    SimpleError(String),
    // Integer(usize),
    BulkString(String),
    Array(Vec<Message>),
    // Null,
    // Boolean,
    // Double,
    // BigNumber,
    // BulkError,
    // Verbatium,
    // Map,
    // Set,
    // Push,
}

fn parse_from_str(input: &str) -> Result<(Message, usize), String> {
    match input.as_bytes().first() {
        Some(&ch) => {
            match ch as char {
                '+' => {
                    let simple_string_lenght = match input.find(TERM_CHAR) {
                        Some(value) => value,
                        None => {
                            return Err(String::from(
                                "ERR simple string doesn't contain terminal sequence.",
                            ))
                        }
                    };
                    let input_content: String = match input.get(1..simple_string_lenght) {
                        Some(value) => value.to_string(),
                        None => {
                            return Err(String::from(
                                "ERR Too small commands. [Doesn't contain terminal symbols]",
                            ))
                        }
                    };
                    Ok((Message::SimpleString(input_content), simple_string_lenght))
                }
                '-' => {
                    let simple_string_lenght = match input.find(TERM_CHAR) {
                        Some(value) => value,
                        None => {
                            return Err(String::from(
                                "ERR simple string doesn't contain terminal sequence.",
                            ))
                        }
                    };
                    let input_content: String = match input.get(1..simple_string_lenght) {
                        Some(value) => value.to_string(),
                        None => {
                            return Err(String::from(
                                "ERR Too small commands. [Doesn't contain terminal symbols]",
                            ))
                        }
                    };
                    Ok((Message::SimpleError(input_content), simple_string_lenght))
                }
                // ":" => Ok(DataType::Integer),
                '$' => {
                    let splitter_index = match input.find(TERM_CHAR){
                        Some(index) => index,
                        None => return Err(String::from(
                            "Wrong format the Bulk String\n
                            Message doesn't contain terminate sequence beetwin lenght of string and a string"
                        )),
                    };
                    let bulk_string_length = match input[1..splitter_index].parse::<usize>() {
                        Ok(index) => index,
                        Err(err) => {
                            return Err(String::from(format!(
                                "Wrong format the Bulk String\n
                            Length of bulk string has a wrong format:\n{}",
                                err
                            )))
                        }
                    };
                    let bulk_string_start = splitter_index + TERM_CHAR.len();
                    let bulk_string_value =
                        &input[bulk_string_start..bulk_string_start + bulk_string_length];
                    Ok((
                        Message::BulkString(String::from(bulk_string_value)),
                        bulk_string_start + bulk_string_value.len() + TERM_CHAR.len(),
                    ))
                }
                '*' => {
                    let splitter_index = match input.find(TERM_CHAR) {
                        Some(index) => index,
                        None => {
                            return Err(String::from(
                                "Wrong format the Array\n
                            Message doesn't contain terminate sequence after lenght of array",
                            ))
                        }
                    };
                    let array_length = match input[1..splitter_index].parse::<usize>() {
                        Ok(index) => index,
                        Err(err) => {
                            return Err(String::from(format!(
                                "Wrong format the Array\n
                                Length of array has a wrong format:\n{}",
                                err
                            )))
                        }
                    };
                    let mut items: Vec<Message> = Vec::with_capacity(array_length);
                    let mut left_index = splitter_index + TERM_CHAR.len();
                    for _ in 1..array_length + 1 {
                        match parse_from_str(&input[left_index..]) {
                            Ok((item, size)) => {
                                left_index += size;
                                items.push(item);
                            }
                            Err(err) => return Err(err),
                        }
                    }
                    Ok((Message::Array(items), left_index))
                }
                '_' => todo!(),
                '#' => todo!(),
                ',' => todo!(),
                '(' => todo!(),
                '!' => todo!(),
                '=' => todo!(),
                '%' => todo!(),
                '~' => todo!(),
                '>' => todo!(),

                // "#" => Ok(DataType::Boolean),
                // "," => Ok(DataType::Double),
                // "(" => Ok(DataType::BigNumber),
                // "!" => Ok(DataType::BulkError),
                // "=" => Ok(DataType::Verbatium),
                // "%" => Ok(DataType::Map),
                // "~" => Ok(DataType::Set),
                // ">" => Ok(DataType::Push),
                _ => Err(String::from("Wrong resp data type")),
            }
        }
        None => return Err(String::from("Request is empty")),
    }
}

impl FromStr for Message {
    type Err = String;

    fn from_str(input: &str) -> Result<Message, Self::Err> {
        match parse_from_str(input) {
            Ok((message, _length)) => Ok(message),
            Err(error_message) => Err(error_message),
        }
    }
}

impl From<&Message> for String {
    fn from(value: &Message) -> Self {
        match value {
            Message::SimpleString(value) => String::from("+") + value + TERM_CHAR,
            Message::SimpleError(err) => String::from("-") + err + TERM_CHAR,
            // DataType::Integer => String::from(":"),
            // Message::BulkString => String::from("$"),
            Message::Array(arr) => format!(
                "*{}{}{}",
                arr.len(),
                TERM_CHAR,
                arr.iter()
                    .map(|item| String::from(item))
                    .into_iter()
                    .collect::<Vec<String>>()
                    .concat()
                    .as_str()
            ),
            Message::BulkString(value) => {
                String::from("$") + value.len().to_string().as_str() + TERM_CHAR + value + TERM_CHAR
            }
            // DataType::Null => String::from("_"),
            // DataType::Boolean => String::from("#"),
            // DataType::Double => String::from(","),
            // DataType::BigNumber => String::from("("),
            // DataType::BulkError => String::from("!"),
            // DataType::Verbatium => String::from("="),
            // DataType::Map => String::from("%"),
            // DataType::Set => String::from("~"),
            // DataType::Push => String::from("~"),
        }
    }
}

const TERM_CHAR: &str = "\r\n";

#[test]
fn name() {
    assert_eq!(
        String::from(&Message::SimpleString("fuck".to_string())),
        "+fuck\r\n"
    );
}

#[test]
fn test_bulk_string() {
    assert_eq!(
        Message::from_str("$5\r\nhello\r\n").unwrap(),
        Message::BulkString(String::from("hell0"))
    )
    // );
}

#[test]
fn test_simple_error() {
    assert_eq!(
        String::from(&Message::SimpleError("ERR fucking error".to_string())),
        "-ERR fucking error\r\n"
    );
}

#[test]
fn test_array() {
    assert_eq!(
        String::from(&Message::Array(vec![
            Message::SimpleString("fuck".to_string()),
            Message::Array(vec![Message::SimpleString("fuck".to_string())])
        ])),
        "*2\r\n+fuck\r\n*1\r\n+fuck\r\n"
    );
}

#[test]
fn test_bords() {
    for i in 0..2 {
        println!("{}", i);
    }
}
