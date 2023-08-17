
use core::str::FromStr;
#[derive(Debug, PartialEq)]
pub enum DataType {
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

impl FromStr for DataType {
    type Err = ();

    fn from_str(input: &str) -> Result<DataType, Self::Err> {
        match input {
            "+" => Ok(DataType::String),
            "-" => Ok(DataType::Error),
            ":" => Ok(DataType::Integer),
            "$" => Ok(DataType::BulkStrings),
            "*" => Ok(DataType::Array),
            "_" => Ok(DataType::Null),
            "#" => Ok(DataType::Boolean),
            "," => Ok(DataType::Double),
            "(" => Ok(DataType::BigNumber),
            "!" => Ok(DataType::BulkError),
            "=" => Ok(DataType::Verbatium),
            "%" => Ok(DataType::Map),
            "~" => Ok(DataType::Set),
            ">" => Ok(DataType::Push),

            _ => Err(()),
        }
    }
}

impl From<DataType> for String {
    fn from(value: DataType) -> Self {
        match value {
            DataType::String => String::from("+"),
            DataType::Error => String::from("-"),
            DataType::Integer => String::from(":"),
            DataType::BulkStrings => String::from("$"),
            DataType::Array => String::from("*"),
            DataType::Null => String::from("_"),
            DataType::Boolean => String::from("#"),
            DataType::Double => String::from(","),
            DataType::BigNumber => String::from("("),
            DataType::BulkError => String::from("!"),
            DataType::Verbatium => String::from("="),
            DataType::Map => String::from("%"),
            DataType::Set => String::from("~"),
            DataType::Push => String::from("~"),
        }
    }
}
const TERM_CHAR: &str = "\r\n";

pub fn form_response(data_type: DataType, response: &str) -> String {
    String::from(data_type) + response + TERM_CHAR
}
