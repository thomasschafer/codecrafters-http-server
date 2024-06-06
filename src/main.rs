use anyhow::{anyhow, bail, Result};
use regex::Regex;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str;

fn response(request: &str) -> Result<String> {
    let request_split = request.split_whitespace().collect::<Vec<_>>();

    let method = request_split[0];
    if method != "GET" {
        bail!("Unhandled request type {}", method);
    }

    let path = request_split[1];
    if let Some(captures) = Regex::new(r"/echo/(.*)").unwrap().captures(path) {
        let str_to_echo = captures
            .get(1)
            .map(|m| m.as_str())
            .ok_or(anyhow!("Found no match for /echo"))?;
        Ok(format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            str_to_echo.len(),
            str_to_echo
        ))
    } else if path == "/user-agent" {
        let lines = request.split("\r\n").collect::<Vec<_>>();
        let re = Regex::new(r"User-Agent: (.*)")?;
        let user_agent = lines
            .into_iter()
            .filter_map(|line| re.captures(line))
            .find(|_| true)
            .ok_or(anyhow!("Didn't find User-Agent header"))?
            .get(1)
            .ok_or(anyhow!("Couldn't parse User-Agent header"))?
            .as_str();
        Ok(format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            user_agent.len(),
            user_agent,
        ))
    } else if path == "/" {
        Ok("HTTP/1.1 200 OK\r\n\r\n".to_string())
    } else {
        Ok("HTTP/1.1 404 Not Found\r\n\r\n".to_string())
    }
}

fn handle_connection(stream: &mut TcpStream) -> Result<()> {
    let mut buf = [0; 128];
    let bytes = stream.read(&mut buf)?;
    if bytes > buf.len() {
        return Err(anyhow!("Couldn't handle request length"));
    }

    let request = str::from_utf8(&buf).unwrap();
    stream.write_all(response(request)?.as_bytes())?;
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
