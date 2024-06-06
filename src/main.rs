use anyhow::{anyhow, bail, Result};
use regex::Regex;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str;

fn handle_connection(stream: &mut TcpStream) -> Result<()> {
    let mut buf = [0; 128];
    let bytes = stream.read(&mut buf)?;
    if bytes > buf.len() {
        return Err(anyhow!("Couldn't handle request length"));
    }

    let request_words = str::from_utf8(&buf)
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>();
    let method = request_words[0];
    if method != "GET" {
        bail!("Unhandled request type {}", method);
    }
    let path = request_words[1];

    let re = Regex::new(r"/echo/(.*)").unwrap();
    let parsed = re.captures(path);
    let response = match (parsed, path) {
        (Some(captures), _) => {
            let str_to_echo = captures.get(1).map(|m| m.as_str()).unwrap();
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                str_to_echo.len(),
                str_to_echo
            )
        }
        (None, "/") => "HTTP/1.1 200 OK\r\n\r\n".to_string(),
        (None, _) => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
    };
    stream.write_all(response.as_bytes())?;
    Ok(())
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for maybe_stream in listener.incoming() {
        match maybe_stream {
            Ok(mut stream) => handle_connection(&mut stream).unwrap(),
            Err(err) => println!("{}", err),
        }
    }
}
