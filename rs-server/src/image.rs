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
            Some(create_image_page())
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

impl ImagePageSession {

	// palette = None
	// drcs = None
	// chars = None

	fn compress(drcs_block: &[u8]) {
		if drcs_block == bytearray(b'@@@@@@@@@@') {
            drcs_block = bytearray(b'\x20')
        } else if drcs_block == bytearray(b'\x7f\x7f\x7f\x7f\x7f\x7f\x7f\x7f\x7f\x7f') {
			drcs_block = bytearray(b'\x2f')
        } else {
			y1 = 0;
			max = 10;
			loop {
				l = 0;
				for y2 in range(y1 + 1, max) {
					if drcs_block[y2] != drcs_block[y1] {
                        break;
                    }
					l += 1;
				if l {
					drcs_block = drcs_block[..y1 + 1] + bytes([0x20 + l]) + drcs_block[y1 + l + 1..];
					y1 += 1;
                    max -= l - 1;
                }
				y1 += 1;
				if y1 == max {
                    break
                }
            }
        }
        return drcs_block;
    }

	fn new(url: &str, colors: u8, drcs_start: u8) {
        // default: colors = 16
        // default: drcs_start = 0x21

		if url == "" {
            return None;
        }
		prinln!("URL: " + pprint.pformat(url) + "\n");
		if url.startswith("http://") || url.startswith("https://") {
            image = Image.open(urllib.request.urlopen(url))
        } else {
            image = Image.open(url)
        }
		image.load();
		(width, height) = image.size;
		prinln!("mode: " + image.mode + ", resolution: " + str(width) + "*" + str(height) + "\n");

		is_grayscale = image.mode == "L" || image.mode == "LA";

		// 4 shades of gray instead of 16, but double resolution
		// disabled, PIL doesn't select good base colors
//		if is_grayscale:
//			colors = 4
		// TODO: calling ImageMagick might give better results:
		// e.g. $ convert in.jpg -resize 54x80\! -dither FloydSteinberg -colors 16 out.png

		prinln!("target colors: " + str(colors) + "\n")

		num_drcs = 0x7f - drcs_start;
		if colors == 16 {
            num_drcs = int(num_drcs / 2)
        }

		// calculate character resolution
		exact_res_x = math.sqrt(num_drcs * width / height);
		exact_res_y = math.sqrt(num_drcs * height / width);
		aspect_ratio = width / height / PIXEL_ASPECT_RATIO;

//		prinln!("exact char resolution: " + str(exact_res_x) + "*" + str(exact_res_y) + "\n")

		res_x_1 = math.floor(exact_res_x);
		res_y_1 = math.floor(num_drcs / res_x_1);
		error_1 = abs(1 - (aspect_ratio / (res_x_1 / res_y_1)));
		res_x_2 = math.ceil(exact_res_x);
		res_y_2 = math.floor(num_drcs / res_x_2);
		error_2 = abs(1 - (aspect_ratio / (res_x_2 / res_y_2)));
		res_y_3 = math.floor(exact_res_y);
		res_x_3 = math.floor(num_drcs / res_y_3);
		error_3 = abs(1 - (aspect_ratio / (res_x_3 / res_y_3)));
		res_y_4 = math.ceil(exact_res_y);
		res_x_4 = math.floor(num_drcs / res_y_4);
		error_4 = abs(1 - (aspect_ratio / (res_x_4 / res_y_4)));

//		prinln!("char resolution 1: " + str(res_x_1) + "*" + str(res_y_1) + ", error: " + str(error_1) + "\n")
//		prinln!("char resolution 2: " + str(res_x_2) + "*" + str(res_y_2) + ", error: " + str(error_2) + "\n")
//		prinln!("char resolution 3: " + str(res_x_3) + "*" + str(res_y_3) + ", error: " + str(error_3) + "\n")
//		prinln!("char resolution 4: " + str(res_x_4) + "*" + str(res_y_4) + ", error: " + str(error_4) + "\n")

		res_x = res_x_1;
		res_y = res_y_1;
		error = error_1;
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

		prinln!("char resolution:   " + str(res_x) + "*" + str(res_y) + ", error: " + str(error) + "\n");

		// remove alpha
		if image.mode == "RGBA" || image.mode == "LA" {
			background = Image.new("RGB", image.size, (255, 255, 255));
			index = 3 if image.mode == "RGBA" else 1;
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
		p = image.getpalette();
		self.palette = [];
		for i in range(0, colors) {
			r = p[i * 3];
			g = p[i * 3 + 1];
			b = p[i * 3 + 2];
            self.palette.append("#{:02x}{:02x}{:02x}".format(r,g,b));
        }

//		prinln!("self.palette: " + pprint.pformat(self.palette) + "\n")

		// create drcs
		self.drcs = bytearray();

		if colors == 4 {
			num_bits = 2
        } elif colors == 16 {
            num_bits = 4
        }

		for base_y in range(0, res_y * 10, 10) {
			for base_x in range(0, res_x * 6, 6) {
				for bitno in range(0, num_bits) {
					self.drcs.extend([0x30 + bitno]);
					drcs_block = bytearray();
					for y in range(0, 10) {
						byte = 0;
						for x in range(0, 6) {
							byte <<= 1;
                            byte |= (image.getpixel((base_x + x, base_y + y)) >> bitno) & 1;
                        }
						byte |= 0x40;
                        drcs_block.append(byte);
                    }

					// compression
					drcs_block = Image_UI.compress(drcs_block)

//					prinln!("drcs_block: " + pprint.pformat(drcs_block) + "\n")
                    self.drcs.extend(drcs_block)
                }
            }
        }

		prinln!("DRCs compressed " + str(40 * res_x * res_y) + " down to " + str(len(self.drcs)) + "\n");

		drcs_header = bytearray();
		if colors == 4 {
			drcs_header.extend(b'\x1f\x23\x20\x4b\x42') // start defining 6x10 @ 4c
        } else if colors == 16 {
			drcs_header.extend(b'\x1f\x23\x20\x4b\x44') // start defining 6x10 @ 16c
        } else {
            error()
        }
		drcs_header.extend([0x1f, 0x23, drcs_start]);

		// prepend
		self.drcs[0:0] = drcs_header;

		// append
		if colors == 4 {
			// set colors to 16, 17, 18, 19
			self.drcs.extend(b'\x1f\x26\x20\x22\x20\x35\x40');
			self.drcs.extend(b'\x1f\x26\x30\x50');
			self.drcs.extend(b'\x1f\x26\x31\x51');
			self.drcs.extend(b'\x1f\x26\x32\x52');
			self.drcs.extend(b'\x1f\x26\x33\x53');

		// create characters to print
		if colors == 16 {
			step = 2
        } else {
            step = 1
        }
		self.chars = [];
		for y in range(0, res_y) {
			l = bytearray();
			for x in range(0, res_x) {
                l.append(drcs_start + (y * res_x + x) * step)
            }
            self.chars.append(l);
        }
    }

	fn create_image_page() -> Page {
//		filename = "/Users/mist/Desktop/RGB_24bits_palette_sample_image.jpg"
//		filename = "/Users/mist/Desktop/Lenna_(test_image).png"
//		filename = "/Users/mist/Desktop/Wikipedia_logo_593.jpg"
		filename = "/Users/mist/Desktop/220px-C64c_system.jpg";

		(palette, drcs, chars) = Image_UI.cept_from_image(filename);

		data_cept = bytearray();
		data_cept.extend(Cept.define_palette(palette));
		data_cept.extend(drcs);

		data_cept.extend(Cept.set_cursor(3, 1));
		data_cept.extend(Cept.load_g0_drcs());
		for l in chars {
			data_cept.extend(l);
            data_cept.extend(b'\r\n');
        }

		meta = {
			"clear_screen": True,
			"links": {
				"0": "0"
			},
			"publisher_color": 0
		};

        return Page { meta, data_cept };
    }
