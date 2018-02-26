mod ini;

use std::io::{Read, Write};

use error::*;

use serde_json::{from_reader, to_writer, to_writer_pretty};
use serde_json::Value;

// #[derive(Debug, Eq, PartialEq)]
// pub enum Value {
//     Bool(bool),
//     Number(String),
//     Text(String),
//     Array(Vec<Value>),
//     Object(BTreeMap<String, Value>),
// }

// type AdaptorError = &'static str;
// type AResult<T> = Result<T, ErrorKind>;

pub use self::ini::IniAdaptor;

pub trait Adaptor<'a> {
    fn serialize<W: Write>(&self, conf: Value, writer: W) -> Result<()>;
    fn deserialize<R: Read>(&self, reader: R) -> Result<Value>;
}

pub struct JsonAdaptor {
    pretty: bool,
}

impl JsonAdaptor {
    pub fn new(pretty: bool) -> JsonAdaptor {
        JsonAdaptor { pretty: pretty }
    }
}

impl<'a> Adaptor<'a> for JsonAdaptor {
    fn deserialize<R>(&self, reader: R) -> Result<Value>
    where
        R: Read,
    {
        from_reader(reader).chain_err(|| "unable to deserialize JSON")
    }

    fn serialize<W>(&self, value: Value, writer: W) -> Result<()>
    where
        W: Write,
    {
        let result = if self.pretty {
            to_writer_pretty(writer, &value)
        } else {
            to_writer(writer, &value)
        };

        result.chain_err(|| "unable to serialize JSON")
    }
}