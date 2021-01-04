use std::collections::HashMap;
use crate::session::*;
use super::pages::*;
use super::user::*;

pub struct PrivateContext<'a> {
    pub user: Option<&'a User>,
    pub stats: Option<&'a Stats>,
}

enum CanSeePrivateContext {
    No,
    Yes,
}

// * If a mask does not end in '*' or '-', the page number must match exactly.
// * If a mask ends in '*', it only has to be a prefix of the page number.
// * If a mask ends in '-', it only has to be a prefix of the page number. The prefix of the
//   page number will be stripped when passed into the function.
// * Only use CanSeePrivateContext::Yes for BTX-internal pages that need to access the
//   user's info and statistics!
// N.B.: The table must be in the right order: longer prefixes must come first!
const DISPATCH_TABLE: &[(&[u8], fn(&PageId, Option<PrivateContext>) -> Option<Page>, CanSeePrivateContext)] = &[
    (b"00000*", super::login::create,    CanSeePrivateContext::Yes),
    (b"9",      super::login::create,    CanSeePrivateContext::Yes),
    (b"77",     super::user::create,     CanSeePrivateContext::No),
    (b"7-",     super::historic::create, CanSeePrivateContext::No),
    (b"*",      super::stat::create,     CanSeePrivateContext::No),
];

pub fn get_page(pageid: &PageId, private_context: PrivateContext) -> Option<Page> {
    for (mask, function, private) in DISPATCH_TABLE {
        let matches;
        let reduce;
        let last = *mask.last().unwrap();
        if last == b'*' || last == b'-' {
            let mask = std::str::from_utf8(&mask[0..mask.len() - 1]).unwrap();
            matches = pageid.page.starts_with(mask);
            reduce = if last == b'*' { 0 } else { mask.len() };
        } else {
            matches = pageid.page == std::str::from_utf8(mask).unwrap();
            reduce = 0;
        };
        if matches {
            let private_context = match private {
                CanSeePrivateContext::Yes => Some(private_context),
                CanSeePrivateContext::No => None,
            };
            return function(&pageid.reduced_by(reduce), private_context);
        }
    }
    return None;
}
