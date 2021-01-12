use std::{collections::HashMap, fs::File};
use std::io::Read;
use std::fs::metadata;
use crate::{cept::Cept, user::*};

use super::page::*;
use super::session::*;
use super::paths::*;
use super::dispatch::*;

pub struct StaticPageSession {
    pageid: PageId,
}

pub fn new<'a>(pageid: PageId, _: User) -> Box<dyn PageSession<'a> + 'a> {
    Box::new(StaticPageSession { pageid })
}

impl<'a> PageSession<'a> for StaticPageSession {
    fn create(&self) -> Option<Page> {
        if let Some((basedir, filename)) = find_basedir(&self.pageid) {
            // read meta
            let filename_meta = resource_filename(&basedir, &filename, "meta");
            println!("filename_meta: {}", filename_meta);
            let f = File::open(&filename_meta).ok()?;
            let mut meta: Meta = serde_json::from_reader(f).ok()?;

            // read glob
            let filename_glob = resource_filename(&basedir, "a", "glob");
            println!("filename_glob: {}", filename_glob);
            let f = File::open(&filename_glob).ok()?;
            let glob_meta: Meta = serde_json::from_reader(f).ok()?;

            // global overrides local
            meta.merge(glob_meta);

            // read text
            let filename_cept = resource_filename(&basedir, &filename, "cept");
            println!("filename_cept: {}", filename_cept);
            let mut buf : Vec<u8> = vec!();
            let mut f = File::open(&filename_cept).ok()?;
            f.read_to_end(&mut buf).ok()?;
            let mut cept = Cept::new();
            cept.add_raw(&buf);

            let cept_palette = load_palette(meta.palette.as_ref(), &basedir);
            let cept_include = load_include(meta.include.as_ref(), &basedir);

            return Some(Page {
                meta,
                cept_palette,
                cept_include,
                cept,
            });
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

fn resource_filename(basedir: &str, resource_name: &str, extention: &str) -> String {
    let mut filename = basedir.to_owned();
    filename += resource_name;
    filename.push('.');
    filename += extention;
    filename
}

fn load_palette(palette_name: Option<&String>, basedir: &str) -> Option<Cept> {
    if let Some(palette_name) = palette_name {
        let filename = resource_filename(basedir, palette_name, "pal");
        println!("loading: {}", filename);
        if let Ok(f) = File::open(&filename) {
            let palette: Result<Palette, _> = serde_json::from_reader(f);
            if let Ok(palette) = palette {
                let mut cept = Cept::new();
                cept.define_palette(&palette.palette, palette.start_color);
                return Some(cept);
            } else {
                println!("ERROR reading palette file! [1]");
                return None;
            }
        } else {
            println!("ERROR reading palette file! [2]");
            return None;
        }
    } else {
        None
    }
}

fn load_include(include_name: Option<&String>, basedir: &str) -> Option<Cept> {
    if let Some(include_name) = include_name {
        let filename = resource_filename(basedir, include_name, "inc");
        let mut cept_include : Vec<u8> = vec!();
        println!("loading: {}", filename);
        if let Ok(mut f) = File::open(&filename) {
            if let Ok(_) = f.read_to_end(&mut cept_include) {
                // ok
            } else {
                println!("ERROR reading include file! [1]");
            }
        } else {
            println!("ERROR creating user! [1]");
        }
        let mut cept = Cept::new();
        // palette definition has to end with 0x1f; add one if
        // the include data doesn't start with one
        if cept_include[0] != 0x1f {
            cept.set_cursor(1, 1)
        }
        cept.add_raw(&cept_include);
        return Some(cept);
    }
    None
}