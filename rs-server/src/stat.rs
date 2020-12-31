use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::fs::File;
use std::io::Read;
use std::fs::metadata;
use super::pages::*;

const PATH_DATA: &str = "../data/";


pub fn create(pageid: &str) -> Option<Page> {
    let mut cept = None;

    if let Some((basedir, filename)) = find_basedir(pageid) {
        let mut basename = basedir.clone();
        basename += filename;

        let mut filename_meta = basename.clone();
        filename_meta += ".meta";
        let mut filename_cept = basename.clone();
        filename_cept += ".cept";
        let mut filename_glob = basedir.clone();
        filename_glob += "a.glob";

        if is_file(&filename_meta) {
            // read meta
            println!("filename_meta: {}", filename_meta);
            let mut f = File::open(&filename_meta).unwrap();
            let mut meta: Meta = serde_json::from_reader(f).unwrap();
            // println!("{:?}", meta);

            // read glob
            println!("filename_glob: {}", filename_glob);
            let mut f = File::open(&filename_glob).unwrap();
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

pub fn find_basedir(pageid: &str) -> Option<(String, &str)> {
    let pageid = pageid.as_bytes();
    for dir in [ "", "hist/10/", "hist/11/" ].iter() {
        for i in (0..pageid.len()).rev() {
            let mut testdir = String::new();
            testdir += PATH_DATA;
            testdir += dir;
            testdir += std::str::from_utf8(&pageid[..i+1]).unwrap();
            if is_dir(&testdir) {
                let filename = std::str::from_utf8(&pageid[i+1..]).unwrap();
                println!("filename: {}", filename);
                let mut basedir = testdir.clone();
                basedir.push('/');
                return Some((basedir, filename));
            }
        }
    }
    return None
}
