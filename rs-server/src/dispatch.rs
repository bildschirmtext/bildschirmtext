use std::collections::HashMap;
use crate::session::*;
use super::page::*;
use super::user::*;

pub struct PrivateContext<'a> {
    pub user: Option<&'a User>,
    pub stats: Option<&'a Stats>,
}

pub struct UserFns {
    pub create: fn(&PageId, PrivateContext) -> Option<Page>,
    pub validate: Option<fn(&PageId, name: &str, input_data: &HashMap<String, String>, PrivateContext) -> ValidateResult>,
    pub send: Option<fn(&PageId, input_data: &HashMap<String, String>, PrivateContext) -> UserRequest>,
}


pub struct AnonymousUserFns {
    pub create: fn(&PageId) -> Option<Page>,
    pub validate: Option<fn(&PageId, name: &str, input_data: &HashMap<String, String>) -> ValidateResult>,
    pub send: Option<fn(&PageId, input_data: &HashMap<String, String>) -> UserRequest>,
}

pub enum Anonymous {
    Yes(AnonymousUserFns),
    No(UserFns),
}

// Mask:
//   * If a mask does not end in '*' or '-', the page number must match exactly.
//   * If a mask ends in '*', it only has to be a prefix of the page number.
//   * If a mask ends in '-', it only has to be a prefix of the page number. The prefix of the
//     page number will be stripped when passed into the function.
// Function:
//   * Only use Anonymous::Yes for BTX-internal pages that need to access the
//     user's info and statistics!
// N.B.: The table must be in the right order: longer prefixes must come first!
const DISPATCH_TABLE: &[(&[u8], Anonymous)] = &[
    (b"00000*", Anonymous::No(super::login::FUNCTIONS)),
    (b"9",      Anonymous::No(super::login::FUNCTIONS)),
    (b"8*",     Anonymous::No(super::ui_messaging::FUNCTIONS)),
    (b"77",     Anonymous::Yes(super::ui_user::FUNCTIONS)),
    (b"7-",     Anonymous::Yes(super::historic::FUNCTIONS)),
    (b"*",      Anonymous::Yes(super::staticp::FUNCTIONS)),
];

pub fn dispatch_pageid(pageid: &PageId) -> &Anonymous {
    for (mask, functions) in DISPATCH_TABLE {
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
            return functions;
        }
    }
    unreachable!();
}
