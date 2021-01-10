use std::collections::HashMap;

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
    palette: String,
    drcs: Cept,
    chars: Cept,
}

impl Image {
	fn compress(drcs_block: &[u8]) -> String {
        let mut drcs_block = drcs_block.to_owned();
		if drcs_block == b"@@@@@@@@@@" {
            drcs_block = vec!(0x20);
        } else if drcs_block == b"\x7f\x7f\x7f\x7f\x7f\x7f\x7f\x7f\x7f\x7f" {
			drcs_block = vec!(0x2f);
        } else {
			let y1 = 0;
			let max = 10;
			loop {
				let l = 0;
				for y2 in y1 + 1 .. max {
					if drcs_block[y2] != drcs_block[y1] {
                        break;
                    }
                    l += 1;
                }
				if l != 0 {
					drcs_block = drcs_block[..y1 + 1] + [0x20u8 + l] + drcs_block[y1 + l + 1..];
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

	fn new(url: &str, colors: u8, drcs_start: u8) -> Option(Self) {
        // default: colors = 16
        // default: drcs_start = 0x21

		if url == "" {
            return None;
        }
        println!("URL: {}", url);
        let image = if url.starts_with("http://") || url.starts_with("https://") {
            ImageLib.open(urllib.request.urlopen(url))
        } else {
            ImageLib.open(url)
        };
		image.load();
		let (width, height) = image.size;
		println!("mode: {}, resolution: {}*{}", , image.mode, str(width), str(height));

		let is_grayscale = image.mode == "L" || image.mode == "LA";

		// 4 shades of gray instead of 16, but double resolution
		// disabled, PIL doesn't select good base colors
//		if is_grayscale:
//			colors = 4
		// TODO: calling ImageMagick might give better results:
		// e.g. $ convert in.jpg -resize 54x80\! -dither FloydSteinberg -colors 16 out.png

		println!("target colors: {}", str(colors));

		let mut num_drcs = 0x7f - drcs_start;
		if colors == 16 {
            num_drcs = num_drcs / 2;
        }

		// calculate character resolution
		let exact_res_x = (num_drcs * width / height).sqrt();
		let exact_res_y = (num_drcs * height / width).sqrt();
		let aspect_ratio = width / height / PIXEL_ASPECT_RATIO;

//		println!("exact char resolution: {}", str(exact_res_x) + "*{}", str(exact_res_y))

		let res_x_1 = (exact_res_x).floor();
		let res_y_1 = (num_drcs / res_x_1).floor();
		let error_1 = (1 - (aspect_ratio / (res_x_1 / res_y_1))).abs();
		let res_x_2 = (exact_res_x).ceil();
		let res_y_2 = (num_drcs / res_x_2).floor();
		let error_2 = (1 - (aspect_ratio / (res_x_2 / res_y_2))).abs();
		let res_y_3 = (exact_res_y).floor();
		let res_x_3 = (num_drcs / res_y_3).floor();
		let error_3 = (1 - (aspect_ratio / (res_x_3 / res_y_3))).abs();
		let res_y_4 = (exact_res_y).ceil();
		let res_x_4 = (num_drcs / res_y_4).floor();
		let error_4 = (1 - (aspect_ratio / (res_x_4 / res_y_4))).abs();

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

		println!("char resolution:   {}*{}, error: {}", str(res_x), str(res_y), str(error));

		// remove alpha
		if image.mode == "RGBA" || image.mode == "LA" {
			let background = Image.new("RGB", image.size, (255, 255, 255));
			let index = if image.mode == "RGBA" { 3 } else { 1 };
			background.paste(image, mask=image.split()[index]);
            image = background;
        }

		// resample
		image = image.resize((res_x * 6, res_y * 10), resample = Image.ANTIALIAS);

		// convert to custom colors
		image = image.quantize(colors = colors, method = 0);
//		image = image.convert(mode = "P", colors = colors, dither = Image.FLOYDSTEINBERG, palette = Image.ADAPTIVE)

		image.save("/tmp/x.png");

		// create array with palette
		let p = image.getpalette();
		self.palette = [];
		for i in 0..colors {
			r = p[i * 3];
			g = p[i * 3 + 1];
			b = p[i * 3 + 2];
            self.palette.append("#{:02x}{:02x}{:02x}".format(r,g,b));
        }

//		println!("self.palette: {}", pprint.pformat(self.palette))

		// create drcs
		self.drcs = bytearray();

		let num_bits = if colors == 4 {
			2
        } else if colors == 16 {
            4
        };

		for base_y in (0 .. res_y * 10).step_by(10) {
			for base_x in (0 .. res_x * 6).step_by( 6) {
				for bitno in 0..num_bits {
					self.drcs.add_raw([0x30 + bitno]);
					let drcs_block = vec!();
					for y in 0..10 {
						let byte: u8 = 0;
						for x in 0..6 {
							byte <<= 1;
                            byte |= (image.getpixel((base_x + x, base_y + y)) >> bitno) & 1;
                        }
						byte |= 0x40;
                        drcs_block.push(byte);
                    }

					// compression
					drcs_block = Image::compress(drcs_block);

//					println!("drcs_block: {}", pprint.pformat(drcs_block))
                    self.drcs.add_raw(drcs_block)
                }
            }
        }

		println!("DRCs compressed {} down to {}", (40 * res_x * res_y).to_string(), (self.drcs.len()).to_string());

		let drcs_header = Cept::new();
		if colors == 4 {
			drcs_header.add_raw(b"\x1f\x23\x20\x4b\x42") // start defining 6x10 @ 4c
        } else if colors == 16 {
			drcs_header.add_raw(b"\x1f\x23\x20\x4b\x44") // start defining 6x10 @ 16c
        } else {
            error()
        }
		drcs_header.add_raw([0x1f, 0x23, drcs_start]);

		// prepend
		self.drcs[0..0] = drcs_header;

		// append
		if colors == 4 {
			// set colors to 16, 17, 18, 19
			self.drcs.add_raw(b"\x1f\x26\x20\x22\x20\x35\x40");
			self.drcs.add_raw(b"\x1f\x26\x30\x50");
			self.drcs.add_raw(b"\x1f\x26\x31\x51");
			self.drcs.add_raw(b"\x1f\x26\x32\x52");
            self.drcs.add_raw(b"\x1f\x26\x33\x53");
        }

		// create characters to print
		let step = if colors == 16 {
			2
        } else {
            1
        };
		self.chars = [];
		for y in 0..res_y {
			l = bytearray();
			for x in 0..res_x {
                l.append(drcs_start + (y * res_x + x) * step)
            }
            self.chars.append(l);
        }

        Self {

        }
    }

	fn create_image_page(&self) -> Page {
//		filename = "/Users/mist/Desktop/RGB_24bits_palette_sample_image.jpg"
//		filename = "/Users/mist/Desktop/Lenna_(test_image).png"
//		filename = "/Users/mist/Desktop/Wikipedia_logo_593.jpg"
		let filename = "/Users/mist/Desktop/220px-C64c_system.jpg";

		(palette, drcs, chars) = Image_UI.cept_from_image(filename);

		let data_cept = Cept::new();
		data_cept.define_palette(palette);
		data_cept.add_raw(drcs);

		data_cept.set_cursor(3, 1);
		data_cept.load_g0_drcs());
		for l in chars {
			data_cept.add_raw(l);
            data_cept.add_raw(b"\r\n");
        }

		let meta = Meta {
            clear_screen: Some(true),
            links: Some(vec![
                Link::new("0", "0"),
            ]),
            publisher_color: Some(7),
		};

        return Page { meta, data_cept };
    }
}