use std::{collections::HashMap, fs::File};
use std::io::Read;
use std::fs::metadata;
use crate::user::*;

use super::page::*;
use super::session::*;
use super::paths::*;
use super::dispatch::*;

pub struct StaticPageSession<'a> {
    pageid: &'a PageId,
}

pub fn new<'a>(pageid: &'a PageId, _: &'a User, _: &'a Stats) -> Box<dyn PageSession<'a> + 'a> {
    Box::new(StaticPageSession { pageid })
}

impl<'a> PageSession<'a> for StaticPageSession<'a> {
    fn create(&self) -> Option<Page> {
        let mut cept = None;

        if let Some((basedir, filename)) = find_basedir(self.pageid) {
            let mut basename = basedir.clone();
            basename += &filename;

            let mut filename_meta = basename.clone();
            filename_meta += ".meta";
            let mut filename_cept = basename.clone();
            filename_cept += ".cept";
            let mut filename_glob = basedir.clone();
            filename_glob += "a.glob";

            if is_file(&filename_meta) {
                // read meta
                println!("filename_meta: {}", filename_meta);
                let f = File::open(&filename_meta).unwrap();
                let mut meta: Meta = serde_json::from_reader(f).unwrap();

                // read glob
                println!("filename_glob: {}", filename_glob);
                let f = File::open(&filename_glob).unwrap();
                let glob_meta: Meta = serde_json::from_reader(f).unwrap();

                meta.merge(glob_meta);

                println!("filename_cept: {}", filename_cept);
                if is_file(&filename_cept) {
                    let mut buf : Vec<u8> = vec!();
                    let mut f = File::open(&filename_cept).unwrap();
                    f.read_to_end(&mut buf);
                    cept = Some(buf);
                }
                let mut page = Page::new(meta);
                page.cept.add_raw(&cept.unwrap());
                return Some(page);
            }
        }

        None
    }

    fn validate(&self, _: &str, _: &HashMap<String, String>) -> ValidateResult {
        unreachable!()
    }

    fn send(&self, _: &HashMap<String, String>) -> UserRequest {
        unreachable!()
    }
}

////////////////////////////////////////////////////////////////////////////////

fn is_dir(path: &str) -> bool {
    if let Ok(md) = metadata(path) {
        md.is_dir()
    } else {
        false
    }
}

pub fn is_file(path: &str) -> bool {
    if let Ok(md) = metadata(path) {
        md.is_file()
    } else {
        false
    }
}

pub fn find_basedir(pageid: &PageId) -> Option<(String, String)> {
    let pageid_str = pageid.to_string();
    let pageid = pageid_str.as_bytes();
    for dir in [ "", "hist/10/", "hist/11/" ].iter() {
        for i in (0..pageid.len()).rev() {
            let mut testdir = String::new();
            testdir += PATH_DATA;
            testdir += dir;
            testdir += std::str::from_utf8(&pageid[..i+1]).unwrap();
            if is_dir(&testdir) {
                let filename = std::str::from_utf8(&pageid[i+1..]).unwrap().to_owned();
                println!("filename: {}", filename);
                let mut basedir = testdir.clone();
                basedir.push('/');
                return Some((basedir, filename));
            }
        }
    }
    return None
}
