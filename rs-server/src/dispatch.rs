use std::collections::HashMap;
use crate::session::*;
use super::page::*;
use super::user::*;


pub trait PageSession<'a> {
    fn create(&self) -> Option<Page>;
    fn validate(&self, name: &str, input_data: &HashMap<String, String>) -> ValidateResult;
    fn send(&self, input_data: &HashMap<String, String>) -> UserRequest;
}

struct PageSessionNewFn<'a>(fn(&'a PageId, Option<&'a User>, Option<&'a Stats>) -> Box<dyn PageSession<'a>>);

// Mask:
//   * If a mask does not end in '*' or '-', the page number must match exactly.
//   * If a mask ends in '*', it only has to be a prefix of the page number.
//   * If a mask ends in '-', it only has to be a prefix of the page number. The prefix of the
//     page number will be stripped when passed into the function.
// Function:
//   * Only use Anonymous::Yes for BTX-internal pages that need to access the
//     user's info and statistics!
// N.B.: The table must be in the right order: longer prefixes must come first!
const DISPATCH_TABLE: &[(&[u8], PageSessionNewFn)] = &[
    (b"00000*", PageSessionNewFn(super::login::new)),
    (b"9",      PageSessionNewFn(super::login::new)),
    (b"8*",     PageSessionNewFn(super::ui_messaging::new)),
    (b"77",     PageSessionNewFn(super::ui_user::new)),
    (b"7-",     PageSessionNewFn(super::historic::new)),
    (b"*",      PageSessionNewFn(super::staticp::new)),
];

pub fn dispatch_pageid<'a>(pageid: &'a PageId, user: Option<&'static User>, stats: Option<&'static Stats>) -> Box<dyn PageSession<'static>> {

    for (mask, new_fn) in DISPATCH_TABLE {
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
            let pageid = &pageid.reduced_by(reduce);
            // return new_fn.0(pageid, user, stats);
            return new_fn.0(pageid, user, stats);
        }
    }
    unreachable!();
}
