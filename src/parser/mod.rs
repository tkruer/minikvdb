use crate::frame::Frame;

use bytes::Bytes;
use core::str;
use std::{fmt, vec};

#[derive(Debug)]
pub struct Parser {
    parts: vec::IntoIter<Frame>,
}

#[derive(Debug)]
pub enum ParserError {
    EndOfStream,
    Other(crate::Error),
}

impl Parser {
    pub fn new(frame: Frame) -> Result<Parser, ParserError> {
        let array = match frame {
            Frame::Array(array) => array,
            frame => return Err(format!("protocol error; expected array, got {:?}", frame).into()),
        };

        Ok(Parser {
            parts: array.into_iter(),
        })
    }

    fn next(&mut self) -> Result<Frame, ParserError> {
        self.parts.next().ok_or(ParserError::EndOfStream)
    }

    pub fn next_string(&mut self) -> Result<String, ParserError> {
        match self.next()? {
            Frame::Simple(s) => Ok(s),
            Frame::Bulk(data) => str::from_utf8(&data[..])
                .map(|s| s.to_string())
                .map_err(|_err| "Protocol error, invalid string".into()),
            frame => Err(format!(
                "Protocol error. Expected Frame or Bulk Frame, Recieved {:?}",
                frame
            )
            .into()),
        }
    }

    pub fn next_bytes(&mut self) -> Result<Bytes, ParserError> {
        match self.next()? {
            Frame::Simple(s) => Ok(Bytes::from(s.into_bytes())),
            Frame::Bulk(data) => Ok(data),
            frame => Err(format!(
                "protocol error; expected simple frame or bulk frame, got {:?}",
                frame
            )
            .into()),
        }
    }

    pub fn next_int(&mut self) -> Result<u64, ParserError> {
        use atoi::atoi;

        const MSG: &str = "protocol error; invalid number";

        match self.next()? {
            Frame::Integer(v) => Ok(v),
            Frame::Simple(data) => atoi::<u64>(data.as_bytes()).ok_or_else(|| MSG.into()),
            Frame::Bulk(data) => atoi::<u64>(&data).ok_or_else(|| MSG.into()),
            frame => Err(format!("protocol error; expected int frame but got {:?}", frame).into()),
        }
    }

    pub fn finish(&mut self) -> Result<(), ParserError> {
        if self.parts.next().is_none() {
            Ok(())
        } else {
            Err("protocol error; expected end of frame, but there was more".into())
        }
    }
}

impl From<String> for ParserError {
    fn from(src: String) -> ParserError {
        ParserError::Other(src.into())
    }
}

impl From<&str> for ParserError {
    fn from(src: &str) -> ParserError {
        src.to_string().into()
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::EndOfStream => "protocol error; unexpected end of stream".fmt(f),
            ParserError::Other(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for ParserError {}
