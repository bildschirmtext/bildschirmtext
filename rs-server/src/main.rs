mod cept;
mod cept_page;
mod dispatch;
mod editor;
mod historic;
mod image;
mod login;
mod mediawiki;
mod messaging;
mod page;
mod paths;
mod session;
mod staticp;
mod sysmsg;
mod user;
mod ui;
mod ui_messaging;
mod ui_user;

mod top;

use serde_json::{Result, Value};
use std::{fs::File, process::exit};
// use scraper::{Html, Selector};
use select::document::Document;
use select::predicate::{Class, Name, Predicate};

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
