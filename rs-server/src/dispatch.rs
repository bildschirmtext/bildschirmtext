use std::collections::HashMap;
use crate::session::*;
use super::pages::*;
use super::user::*;

pub struct PrivateContext<'a> {
    pub user: Option<&'a User>,
    pub stats: Option<&'a Stats>,
}

pub fn get_page(pageid: &PageId, private_context: PrivateContext) -> Option<Page> {
    if pageid.page.starts_with("00000") || pageid.page == "9" {
        super::login::create(pageid, Some(private_context))
    } else if pageid.page == "77" {
        super::user::create(pageid, None)
    } else if pageid.page.starts_with('7') {
        super::historic::create(&pageid.reduced_by(1), None)
    } else {
        super::stat::create(pageid, None)
    }
}
