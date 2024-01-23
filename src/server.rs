use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};


pub struct Server {
    listener: TcpListener,
    link_map: HashMap<String, String>,
}

enum ResponseType {
    Ok,
    BadRequest,
    NotFound,
}


const HTTP_VERSION: &str = "HTTP/1.1";


impl Server {
    pub fn init(addr: &str, link_map: HashMap<String, String>) -> io::Result<Self> {
        Ok(Server {
            listener: TcpListener::bind(addr)?,
            link_map,
        })
    }

    pub fn run(&self) -> io::Result<()> {
        for stream in self.listener.incoming() {
            self.handle_connection(stream?)?;
        }
        Ok(())
    }

    fn handle_connection(&self, stream: TcpStream) -> io::Result<()> {
        let mut lines = BufReader::new(&stream).lines();
        let request_line = match lines.next() {
            Some(line) => line?,
            None => return Ok(()),
        };
        let _headers: Vec<_> = lines
            .flatten()
            .take_while(|line| !line.is_empty())
            .collect();

        let request_tokens: Vec<_> = request_line.split(" ").collect();

        if request_tokens.len() != 3 {
            Self::send_response(stream, ResponseType::BadRequest, None)
        } else if request_tokens[0] != "GET" {
            Self::send_response(stream, ResponseType::NotFound, None)
        } else {
            let path = request_tokens[1];
            let token = &path[1..];

            if let Some(link) = self.link_map.get(token) {
                println!("Token requested: {token}");
                let content = format!("<meta http-equiv=\"refresh\" content=\"0; url={link}\" />");
                Self::send_response(stream, ResponseType::Ok, Some(&content))
            } else {
                Self::send_response(stream, ResponseType::NotFound, None)
            }
        }
    }

    fn send_response(mut stream: TcpStream, response_type: ResponseType, content: Option<&str>) -> io::Result<()> {
        use ResponseType::*;

        let code_and_reason = match response_type {
            Ok => "200 OK",
            BadRequest => "400 BAD REQUEST",
            NotFound => "404 NOT FOUND",
        };

        let content = match content {
            Some(content) => content,
            None => code_and_reason,
        };
        let length = content.len();

        write!(stream, "{HTTP_VERSION} {code_and_reason}\r\n")?;
        write!(stream, "Content-Length: {length}\r\n\r\n")?;
        write!(stream, "{content}")?;

        stream.flush()
    }
}
