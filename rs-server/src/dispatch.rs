use crate::session::*;
use super::page::*;
use super::user::*;

pub struct PrivateContext<'a> {
    pub user: Option<&'a User>,
    pub stats: Option<&'a Stats>,
}

enum CanSeePrivateContext {
    No(fn(&PageId) -> Option<Page>),
    Yes(fn(&PageId, PrivateContext) -> Option<Page>),
}

// Mask:
//   * If a mask does not end in '*' or '-', the page number must match exactly.
//   * If a mask ends in '*', it only has to be a prefix of the page number.
//   * If a mask ends in '-', it only has to be a prefix of the page number. The prefix of the
//     page number will be stripped when passed into the function.
// Function:
//   * Only use CanSeePrivateContext::Yes for BTX-internal pages that need to access the
//     user's info and statistics!
// N.B.: The table must be in the right order: longer prefixes must come first!
const DISPATCH_TABLE: &[(&[u8], CanSeePrivateContext)] = &[
    (b"00000*", CanSeePrivateContext::Yes(super::login::create)),
    (b"9",      CanSeePrivateContext::Yes(super::login::create)),
    (b"77",     CanSeePrivateContext::No (super::ui_user::create)),
    (b"7-",     CanSeePrivateContext::No (super::historic::create)),
    (b"*",      CanSeePrivateContext::No (super::staticp::create)),
];

pub fn get_page(pageid: &PageId, private_context: PrivateContext) -> Option<Page> {
    for (mask, function) in DISPATCH_TABLE {
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
            return match function {
                CanSeePrivateContext::No(function) => {
                    function(pageid)
                }
                CanSeePrivateContext::Yes(function) => {
                    function(pageid, private_context)
                }
            };
        }
    }
    return None;
}
