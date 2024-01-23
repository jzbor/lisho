use std::{io, collections::HashMap};
use std::fs;
use std::env;
use std::process::exit;
mod server;


fn main() -> io::Result<()> {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 || args.len() > 3{
        print_usage();
        exit(1);
    }

    let addr = if args.len() == 3 {
        &args[2]
    } else {
        "localhost:8080"
    };

    let link_map = read_mappings(&args[1])?;
    let nlinks = link_map.len();
    let srv = server::Server::init(addr, link_map)?;

    println!("Listening on {addr} ({nlinks} links)");
    srv.run()
}

fn print_usage() {
    let bin_name = env::args().nth(0).unwrap();
    println!("Usage: {bin_name} <mapping_file> [address]");
}

fn read_mappings(file: &str) -> io::Result<HashMap<String, String>> {
    let file_contents = fs::read_to_string(file)?;
    let lines = file_contents.lines()
        .filter(|l| !l.starts_with("#"))
        .filter(|l| !l.is_empty());
    let mut map = HashMap::new();

    for line in lines {
        let columns: Vec<_> = line.split_whitespace().collect();

        if line.starts_with(" ") && columns.len() >= 1 {
            map.insert("".to_owned(), columns[0].to_owned());
        } else if columns.len() >= 2 {
            map.insert(columns[0].to_owned(), columns[1].to_owned());
        } else {
            eprintln!("Invalid mapping '{line}'");
        }
    }

    Ok(map)
}
