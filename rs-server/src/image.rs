use std::{collections::HashMap, convert::TryInto};
use image::{Pixel, imageops};
use image::GenericImageView;
use exoquant::*;

use super::dispatch::*;
use super::session::*;
use super::page::*;
use super::user::*;
use super::cept::*;

const PIXEL_ASPECT_RATIO: f32 = 0.92;

pub struct ImagePageSession {
    pageid: PageId,
}

pub fn new<'a>(pageid: PageId, _: User) -> Box<dyn PageSession<'a> + 'a> {
    Box::new(ImagePageSession { pageid })
}

impl<'a> PageSession<'a> for ImagePageSession {
    fn create(&self) -> Option<Page> {
        if self.pageid.page == "666" {
            Some(self.create_image_page())
        } else {
            None
        }
    }

    fn validate(&self, _: &str, _: &HashMap<String, String>) -> ValidateResult {
        unreachable!()
    }

    fn send(&self, _: &HashMap<String, String>) -> UserRequest {
        unreachable!()
    }
}

struct Image {
    palette: Vec<String>,
    drcs: Cept,
    chars: Vec<Vec<u8>>,
}

impl Image {
	fn compress(drcs_block: &[u8]) -> Vec<u8> {
        let mut drcs_block = drcs_block.to_owned();
		if drcs_block == b"@@@@@@@@@@" {
            drcs_block = vec!(0x20);
        } else if drcs_block == b"\x7f\x7f\x7f\x7f\x7f\x7f\x7f\x7f\x7f\x7f" {
			drcs_block = vec!(0x2f);
        } else {
			let mut y1 = 0;
			let mut max = 10;
			loop {
				let mut l: usize = 0;
				for y2 in y1 + 1 .. max {
					if drcs_block[y2] != drcs_block[y1] {
                        break;
                    }
                    l += 1;
                }
				if l != 0 {
                    let mut x = drcs_block[..y1 + 1].to_owned();
                    x.push(0x20);
                    x.extend_from_slice(&drcs_block[y1 + l + 1..]);
					drcs_block = x;
					y1 += 1;
                    max -= l - 1;
                }
				y1 += 1;
				if y1 == max {
                    break
                }
            }
        }
        drcs_block
    }

