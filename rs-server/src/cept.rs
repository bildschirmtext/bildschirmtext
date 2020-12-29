pub struct Cept {
    data: Vec<u8>,
    mode: i32,
    // charset: Option<i32>,
}


impl Cept {
    pub fn new() -> Self {
        Self {
            data: vec!(),
            mode: 0,
        }
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    fn g2code(&mut self, s: &[u8]) {
        debug_assert!(s.len() != 0);
        debug_assert!(s.len() <= 2);
        if self.mode == 0 {
            self.data.push(0x19);
            self.data.push(s[0]);
        } else {
            self.data.push(s[0] | 0x80);
        }
        if s.len() > 1 {
            self.data.push(s[1]);
        }
    }

	pub fn add_str(&mut self, s_in: &str) {
        for c in s_in.chars() {
            match c {
                '¤' => self.data.push(b'$'),         // $ and ¤ are swapped
                '$' => self.g2code(b"$"), // $ and ¤ are swapped
    			// '¦' => b"?",      // not available
    			// '¨' => b"?",      // not available
                '©' => self.g2code(b"S"),
    			// "ª" => b"?",      // not available
                '\u{00ad}' => {},    // soft hyphen
                '®' => self.g2code(b"R"),
    			// '¯' => b"?",      // not available
    			// '´' => b"?",      // not available
    			// '¸' => b"?",      // not available
    			// '¹' => b"?",      // not available
    			// 'º' => b"?",      // not available
                'À' => self.g2code(b"AA"),
                'Á' => self.g2code(b"BA"),
                'Â' => self.g2code(b"CA"),
                'Ã' => self.g2code(b"DA"),
                'Ä' => self.g2code(b"HA"),
                'Å' => self.g2code(b"JA"),
                'Æ' => self.g2code(b"a"),
                'Ç' => self.g2code(b"KC"),
                'È' => self.g2code(b"AE"),
                'É' => self.g2code(b"BE"),
                'Ê' => self.g2code(b"CE"),
                'Ë' => self.g2code(b"HE"),
                'Ì' => self.g2code(b"AI"),
                'Í' => self.g2code(b"BI"),
                'Î' => self.g2code(b"CI"),
                'Ï' => self.g2code(b"HI"),
                'Ð' => self.g2code(b"b"),
                'Ñ' => self.g2code(b"DN"),
                'Ò' => self.g2code(b"AO"),
                'Ó' => self.g2code(b"BO"),
                'Ô' => self.g2code(b"CO"),
                'Õ' => self.g2code(b"DO"),
                'Ö' => self.g2code(b"HO"),
                '×' => self.g2code(b"4"),
                'Ø' => self.g2code(b"i"),
                'Ù' => self.g2code(b"AU"),
                'Ú' => self.g2code(b"BU"),
                'Û' => self.g2code(b"CU"),
                'Ü' => self.g2code(b"HU"),
                'Ý' => self.g2code(b"BY"),
                'Þ' => self.g2code(b"l"),
                'ß' => self.g2code(b"{"),
                'à' => self.g2code(b"Aa"),
                'á' => self.g2code(b"Ba"),
                'â' => self.g2code(b"Ca"),
                'ã' => self.g2code(b"Da"),
                'ä' => self.g2code(b"Ha"),
                'å' => self.g2code(b"Ja"),
                'æ' => self.g2code(b"q"),
                'ç' => self.g2code(b"Kc"),
                'è' => self.g2code(b"Ae"),
                'é' => self.g2code(b"Be"),
                'ê' => self.g2code(b"Ce"),
                'ë' => self.g2code(b"He"),
                'ì' => self.g2code(b"Ai"),
                'í' => self.g2code(b"Bi"),
                'î' => self.g2code(b"Ci"),
                'ï' => self.g2code(b"Hi"),
                'ð' => self.g2code(b"s"),
                'ñ' => self.g2code(b"Dn"),
                'ò' => self.g2code(b"Ao"),
                'ó' => self.g2code(b"Bo"),
                'ô' => self.g2code(b"Co"),
                'õ' => self.g2code(b"Do"),
                'ö' => self.g2code(b"Ho"),
                '÷' => self.g2code(b"8"),
                'ø' => self.g2code(b"u"),
                'ù' => self.g2code(b"Au"),
                'ú' => self.g2code(b"Bu"),
                'û' => self.g2code(b"Cu"),
                'ü' => self.g2code(b"Hu"),
                'ý' => self.g2code(b"Ay"),
                'þ' => self.g2code(b"|"),
                'ÿ' => self.g2code(b"Hy"),

                // arrows
                '←' => self.g2code(b","),
                '↑' => self.g2code(b"-"),
                '→' => self.g2code(b"."),
                '↓' => self.g2code(b"/"),

                // math
                '⋅' => self.g2code(b"7"),

                // line feed
                '\n' => self.data.extend(b"\r\n"),

                // latin other
                'š' => self.g2code(b"Os"),
                'Œ' => self.g2code(b"j"),
                'œ' => self.g2code(b"z"),
                'ł' => self.g2code(b"x"),
                'č' => self.g2code(b"Oc"),
                'ć' => self.g2code(b"Bc"),

                // greek
                'ŋ' => self.g2code(b"\x7e"),
                'μ' => self.g2code(b"5"),
                'Ω' => self.g2code(b"`"),

                // punctuation
                '‚' => self.g2code(b")"),
                '’' => self.g2code(b"9"),
                '‘' => self.g2code(b"9"),
                '„' => self.g2code(b"*"),
                '“' => self.g2code(b":"),
                '″' => self.g2code(b":"),
                '”' => self.g2code(b":"),
                '–' => self.g2code(b"P"),

                // look-alikes
                '†' => self.data.push(b'+'),
                '−' => self.data.push(b'-'), // MINUS SIGN
                '⟨' => self.data.push(b'<'),
                '⟩' => self.data.push(b'>'),
                '∗' => self.data.push(b'*'),
                '‐' => self.data.push(b'-'),
                '—' => self.data.push(b'-'),

                // spaces
                ' ' => self.data.push(b' '), // NARROW NO-BREAK SPACE
                ' ' => self.data.push(b' '), // THIN SPACE
                ' ' => self.data.push(b' '), // ZERO WIDTH SPACE
                ' ' => self.data.push(b' '), // EN SPACE

                // used in phonetic alphabet
                'ˈ' => self.data.push(b'\''),
                'ː' => self.data.push(b':'),

                // XXX these change the length!!
                '€' => self.data.extend(b"EUR"),
                '…' => self.data.extend(b"..."),

                // ASCII
                #[allow(overlapping_patterns)]
                ' '..='~' => self.data.push(c as u8),

                _ => {
                    // sys.stderr.write("unknown character: '" + c + "' (" + hex(ord(c)) + ")n '" + s_in + "'\n")
                    // if charset:
                        // data_cept = charset.get(c)
                        // if data_cept:
                            // s2.extend(data_cept)
                        // else:
                            // s2.append(ord('?'))
                    // else:
                    self.data.push(b'?');
                },
            };
        }
    }
}