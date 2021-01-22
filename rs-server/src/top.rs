extern crate html2text;

use super::cept::*;

use html2text::render::text_renderer::{RichAnnotation, TaggedLineElement};
use std::io::Read;
#[derive(Default)]
struct CeptState {
    italics: bool,
    bold: bool,
    link: bool,
    code: bool,
}

fn to_style(state: &mut CeptState, tag: &Vec<RichAnnotation>) -> Cept {
    let mut cept = Cept::new();

    let mut state = CeptState::default();

    for annotation in tag {
        // println!("{:?}", annotation);
        match *annotation {
            RichAnnotation::Default => (),
            RichAnnotation::Link(_) => {
                state.link = true;
            },
            RichAnnotation::Image => {
                state.link = true;
            },
            RichAnnotation::Emphasis => {
                state.bold = true;
            },
            RichAnnotation::Strong => {
                state.bold = true;
            },
            RichAnnotation::Strikeout => (),
            RichAnnotation::Code => {
                state.code = true;
            },
            RichAnnotation::Preformat(is_cont) => {
                // if is_cont {
                //     style.push_str(&format!(
                //         "{}",
                //         termion::color::Fg(termion::color::LightMagenta)
                //     ));
                // } else {
                //     style.push_str(&format!("{}", termion::color::Fg(termion::color::Magenta)));
                // }
            }
        }
    }

    if state.italics {
        cept.set_fg_color(CeptColor::Cyan as u8);
    } else if state.bold {
        cept.set_fg_color(CeptColor::Black as u8);
    }
    if state.code {
        cept.set_bg_color(CeptColor::Cyan as u8);
    } else {
        cept.set_bg_color(CeptColor::White as u8);
    }
    if state.link {
        cept.underline_on();
        cept.set_fg_color(CeptColor::Blue as u8);
    }
    if !state.italics && !state.bold && !state.link && !state.code {
		cept.set_fg_color(CeptColor::Gray as u8);
		cept.set_bg_color(CeptColor::White as u8);
        cept.underline_off();
    }

    cept
}

pub fn html2cept(file: &mut impl Read) -> Vec<Cept>{
    let width: u16 = 40;

    let annotated = html2text::from_read_rich(file, width as usize);
    // println!("{:?}", annotated);

    let mut lines = 0;
    let mut cepts = vec!();
    let mut cept = Cept::new();
    let mut state = CeptState::default();

    for (i, line) in annotated.iter().enumerate() {
        use self::TaggedLineElement::Str;

        let mut x = 0;
        let mut debug_line = String::new();
        for tli in line.iter() {
            if let Str(ts) = tli {
                let cept_style = to_style(&mut state, &ts.tag);
                cept += cept_style;
                cept.add_str(&ts.s);
                x += ts.s.chars().count();
                debug_line += &ts.s;
            }
        }
        println!("{:02} {}", x, debug_line);
        if x != 40 {
            cept.add_str("\n");
        }
        lines += 1;
        if lines == 20 {
            cepts.push(cept);
            lines = 0;
            cept = Cept::new();
        }
    }
    cepts.push(cept);

    cepts
}
