use std::collections::HashMap;
use crate::session::*;
use super::page::*;
use super::user::*;


pub trait PageSession<'a> {
    fn create(&self) -> Option<Page>;
    fn validate(&self, name: &str, input_data: &HashMap<String, String>) -> ValidateResult;
    fn send(&self, input_data: &HashMap<String, String>) -> UserRequest;
}

struct PageSessionNewFn(fn(&str, PageId, User) -> Box<dyn PageSession<'static>>);

// mask:
//   * If a mask does not end in '*' or '-', the page number must match exactly.
//   * If a mask ends in '*', it only has to be a prefix of the page number.
//   * If a mask ends in '-', it only has to be a prefix of the page number. The prefix of the
//     page number will be stripped when passed into the function.
// bool:
//   * Only use 'true' for BTX-internal pages that need to access the
//     user's info and statistics!
// N.B.: The table must be in the right order: longer prefixes must come first!
const DISPATCH_TABLE: &[(&[u8], bool, PageSessionNewFn, &str)] = &[
    (b"00000*", true,  PageSessionNewFn(super::login::new),        ""),
    (b"9",      true,  PageSessionNewFn(super::login::new),        ""),
    (b"8*",     true,  PageSessionNewFn(super::ui_messaging::new), ""),
    (b"77",     false, PageSessionNewFn(super::ui_user::new),      ""),
    (b"7-",     false, PageSessionNewFn(super::historic::new),     ""),
    (b"666",    false, PageSessionNewFn(super::image::new),        ""),

    // static pages
    (b"0-",     false, PageSessionNewFn(super::staticp::new),      "../data/0/"),

    // historic pages
    (b"1050-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/1050/"),
    (b"1188-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/1188/"),
    (b"1690-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/1690/"),
    (b"1692-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/1692/"),
    (b"20000-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/20000/"),
    (b"20095-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/20095/"),
    (b"20096-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/20096/"),
    (b"20511-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/20511/"),
    (b"21212-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/21212/"),
    (b"25800-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/25800/"),
    (b"30003-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/30003/"),
    (b"30711-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/30711/"),
    (b"33033-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/33033/"),
    (b"34034-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/34034/"),
    (b"34344-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/34344/"),
    (b"35853-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/35853/"),
    (b"40040-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/40040/"),
    (b"44479-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/44479/"),
    (b"50257-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/50257/"),
    (b"54004-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/54004/"),
    (b"57575-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/57575/"),
    (b"64064-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/64064/"),
    (b"65432-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/65432/"),
    (b"67007-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/67007/"),
    (b"201474-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/201474/"),
    (b"208585-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/208585/"),
    (b"208888-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/208888/"),
    (b"402060-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/402060/"),
    (b"8211882-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/8211882/"),
    (b"12001551-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/12001551/"),
    (b"50707545-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/50707545/"),
    (b"86553222-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/86553222/"),
    (b"505050035-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/505050035/"),
    (b"2000014317-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/2000014317/"),
    (b"15148830101-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/15148830101/"),
    (b"920492040092-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/920492040092/"),
    (b"1180040000004-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/1180040000004/"),
    (b"1200833401083-",  false, PageSessionNewFn(super::staticp::new),      "../data/hist/10/1200833401083/"),

    (b"*",  false, PageSessionNewFn(super::staticp::new),      ""), // will return None

];

pub fn dispatch_pageid<'a>(pageid: &PageId, user: &User, anonymous_user: &User) -> Box<dyn PageSession<'static>> {
    for (mask, private_data, new_fn, arg) in DISPATCH_TABLE {
        let matches;
        let reduce;
        let last = *mask.last().unwrap();
        if last == b'*' || last == b'-' {
            let mask = std::str::from_utf8(&mask[0..mask.len() - 1]).unwrap();
            matches = pageid.page.starts_with(mask);
            reduce = match last {
                b'*' => 0,
                _    => mask.len(),
            };
        } else {
            matches = pageid.page == std::str::from_utf8(mask).unwrap();
            reduce = 0;
        };
        if matches {
            let pageid = pageid.reduced_by(reduce).clone();
            let user = if *private_data { user } else { anonymous_user };
            return new_fn.0(arg, pageid, user.clone());
        }
    }
    unreachable!();
}
