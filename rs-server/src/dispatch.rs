use std::collections::HashMap;
use crate::session::*;
use super::page::*;
use super::user::*;


pub trait PageSession<'a> {
    fn create(&self) -> Option<Page>;
    fn validate(&self, name: &str, input_data: &HashMap<String, String>) -> ValidateResult;
    fn send(&self, input_data: &HashMap<String, String>) -> UserRequest;
}

struct PageSessionNewFn(fn(PageId, User) -> Box<dyn PageSession<'static>>);

// mask:
//   * If a mask does not end in '*' or '-', the page number must match exactly.
//   * If a mask ends in '*', it only has to be a prefix of the page number.
//   * If a mask ends in '-', it only has to be a prefix of the page number. The prefix of the
//     page number will be stripped when passed into the function.
// bool:
//   * Only use 'true' for BTX-internal pages that need to access the
//     user's info and statistics!
// N.B.: The table must be in the right order: longer prefixes must come first!
const DISPATCH_TABLE: &[(&[u8], bool, PageSessionNewFn)] = &[
    (b"00000*", true, PageSessionNewFn(super::login::new)),
    (b"9",      true, PageSessionNewFn(super::login::new)),
    (b"8*",     true, PageSessionNewFn(super::ui_messaging::new)),
    (b"77",     false, PageSessionNewFn(super::ui_user::new)),
    (b"7-",     false, PageSessionNewFn(super::historic::new)),
    (b"666",    false, PageSessionNewFn(super::image::new)),
    (b"*",      false, PageSessionNewFn(super::staticp::new)),
];

pub fn dispatch_pageid<'a>(pageid: &PageId, user: &User, anonymous_user: &User) -> Box<dyn PageSession<'static>> {
    for (mask, private_data, new_fn) in DISPATCH_TABLE {
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
            return new_fn.0(pageid, user.clone());
        }
    }
    unreachable!();
}
