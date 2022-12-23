use crate::utils::iterate_on_words;

use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{ Display, Debug, Formatter };
// different from base Result enum
use std::fmt::Result as FmtResult;
use std::str::{self, FromStr};
use std::str::Split;

pub struct Request<'buf> {
    path: &'buf str,
    query: Option<&'buf str>,
    method: Method 
}



impl<'buf> TryFrom::<&'buf [u8]> for Request<'buf>{
    // implement error type
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error>{
        let request = str::from_utf8(buf)
                               .or(Err(ParseError::InvalidEncoding))?;
        
        
        // extract first part of request and the rest of it
        let (method, request) = iterate_on_words(
            request,  ' '
        ).ok_or(ParseError::InvalidRequest)?;
        let (mut path, protocol) = iterate_on_words(
            request,  ' '
        ).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = iterate_on_words(
            request,  ' '
        ).ok_or(ParseError::InvalidRequest)?;

        let mut query_string = None;
        if let Some(i)= path.find("?"){
            query_string = Some(&path[i+1..]);
            path = &path[..i]
        }

        // Check method
        let method = method.parse::<Method>()?;

        let protocol = protocol.replace('\n', "").replace('\r', "");
        println!(r"{}",protocol);
        if protocol.trim() != "HTTP/1.1"{
            return Err(ParseError::InvalidProtocol)
        }

        //TODO: Implement header extraction from here
        //RENAME PROTOCOL TO REQUEST
        // let (_, request) = iterate_on_words(
        //     request,  '\r'
        // ).ok_or(ParseError::InvalidRequest)?;
        // Easy part of error handling
        // let (_, request) = iterate_on_words(
        //     request,  '\n'
        // ).ok_or(ParseError::InvalidRequest)?;

        Ok(Self { path: path, query: query_string, method: method })
    }
}



pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidMethod => "Invalid Method",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidRequest => "Invalid Request"
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}

pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
}

impl FromStr for Method {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "PATCH" => Ok(Self::PATCH),
            "DELETE" => Ok(Self::DELETE),
            "HEAD" => Ok(Self::HEAD),
            "CONNECT" => Ok(Self::CONNECT),
            "OPTIONS" => Ok(Self::OPTIONS),
            "TRACE" => Ok(Self::TRACE), 
            _ => Err(ParseError::InvalidMethod)     
        }
    }
}