// Uncomment this block to pass the first stage
use std::{
    io::{Read, Write},
    net::TcpListener,
    str::FromStr,
};

#[derive(Debug, PartialEq)]
enum RespDataType {
    String,
    Error,
    Integer,
    BulkStrings,
    Array,
    Null,
    Boolean,
    Double,
    BigNumber,
    BulkError,
    Verbatium,
    Map,
    Set,
    Push,
}

impl FromStr for RespDataType {
    type Err = ();

    fn from_str(input: &str) -> Result<RespDataType, Self::Err> {
        match input {
            "+" => Ok(RespDataType::String),
            "-" => Ok(RespDataType::Error),
            ":" => Ok(RespDataType::Integer),
            "$" => Ok(RespDataType::BulkStrings),
            "*" => Ok(RespDataType::Array),
            "_" => Ok(RespDataType::Null),
            "#" => Ok(RespDataType::Boolean),
            "," => Ok(RespDataType::Double),
            "(" => Ok(RespDataType::BigNumber),
            "!" => Ok(RespDataType::BulkError),
            "=" => Ok(RespDataType::Verbatium),
            "%" => Ok(RespDataType::Map),
            "~" => Ok(RespDataType::Set),
            ">" => Ok(RespDataType::Push),

            _ => Err(()),
        }
    }
}

impl From<RespDataType> for String {
    fn from(value: RespDataType) -> Self {
        match value {
            RespDataType::String => String::from("+"),
            RespDataType::Error => String::from("-"),
            RespDataType::Integer => String::from(":"),
            RespDataType::BulkStrings => String::from("$"),
            RespDataType::Array => String::from("*"),
            RespDataType::Null => String::from("_"),
            RespDataType::Boolean => String::from("#"),
            RespDataType::Double => String::from(","),
            RespDataType::BigNumber => String::from("("),
            RespDataType::BulkError => String::from("!"),
            RespDataType::Verbatium => String::from("="),
            RespDataType::Map => String::from("%"),
            RespDataType::Set => String::from("~"),
            RespDataType::Push => String::from("~"),
        }
    }
}
const TERM_CHAR: &str = "\r\n";

fn build_response<'a>(response: &'a str) -> String {
    String::from(RespDataType::String) + response + TERM_CHAR
}

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let mut input_buffer: [u8; 64] = [0; 64];
                let req_read_res = stream.read(&mut input_buffer);

                if let Ok(buffer_size) = req_read_res {
                    let request = String::from_utf8_lossy(&input_buffer[..buffer_size]);
                    println!("Input buffer size is {}", buffer_size);
                    println!("Input buffer: `{:?}`", request.as_ref().to_string());

                    let res_write_result = stream.write(build_response("PONG").as_bytes());

                    match res_write_result {
                        Ok(size) => {
                            println!("Sent {} bytes", size);
                        }
                        Err(err) => {
                            println!("error with response sending: {}", err);
                        }
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
