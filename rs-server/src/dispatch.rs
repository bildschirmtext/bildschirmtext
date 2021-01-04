use std::collections::HashMap;
use crate::session::*;
use super::pages::*;
use super::user::*;

pub fn get_page(pageid: &PageId, user: Option<&User>, stats: Option<&Stats>) -> Option<Page> {
    if pageid.page.starts_with("00000") || pageid.page == "9" {
        super::login::create(pageid, user, stats)
    } else if pageid.page == "77" {
        super::user::create(pageid)
    } else if pageid.page.starts_with('7') {
        super::historic::create(&pageid.reduced_by(1))
    } else {
        super::stat::create(pageid)
    }
}
