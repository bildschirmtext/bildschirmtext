mod cept;
mod cept_page;
mod database;
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

use session::*;
use std::io::{stdin, stdout};


fn main() {
    // println!("{}", make());
    // exit(0);

    eprintln!("Neu-Ulm running.");
    let mut session = Session::new();
    session.run(&mut stdin(), &mut stdout());
}

use html5ever::{interface::QualName, local_name, namespace_url, ns};

// use html5ever::*;
use kuchiki::{traits::*, Attribute, ExpandedName, NodeRef};

pub fn make() -> String {
    let text = "
            <p class='foo'>Hello, world!</p>
            <p class='foo'>I love HTML</p>
            <b class=\"nested\">
                <a href=\"blah\" style=\"display:none\">Clade of insects</div>
            </b>
            ";


    let document = kuchiki::parse_html().one(text);

    let paragraph = document.select("a").unwrap().collect::<Vec<_>>();

    let mut link_count = 10;

    for element in paragraph {
        eprintln!("{:?}", element);
        // let par = NodeRef::new_element(
        //     QualName::new(None, ns!(html), local_name!("p")),
        //     Some((
        //         ExpandedName::new("", "class"),
        //         Attribute {
        //                 prefix: None,
        //                 value: "newp".to_owned(),
        //         },
        //     )),
        // );
        // par.append(NodeRef::new_text("My new text"));

        let par = NodeRef::new_text(format!("[{}]", link_count));
        link_count += 1;

        // let par = NodeRef::new_comment("removed");
        element.as_node().insert_after(par);
        // element.as_node().detach();
    }

    document.to_string()
}
