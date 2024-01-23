use std::io;
use std::env;
use std::process::exit;

mod server;
mod store;


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

    let store = store::Store::new(&args[1])?;
    let nlinks = store.len();
    let mut srv = server::Server::init(addr, store)?;

    println!("Listening on {addr} ({nlinks} links)");
    srv.run()
}

fn print_usage() {
    let bin_name = env::args().nth(0).unwrap();
    println!("Usage: {bin_name} <mapping_file> [address]");
}
