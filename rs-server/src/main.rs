mod cept;
mod editor;
mod historic;
mod login;
mod pages;
mod session;
mod stat;

use std::net::TcpListener;
use std::thread;
use session::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:20000").unwrap();
    println!("Neu-Ulm running.");
    for stream in listener.incoming() {
        thread::spawn(|| {
            let mut stream = stream.unwrap();
            let mut session = Session::new();
            session.run(&mut stream);
        });
    }

}
