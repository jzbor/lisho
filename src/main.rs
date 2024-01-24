use std::env;
use std::process;
use std::process::exit;

mod server;
mod store;


fn main() {
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

    let store = match store::Store::new(&args[1]) {
        Ok(store) => store,
        Err(e) => { eprintln!("Unable to create store: {}", e); process::exit(1); },
    };
    let nlinks = store.len();

    let mut srv = match server::Server::init(addr, store) {
        Ok(store) => store,
        Err(e) => { eprintln!("Unable to start server: {}", e); process::exit(1); },
    };

    println!("Listening on {addr} ({nlinks} links)");
    srv.run();
}

fn print_usage() {
    let bin_name = env::args().next().unwrap();
    println!("Usage: {bin_name} <mapping_file> [address]");
}
