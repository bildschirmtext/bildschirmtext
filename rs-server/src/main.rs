mod cept;
mod editor;
mod historic;
mod pages;
mod stat;

use std::net::TcpListener;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:20000").unwrap();
    println!("Neu-Ulm running.");
    for stream in listener.incoming() {
        thread::spawn(|| {
            let mut stream = stream.unwrap();
            pages::interactive_mode(&mut stream);
        });
    }

}
