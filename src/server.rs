use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};

use crate::store::Store;


pub struct Server {
    listener: TcpListener,
    store: Store
}

enum ResponseType {
    Ok,
    TemporaryRedirect,
    PermanentRedirect,
    BadRequest,
    NotFound,
}


const HTTP_VERSION: &str = "HTTP/1.1";
const LET_CLIENTS_CACHE: bool = true;
const NOT_FOUND_PAGE: &str = include_str!("404.html");
const REDIRECTION_PAGE: &str = include_str!("redirect.html");
const INDEX_PAGE: &str = include_str!("index.html");
const STYLE_SHEET: &str = include_str!("style.css");


impl Server {
    pub fn init(addr: &str, store: Store) -> io::Result<Self> {
        Ok(Server {
            listener: TcpListener::bind(addr)?,
            store,
        })
    }

    pub fn run(&mut self) {
        for stream in self.listener.incoming() {
            if let Ok(res) = self.store.has_changed() {
                if res {
                    let status = self.store.refresh();
                    if status.is_ok() {
                        let nlinks = self.store.len();
                        println!("Reloading store ({nlinks} links)");
                    }
                }
            }

            if let Ok(stream) = stream {
                let _ = self.handle_connection(stream);
            }
        }
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

        let request_tokens: Vec<_> = request_line.split(' ').collect();


        if request_tokens.len() != 3 {
            Self::send_response(stream, ResponseType::BadRequest, HashMap::new(), None)
        } else if request_tokens[0] != "GET" {
            Self::send_response(stream, ResponseType::NotFound, HashMap::new(), None)
        } else {
            let path = request_tokens[1];
            let token = &path[1..];

            if let Some(link) = self.store.get(token) {
                println!("Token requested: {token}");
                let content = str::replace(REDIRECTION_PAGE, "REDIRECTION_TOKEN", token);
                let content = str::replace(&content, "REDIRECTION_LINK", link);

                let response_type = if LET_CLIENTS_CACHE {
                    ResponseType::PermanentRedirect
                } else {
                    ResponseType::TemporaryRedirect
                };
                let headers = HashMap::from([("Location", link)]);
                Self::send_response(stream, response_type, headers, Some(&content))
            } else {
                match path {
                    "/" | "/index.html" =>
                        Self::send_response(stream, ResponseType::Ok, HashMap::new(), Some(INDEX_PAGE)),
                    "/style.css" =>
                        Self::send_response(stream, ResponseType::Ok, HashMap::new(), Some(STYLE_SHEET)),
                    _ => {
                        let content = str::replace(NOT_FOUND_PAGE, "REDIRECTION_TOKEN", token);
                        Self::send_response(stream, ResponseType::NotFound, HashMap::new(), Some(&content))
                    },
                }
            }
        }
    }

    fn send_response(mut stream: TcpStream, response_type: ResponseType,
                        headers: HashMap<&str, &str>, content: Option<&str>) -> io::Result<()> {
        use ResponseType::*;

        let code_and_reason = match response_type {
            Ok => "200 OK",
            TemporaryRedirect => "307 TEMPORARY REDIRECT",
            PermanentRedirect => "307 PERMANENT REDIRECT",
            BadRequest => "400 BAD REQUEST",
            NotFound => "404 NOT FOUND",
        };

        let content = match content {
            Some(content) => content,
            None => code_and_reason,
        };
        let length = content.len();

        // Status line
        write!(stream, "{HTTP_VERSION} {code_and_reason}\r\n")?;

        // Headers
        for (key, value) in &headers {
            write!(stream, "{key}: {value}\r\n")?;
        }
        write!(stream, "Content-Length: {length}\r\n\r\n")?;

        // Content
        write!(stream, "{content}")?;

        stream.flush()
    }
}
