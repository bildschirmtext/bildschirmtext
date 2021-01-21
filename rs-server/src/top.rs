extern crate html2text;
#[cfg(unix)]
extern crate termion;
#[cfg(unix)]
extern crate unicode_width;
#[cfg(unix)]

use html2text::render::text_renderer::{RichAnnotation, TaggedLine, TaggedLineElement};
use std::{collections::HashMap, io::Read};
use std::io::{self, Write};
use termion::cursor::Goto;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use unicode_width::UnicodeWidthStr;

fn to_style(tag: &Vec<RichAnnotation>) -> String {
    let mut style = String::new();

    for ann in tag {
        match *ann {
            RichAnnotation::Default => (),
            RichAnnotation::Link(_) => {
                style.push_str(&format!("{}", termion::style::Underline));
            }
            RichAnnotation::Image => {
                style.push_str(&format!(
                    "{}",
                    termion::color::Fg(termion::color::LightBlue)
                ));
            }
            RichAnnotation::Emphasis => {
                style.push_str(&format!(
                    "{}",
                    termion::color::Fg(termion::color::LightGreen)
                ));
            }
            RichAnnotation::Strong => {
                style.push_str(&format!(
                    "{}",
                    termion::color::Fg(termion::color::LightGreen)
                ));
            }
            RichAnnotation::Strikeout => (),
            RichAnnotation::Code => {
                style.push_str(&format!(
                    "{}",
                    termion::color::Fg(termion::color::LightYellow)
                ));
            }
            RichAnnotation::Preformat(is_cont) => {
                if is_cont {
                    style.push_str(&format!(
                        "{}",
                        termion::color::Fg(termion::color::LightMagenta)
                    ));
                } else {
                    style.push_str(&format!("{}", termion::color::Fg(termion::color::Magenta)));
                }
            }
        }
    }
    style
}

struct LinkMap {
    lines: Vec<Vec<Option<String>>>, // lines[y][x] => Some(URL) or None
}

impl LinkMap {
    pub fn link_at(&self, x: usize, y: usize) -> Option<&str> {
        if let Some(ref linevec) = self.lines.get(y) {
            if let Some(&Some(ref text)) = linevec.get(x) {
                return Some(&text);
            }
        }
        None
    }
}

fn link_from_tag(tag: &Vec<RichAnnotation>) -> Option<String> {
    let mut link = None;
    for annotation in tag {
        if let RichAnnotation::Link(ref text) = *annotation {
            link = Some(text.clone());
        }
    }
    link
}

fn find_links(lines: &Vec<TaggedLine<Vec<RichAnnotation>>>) -> LinkMap {
    use self::TaggedLineElement::Str;

    let mut map = Vec::new();
    for line in lines {
        let mut linevec = Vec::new();

        for tli in line.iter() {
            if let Str(ts) = tli {
                let link = link_from_tag(&ts.tag);
                for _ in 0..UnicodeWidthStr::width(ts.s.as_str()) {
                    linevec.push(link.clone());
                }
            }
        }

        map.push(linevec);
    }
    LinkMap { lines: map }
}

struct FragMap {
    start_xy: HashMap<String, (usize, usize)>,
}

fn find_frags(lines: &Vec<TaggedLine<Vec<RichAnnotation>>>) -> FragMap {
    use self::TaggedLineElement::*;

    let mut map = HashMap::new();
    let mut y = 0;
    for line in lines {
        let mut x = 0;
        for tli in line.iter() {
            match tli {
                FragmentStart(fragname) => {
                    map.insert(fragname.to_string(), (x, y));
                }
                Str(ts) => {
                    x += UnicodeWidthStr::width(ts.s.as_str());
                }
            }
        }
        y += 1;
    }
    FragMap { start_xy: map }
}

pub fn html2term(file: &mut impl Read) {
    let width: u16 = 40;

    let annotated = html2text::from_read_rich(file, width as usize);

    for (i, line) in annotated.iter().enumerate() {
        use self::TaggedLineElement::Str;

        for tli in line.iter() {
            if let Str(ts) = tli {
                let style = to_style(&ts.tag);
                print!("{}{}{}", style, ts.s, termion::style::Reset);
            }
        }
        println!("");
    }
}
