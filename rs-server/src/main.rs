mod cept;
mod cept_page;
mod dispatch;
mod editor;
mod historic;
mod image;
mod login;
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

use serde_json::{Result, Value};
use std::{fs::File, process::exit};
use scraper::{Html, Selector};

use std::net::TcpListener;
use std::thread;
use session::*;



fn main() {
    // let f = File::open("/Users/mist/Desktop/bee.json").unwrap();
    // let json: Value = serde_json::from_reader(f).unwrap();
    // let parse = json.get("parse").unwrap();
    // let pageid = parse.get("pageid").unwrap().to_string();
    // let text = parse.get("text").unwrap().get("*").unwrap().to_string();
    // let title = parse.get("title").unwrap().to_string();
    // println!("{}", title);
    // println!("{}", pageid);
    // // println!("{}", text);
    // let document = Html::parse_document(&text);
    // // let mut c = document.root_element().children().nth(1).unwrap();
    // // for c in c.children() {
    // //     println!("1 {:?}", c.value());
    // // }

    // let c = document.select(&Selector::parse("body").unwrap()).nth(0).unwrap();
    // let c = c.select(&Selector::parse("div").unwrap()).nth(0).unwrap();
    // println!("{}", c.value().name());

    // for c in c.children() {
    //     if c.value().is_element() {
    //         println!("1 {:?}", c.value().as_element().unwrap().name());
    //     } else {
    //         println!("2 {:?}", c.value());
    //     }
    // }


    // // let mut c = c.nth(0).unwrap().children();
    // // println!("2 {:?}", c);
    // // let c = c.nth(0).unwrap().children();
    // // println!("3 {:?}", c);
    // // for (i, child) in c.enumerate() {
    // //     println!("    {} {:?}", i, child.value());
    // // }
    // exit(0);


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
