mod cept;
mod cept_page;
mod editor;
mod historic;
mod login;
mod message;
mod pages;
mod session;
mod stat;
mod user;

// use serde_json::{Result, Value};
// use std::fs::File;
// use scraper::Html;

use std::net::TcpListener;
use std::thread;
use session::*;



fn main() {
    // let f = File::open("/Users/mist/Desktop/bee.json").unwrap();
    // let json: Value = serde_json::from_reader(f).unwrap();
    // let parse = json.get("parse").unwrap();
    // let pageid = parse.get("pageid").unwrap().to_string();
    // let text = parse.get("text").unwrap().to_string();
    // let title = parse.get("title").unwrap().to_string();
    // println!("{}", title);
    // println!("{}", pageid);
    // // println!("{}", text);
    // let document = Html::parse_document(&text);
    // for child in document.root_element().children() {
    //     println!("{:?}", child.value());
    //     // println!("{:?}", child);
    // }


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
