use core::str::FromStr;
#[derive(Debug, PartialEq)]
pub enum Message {
    SimpleString(String),
    SimpleError(String),
    // Integer,
    // BulkStrings,
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

impl FromStr for Message {
    type Err = ();

    fn from_str(input: &str) -> Result<Message, Self::Err> {
        let input_chars: Vec<char> = input.chars().collect();
        let input_content: String = input[1..].to_string();
        match input_chars.first() {
            Some(ch) => {
                match ch {
                    '+' => Ok(Message::SimpleString(input_content)),
                    '-' => Ok(Message::SimpleError(input_content)),
                    // ":" => Ok(DataType::Integer),
                    // "$" => Ok(DataType::BulkStrings),
                    '*' => {
                        if let Some(array_size_index) = input_content.find(TERM_CHAR) {
                            if let Some(str_array_size) = input_content.get(0..array_size_index) {
                                if let Ok(_array_size) = str_array_size.parse::<u32>() {
                                    return Ok(Message::Array(
                                        input_content
                                            .get(array_size_index + 2..)
                                            .unwrap_or("")
                                            .split(TERM_CHAR)
                                            .into_iter()
                                            .map(|x| {
                                                Message::from_str(x)
                                                    .unwrap_or(Message::SimpleError(String::new()))
                                            })
                                            .collect(),
                                    ));
                                }
                            }
                        }

                        Err(())
                    }
                    // "_" => Ok(DataType::Null),
                    // "#" => Ok(DataType::Boolean),
                    // "," => Ok(DataType::Double),
                    // "(" => Ok(DataType::BigNumber),
                    // "!" => Ok(DataType::BulkError),
                    // "=" => Ok(DataType::Verbatium),
                    // "%" => Ok(DataType::Map),
                    // "~" => Ok(DataType::Set),
                    // ">" => Ok(DataType::Push),
                    _ => Err(()),
                }
            }
            None => return Err(()),
        }
    }
}

impl From<&Message> for String {
    fn from(value: &Message) -> Self {
        match value {
            Message::SimpleString(value) => String::from("+") + &value.clone() + TERM_CHAR,
            Message::SimpleError(err) => String::from("-") + &err.clone() + TERM_CHAR,
            // DataType::Integer => String::from(":"),
            // DataType::BulkStrings => String::from("$"),
            Message::Array(arr) => 
                    format!(
                        "*{}{}{}",
                        arr.len(),
                        TERM_CHAR,
                        arr
                    .iter()
                    .map(|item|String::from(item))
                    .into_iter()
                    .collect::<Vec<String>>()
                    .concat()
                    .as_str()
                )
            
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

// pub fn form_response(data_type: Message, response: &str) -> String {
//     String::from(data_type) + response + TERM_CHAR
// }

#[test]
fn name() {
    assert_eq!(
        String::from(
            &Message::SimpleString("fuck".to_string())
        ),
        "+fuck\r\n"
    );
}
#[test]

fn test_simple_error(){
    assert_eq!(
        String::from(
            &Message::SimpleError("ERR fucking error".to_string())
        ),
        "-ERR fucking error\r\n"
    );
}
#[test]

fn test_array(){
    assert_eq!(
        String::from(
            &Message::Array(
                vec![
                    Message::SimpleString("fuck".to_string()),
                    Message::Array(
                            vec![
                                Message::SimpleString("fuck".to_string())
                            ]
                        )
                ]
            )
        ),
        "*2\r\n+fuck\r\n*1\r\n+fuck\r\n"
    );
}