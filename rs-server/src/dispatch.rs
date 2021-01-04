use std::collections::HashMap;
use crate::session::*;
use super::pages::*;
use super::user::*;

pub struct PrivateContext<'a> {
    pub user: Option<&'a User>,
    pub stats: Option<&'a Stats>,
}

const DISPATCH_TABLE: &[(&[u8], fn(&PageId, Option<PrivateContext>) -> Option<Page>, usize, bool)] = &[
    (b"00000*", super::login::create,    0, true),
    (b"9",      super::login::create,    0, true),
    (b"77",     super::user::create,     0, false),
    (b"7*",     super::historic::create, 1, false),
    (b"*",      super::stat::create,     0, false),
];

pub fn get_page(pageid: &PageId, private_context: PrivateContext) -> Option<Page> {
    for (mask, function, reduce, private) in DISPATCH_TABLE {
        let matches = if *mask.last().unwrap() == b'*' {
            let mask = std::str::from_utf8(&mask[0..mask.len() - 1]).unwrap();
            pageid.page.starts_with(mask)
        } else {
            pageid.page == std::str::from_utf8(mask).unwrap()
        };
        if matches {
            return function(&pageid.reduced_by(*reduce), if *private { Some(private_context) } else { None });
        }
    }
    return None;
}
