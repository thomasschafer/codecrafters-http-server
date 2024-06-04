use anyhow::{anyhow, Result};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str;

fn handle_connection(stream: &mut TcpStream) -> Result<()> {
    let mut buf = [0; 128];
    let bytes = stream.read(&mut buf)?;
    if bytes > buf.len() {
        return Err(anyhow!("Couldn't handle request length"));
    }

    let request_words = str::from_utf8(&buf).unwrap().split(' ').collect::<Vec<_>>();
    let path = request_words[1];

    let response = match path {
        "/" => "HTTP/1.1 200 OK\r\n\r\n",
        _ => "HTTP/1.1 404 Not Found\r\n\r\n",
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
