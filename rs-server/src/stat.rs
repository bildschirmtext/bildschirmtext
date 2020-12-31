use std::fs::File;
use std::io::Read;
use std::fs::metadata;
use super::pages::*;

const PATH_DATA: &str = "../data/";

pub fn create(pageid: &str) -> Page {
    let mut cept = None;

    if let Some((basedir, filename)) = find_basedir(pageid) {
        let mut basename = basedir.clone();
        basename += filename;

        let mut filename_meta = basename.clone();
        filename_meta += ".meta";
        let mut filename_cept = basename.clone();
        filename_cept += ".cept";
        let mut filename_cept = basename.clone();
        filename_cept += ".cept";

        if is_file(&filename_meta) {
            println!("found: {}", filename_meta);
            // let data_cept = None;
            // with open(filename_meta) as f
            // meta = json.load(f)
            if is_file(&filename_cept) {
                let mut buf : Vec<u8> = vec!();
                let mut f = File::open(&filename_cept).unwrap();
                f.read_to_end(&mut buf);
                cept = Some(buf);
            // } elif os.path.isfile(filename_cm) {
            //     data_cept = CM.read(filename_cm)
            }
            // break;
        }
    }

    // if data_cept is None {
    //     return None
    // }
    let meta = Meta {
        publisher_name: Some("!BTX".to_owned()),
        clear_screen: true,
        cls2: false,
        parallel_mode: false,
        links: vec![
            ("0".to_owned(), "0".to_owned()),
            ("10".to_owned(), "710".to_owned()),
            ("11".to_owned(), "711".to_owned()),
            ("#".to_owned(), "711".to_owned()),
        ],
        publisher_color: 7,
        inputs: None,
    };
    let mut page = Page::new(meta);
    page.cept.add_raw(&cept.unwrap());
    page
}

fn is_dir(path: &str) -> bool {
    if let Ok(md) = metadata(path) {
        md.is_dir()
    } else {
        false
    }
}

fn is_file(path: &str) -> bool {
    if let Ok(md) = metadata(path) {
        md.is_file()
    } else {
        false
    }
}

fn find_basedir(pageid: &str) -> Option<(String, &str)> {
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
