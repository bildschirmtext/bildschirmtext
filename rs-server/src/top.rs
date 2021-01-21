extern crate html2text;
#[cfg(unix)]
extern crate termion;
#[cfg(unix)]
extern crate unicode_width;
#[cfg(unix)]

use html2text::render::text_renderer::{RichAnnotation, TaggedLineElement};
use std::io::Read;

fn to_style(tag: &Vec<RichAnnotation>) -> String {
    let mut style = String::new();

    for annotation in tag {
        match *annotation {
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