	fn new(url: &str, num_colors: Option<u8>, drcs_start: Option<u8>) -> Option<Self> {
        let num_colors = num_colors.unwrap_or(16);
        let drcs_start = drcs_start.unwrap_or(0x21);

		if url == "" {
            return None;
        }
        println!("URL: {}", url);
        // let image = if url.starts_with("http://") || url.starts_with("https://") {
        //     ImageLib.open(urllib.request.urlopen(url))
        // } else {
            // ImageLib.open(url)
        // };
        let image = image::open(url).unwrap();
		let (width, height) = image.dimensions();
		println!("color: {:?}, resolution: {}*{}", image.color(), width, height);

		let is_grayscale = !image.color().has_color();

		// 4 shades of gray instead of 16, but double resolution
		// disabled, PIL doesn't select good base colors
//		if is_grayscale:
//			num_colors = 4
		// TODO: calling ImageMagick might give better results:
		// e.g. $ convert in.jpg -resize 54x80\! -dither FloydSteinberg -colors 16 out.png

		println!("target colors: {}", num_colors);

		let mut num_drcs: u8 = 0x7f - drcs_start;
		if num_colors == 16 {
            num_drcs = num_drcs / 2;
        }
        let num_drcs_f32 = num_drcs as f32;

		// calculate character resolution
		let exact_res_x = (num_drcs_f32 * width as f32 / height as f32).sqrt();
		let exact_res_y = (num_drcs_f32 * height as f32 / width as f32).sqrt();
		let aspect_ratio = width as f32 / height as f32 / PIXEL_ASPECT_RATIO;

//		println!("exact char resolution: {}", str(exact_res_x) + "*{}", str(exact_res_y))

        let res_x_1 = (exact_res_x).floor();
        let res_y_1 = (num_drcs_f32 / res_x_1).floor();
        let error_1 = (1.0 - (aspect_ratio / (res_x_1 / res_y_1))).abs();
        let res_x_2 = (exact_res_x).ceil();
        let res_y_2 = (num_drcs_f32 / res_x_2).floor();
        let error_2 = (1.0 - (aspect_ratio / (res_x_2 / res_y_2))).abs();
        let res_y_3 = (exact_res_y).floor();
        let res_x_3 = (num_drcs_f32 / res_y_3).floor();
        let error_3 = (1.0 - (aspect_ratio / (res_x_3 / res_y_3))).abs();
        let res_y_4 = (exact_res_y).ceil();
        let res_x_4 = (num_drcs_f32 / res_y_4).floor();
        let error_4 = (1.0 - (aspect_ratio / (res_x_4 / res_y_4))).abs();

//		println!("char resolution 1: {}", str(res_x_1) + "*{}", str(res_y_1) + ", error: {}", str(error_1))
//		println!("char resolution 2: {}", str(res_x_2) + "*{}", str(res_y_2) + ", error: {}", str(error_2))
//		println!("char resolution 3: {}", str(res_x_3) + "*{}", str(res_y_3) + ", error: {}", str(error_3))
//		println!("char resolution 4: {}", str(res_x_4) + "*{}", str(res_y_4) + ", error: {}", str(error_4))

		let mut res_x = res_x_1;
		let mut res_y = res_y_1;
		let mut error = error_1;
		if error_2 < error {
			res_x = res_x_2;
			res_y = res_y_2;
            error = error_2;
        }
		if error_3 < error {
			res_x = res_x_3;
			res_y = res_y_3;
            error = error_3;
        }
		if error_4 < error {
			res_x = res_x_4;
			res_y = res_y_4;
            error = error_4;
        }

        let res_x = res_x as u32;
        let res_y = res_y as u32;

		println!("char resolution:   {}*{}, error: {}", res_x, res_y, error);

		// // remove alpha
		// if image.color().has_alpha() {
		// 	let background = Image.new("RGB", image.size, (255, 255, 255));
		// 	let index = if image.mode == "RGBA" { 3 } else { 1 };
		// 	background.paste(image, mask=image.split()[index]);
        //     image = background;
        // }

		// resample
		let image = image.resize(res_x * 6, res_y * 10, imageops::FilterType::Lanczos3);

        // convert to custom colors
        // https://github.com/myfreeweb/imgroll
        let pixels = image
            .pixels()
            .map(|(_, _, p)| {
                let cols = p.channels();
                Color::new(cols[0], cols[1], cols[2], cols[3])
            })
            .collect::<Vec<_>>();
        let (palette, indexed_pixels) = convert_to_indexed(
            &pixels,
            image.width() as usize,
            num_colors as usize,
            &optimizer::KMeans,
            &ditherer::FloydSteinberg::checkered(),
        );

		// image.save("/tmp/x.png");

        let mut si = Image {
            palette: vec!(),
            drcs: Cept::new(),
            chars: vec!(vec!()),
        };

		// create array with palette
		for i in 0..num_colors {
			let r = palette[i as usize].r;
			let g = palette[i as usize].g;
			let b = palette[i as usize].b;
            si.palette.push(format!("#{:02x}{:02x}{:02x}", r, g, b));
        }

//		println!("si.palette: {}", pprint.pformat(si.palette))

		// create drcs
		si.drcs = Cept::new();

		let num_bits = if num_colors == 4 {
			2
        } else if num_colors == 16 {
            4
        } else {
            panic!();
        };

		for base_y in (0 .. res_y * 10).step_by(10) {
			for base_x in (0 .. res_x * 6).step_by( 6) {
				for bitno in 0..num_bits {
					si.drcs.add_raw(&[0x30 + bitno]);
					let mut drcs_block = vec!();
					for y in 0..10 {
						let mut byte: u8 = 0;
						for x in 0..6 {
							byte <<= 1;
                            byte |= (indexed_pixels[((base_x + x) + res_y * (base_y + y)) as usize] >> bitno) & 1;
                        }
						byte |= 0x40;
                        drcs_block.push(byte);
                    }

					// compression
					// drcs_block = Image::compress(drcs_block);

//					println!("drcs_block: {}", pprint.pformat(drcs_block))
                    si.drcs.add_raw(&drcs_block)
                }
            }
        }

		println!("DRCs compressed {} down to {}", (40 * res_x * res_y).to_string(), (si.drcs.data().len()).to_string());

		let mut drcs_header = Cept::new();
		if num_colors == 4 {
			drcs_header.add_raw(b"\x1f\x23\x20\x4b\x42") // start defining 6x10 @ 4c
        } else if num_colors == 16 {
			drcs_header.add_raw(b"\x1f\x23\x20\x4b\x44") // start defining 6x10 @ 16c
        } else {
            panic!()
        }
		drcs_header.add_raw(&[0x1f, 0x23, drcs_start]);

        // prepend
        let mut x = Cept::new();
        x.add_raw(drcs_header.data());
        x.add_raw(si.drcs.data());
		si.drcs = x;

		// append
		if num_colors == 4 {
			// set colors to 16, 17, 18, 19
			si.drcs.add_raw(b"\x1f\x26\x20\x22\x20\x35\x40");
			si.drcs.add_raw(b"\x1f\x26\x30\x50");
			si.drcs.add_raw(b"\x1f\x26\x31\x51");
			si.drcs.add_raw(b"\x1f\x26\x32\x52");
            si.drcs.add_raw(b"\x1f\x26\x33\x53");
        }

		// create characters to print
		let step = if num_colors == 16 {
			2
        } else {
            1
        };
		si.chars = vec!();
		for y in 0..res_y {
			let mut l = vec!();
			for x in 0..res_x {
                l.push(drcs_start + ((y * res_x + x) * step) as u8)
            }
            si.chars.push(l);
        }

        Some(si)
    }
}

impl ImagePageSession {
	fn create_image_page(&self) -> Page {
//		filename = "/Users/mist/Desktop/RGB_24bits_palette_sample_image.jpg"
//		filename = "/Users/mist/Desktop/Lenna_(test_image).png"
//		filename = "/Users/mist/Desktop/Wikipedia_logo_593.jpg"
		let filename = "/Users/mist/Desktop/test.jpg";

        let image = Image::new(filename, None, None).unwrap();
		// let (palette, drcs, chars) = Image_UI.cept_from_image(filename);

		let mut cept = Cept::new();
		cept.define_palette(&image.palette, None);
		cept.add_raw(image.drcs.data());

		cept.set_cursor(3, 1);
		cept.load_g0_drcs();
		for l in image.chars {
			cept.add_raw(&l);
            cept.add_raw(b"\r\n");
        }

		let meta = Meta {
            clear_screen: Some(true),
            links: Some(vec![
                Link::new("0", "0"),
            ]),
            publisher_color: Some(7),

            ..Default::default()
		};

        return Page { meta, cept };
    }
}