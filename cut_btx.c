#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>
#include <stdlib.h>

int debug = 0;
int verbose = 0;
int create_files = 1;

void
print_hex(uint8_t *q, int c)
{
	for (int i = 0; i < c; i++) {
//		if (i && !(i & 15)) {
//			printf("\n");
//		}
		printf("%02x ", *q++);
	}
	printf("\n");
}

void
print_text(FILE *f, uint8_t *p, int c)
{
	for (int d = c - 1; d > 0; d--) {
		if (p[d] == ' ') {
			c--;
		} else {
			break;
		}
	}

	while (*p == ' ') {
		p++;
		c--;
	}

	uint8_t *q = p;
	fprintf(f, "\"");
	for (; q < p + c;) {
		if (*q == 0x19) {
			uint8_t ch = *++q;
			if (ch == 0x48) {
				switch (*++q) {
					case 'u':
						fprintf(f, "ü");
						break;
					case 'O':
						fprintf(f, "Ö");
						break;
					default:
						printf("ERROR: unknown encoding: 0x48 + 0x%02x\n", *q);
						exit(1);
				}
				q++;
			}
		} else {
			fprintf(f, "%c", *q++);
		}
	}
	fprintf(f, "\"");
}

uint8_t *
print_links(uint8_t *p)
{
	printf("{\n");
	uint8_t *q = p;
	bool first = true;
	for (;;) {
		if (q[0] != 0x1f || q[1] != 0x3d || (q[2] & 0xf0) != 0x30) {
			break;
		}
		if (!first) {
			printf(",\n");
		}

		q += 3;
		if (*q == 0x1f) {
			// empty
			continue;
		}
		first = false;
		printf("\t\"%c", q[0]);
		if (q[1] != ' ') {
			printf("%c", q[1]);
		}
		q += 2;
		printf("\": \"");
		while (*q != 0x1f) {
			printf("%c", *q++);
		}
		printf("\"");
	}
	printf("\n},\n");
	return q;
}

void
print_palette(FILE *f, uint8_t *p, int c)
{
	fprintf(f, "[\n");
	uint8_t *q = p;
	bool first = true;
	for (; q < p + c;) {
		if (!first) {
			fprintf(f, ",\n");
		}
		first = false;
		int r3 = (q[0] >> 5) & 1;
		int g3 = (q[0] >> 4) & 1;
		int b3 = (q[0] >> 3) & 1;
		int r2 = (q[0] >> 2) & 1;
		int g2 = (q[0] >> 1) & 1;
		int b2 = (q[0] >> 0) & 1;
		int r1 = (q[1] >> 5) & 1;
		int g1 = (q[1] >> 4) & 1;
		int b1 = (q[1] >> 3) & 1;
		int r0 = (q[1] >> 2) & 1;
		int g0 = (q[1] >> 1) & 1;
		int b0 = (q[1] >> 0) & 1;
		int r = (r0 | (r1 << 1) | (r2 << 2) | (r3 << 3)) << 4;
		int g = (g0 | (g1 << 1) | (g2 << 2) | (g3 << 3)) << 4;
		int b = (b0 | (b1 << 1) | (b2 << 2) | (b3 << 3)) << 4;
		fprintf(f, "\t\"#%02x%02x%02x\"", r, g, b);
		q += 2;
	}
	fprintf(f, "\n]\n");
}

int
main(int argc, char **argv)
{
	int found;
	uint8_t *p_old;

	if (debug) {
		verbose = 1;
	}

	char filename_globals[256];
	char filename_palette[256];
	char filename_include[256];
	char filename_payload[256];
	strcpy(filename_globals, argv[1]);
	strcpy(filename_globals + strlen(argv[1]), ".glob");
	strcpy(filename_palette, argv[1]);
	strcpy(filename_palette + strlen(argv[1]), ".pal");
	strcpy(filename_include, argv[1]);
	strcpy(filename_include + strlen(argv[1]), ".inc");
	strcpy(filename_payload, argv[1]);
	strcpy(filename_payload + strlen(argv[1]), ".cept");

	FILE *file_globals;
	if (create_files) {
		file_globals = fopen(filename_globals, "w");
		fprintf(file_globals, "{\n");
	}

	FILE *f = fopen(argv[1], "r");
	uint8_t buffer[10*1024];
	memset(buffer, 255, sizeof(buffer));
	int total_length = fread(buffer, 1, sizeof(buffer), f);
	fclose(f);

	uint8_t *p = buffer;

	// skip remote echo of previous user entry that
	// ended up in the dump
	while ((*p >= '0' && *p <= '9') || *p == '#' || *p == ' ' || *p == 8 || *p == 0xbe || *p == 0xff) {
		p++;
	}

	const uint8_t data1[] = { 0x14 };

	if (!memcmp(p, data1, sizeof(data1))) {
		if (debug) printf("HIDE_CURSOR detected.\n");
		p += sizeof(data1);
	} else {
		printf("ERROR: HIDE_CURSOR not detected.\n");
		print_hex(p, 32);
		return 1;
	}

	const uint8_t data2[] = {
		0x1f,0x2f,0x43,                          // serial limited mode
		0x0c,                                    // clear screen
	};

	if (!memcmp(p, data2, sizeof(data2))) {
		printf("\"clear_screen\": true,\n");
		p += sizeof(data2);
	} else {
		printf("\"clear_screen\": false,\n");
	}

	const uint8_t data2b[] = {
		0x1f,0x2f,0x40,0x58,                     // service break to row 24
		0x18,                                    // clear line
		0x53,0x65,0x69,0x74,0x65,0x20,0x77,0x69, // "Seite wi"
		0x72,0x64,0x20,0x61,0x75,0x66,0x67,0x65, // "rd aufge"
		0x62,0x61,0x75,0x74,0x20,0x20,0x20,0x20, // "baut    "
		0x20,0x20,0x20,0x20,0x20,0x20,0x20,0x20, // "        "
		0x20,0x20,0x20,                          // "   "
		0x98,                                    // hide
		0x08,                                    // cursor left
		0x53,0x48,0x32,0x39,0x31,                // "SH291"
		0x1f,0x2f,0x4f,                          // service break back
	};

again:

	if (!memcmp(p, data2b, sizeof(data2b))) {
		if (verbose) {
			printf("\"sh291\": true,\n");
		}
		p += sizeof(data2b);
	} else {
		if (verbose) {
			printf("\"sh291\": false,\n");
		}
	}

	const uint8_t data2c[] = {
		0x1f,0x26,0x20,                          // start defining colors
		0x1f,0x26,0x31,0x36,                     // define colors 16+
	};

	if (!memcmp(p, data2c, sizeof(data2c))) {
		if (debug) printf("INCLUDE1 detected.\n");
		p += sizeof(data2c);

		uint8_t *p_old = p;
		while (*p != 0x1f) {
			p++;
		}

		if (create_files) {
			f = fopen(filename_palette, "w");
			fprintf(f, "{\n\"palette\": ");
			print_palette(f, p_old, p - p_old);
			fprintf(f, "}\n");
			fclose(f);
		} else {
			printf("\"palette\": ");
			print_palette(stdout, p_old, p - p_old);
		}

		const uint8_t data3[] = {
			0x1f,0x41,0x41,                           // set cursor to x=1 y=1
		};

		if (!memcmp(p, data3, sizeof(data3))) {
			printf("\"set_cursor\": true,\n");
			p += sizeof(data3);
		} else {
			printf("\"set_cursor\": false,\n");
		}
	} else {
		if (debug) printf("INCLUDE1 not detected.\n");
	}

	const uint8_t data4[] = {
		0x1f,0x2f,0x43,                           // serial limited mode
		0x0c                                      // clear screen
	};

	const uint8_t data5[] = {
		0x1f,0x2d,                                // set resolution to 40x24
		0x1f,0x57,0x41,                           // set cursor to line 23, column 1
		0x9b,0x31,0x51,                           // unprotect line
		0x1b,0x23,0x21,0x4c,                      // set fg color of line to 12
		0x1f,0x2f,0x44,                           // parallel limited mode
		0x1f,0x58,0x41,                           // set cursor to line 24, column 1
		0x9b,0x31,0x51,                           // unprotect line
		0x20,                                     // " "
		0x08,                                     // cursor left
		0x18,                                     // clear line
		0x1e,                                     // cursor home
		0x9b,0x31,0x51,                           // unprotect line
		0x20,                                     // " "
		0x08,                                     // cursor left
		0x18,                                     // clear line
		0x1f,0x2f,0x43,                           // serial limited mode
		0x1f,0x58,0x41,                           // set cursor to line 24, column 1
		0x9b,0x31,0x40,                           // select palette #1
		0x80,                                     // set fg color to #0
		0x08,                                     // cursor left
		0x9d,                                     // Hintergrundfarbe setzen bzw. inverse Polarität
		0x08,                                     // cursor left
	};

	found = 0;
	p_old = p;
	do {

		if (!memcmp(p, data4, sizeof(data4))) {
			found = 1;
			break;
		}
		if (!memcmp(p, data5, sizeof(data5))) {
			found = 2;
			break;
		}
		p++;
	} while(p <= buffer + total_length - sizeof(data4));

	if (found && p != p_old) {
		if (verbose) {
			printf("include: ");
			print_hex(p_old, p - p_old);
		}
		if (create_files) {
			f = fopen(filename_include, "w");
			fwrite(p_old, 1, p - p_old, f);
			fclose(f);
		}
	}
	if (found == 1) {
		if (debug) printf("CLS detected.\n");
		p += sizeof(data4);

		if (!memcmp(p, data5, sizeof(data5))) {
			if (debug) printf("HEADER1 detected.\n");
			p += sizeof(data5);
		} else {
			printf("ERROR: HEADER1 not detected.\n");
			print_hex(p, 32);
			return 1;
		}
	} else if (found == 2) {
		if (debug) printf("HEADER1 detected.\n");
		p += sizeof(data5);
	} else {
		printf("ERROR: CLS/HEADER1 not detected.\n");
		print_hex(p, 32);
		return 1;
	}

	const uint8_t data5b[] = {
		0x1f,0x58,0x53,                           // set cursor to line 24, column 19
	};

	found = 0;
	p_old = p;
	do {

		if (!memcmp(p, data5b, sizeof(data5b))) {
			found = 1;
			break;
		}
		p++;
	} while(p <= buffer + total_length - sizeof(data5b));

	if (found) {
		if (p_old[0] == 0x9b && p_old[1] == 0x30 && p_old[2] == 0x40) {
			p_old += 3;
		}
		uint8_t color = p_old[0] & 0xf;

		printf("\"publisher_color\": ");
		if (debug) {
			print_hex(p_old, p - p_old);
		} else {
			printf("%d,\n", color);
		}

		if (debug) printf("HEADERX detected.\n");
		p += sizeof(data5b);
	} else {
		printf("ERROR: HEADERX not detected.\n");
		print_hex(p, 32);
		return 1;
	}

	if (verbose) {
		printf("\"page_number\": ");
		print_text(stdout, p, 22);
	printf(",\n");
	}
	p += 22;

	const uint8_t data6[] = {
		0x1e,                                     // cursor home
		0x9b,0x31,0x40,                           // select palette #1
		0x80,                                     // set fg color to #0
		0x08,                                     // cursor left
		0x9d,                                     // Hintergrundfarbe setzen bzw. inverse Polarität
		0x08,                                     // cursor left
	};
//		0x87,                                     // set fg color to #7
	const uint8_t data6b[] = {
		0x0d,                                     // cursor to beginning of line
	};

	if (!memcmp(p, data6, sizeof(data6))) {
		if (debug) printf("HEADER2 detected.\n");
		p += sizeof(data6);
	} else {
		printf("ERROR: HEADER2 not detected.\n");
		print_hex(p, 32);
		return 1;
	}

	found = 0;
	p_old = p;
	do {

		if (!memcmp(p, data6b, sizeof(data6b))) {
			found = 1;
			break;
		}
		p++;
	} while(p <= buffer + total_length - sizeof(data6b));

	if (found) {
		if (verbose) {
			printf("publisher_color_2: ");
			print_hex(p_old, p - p_old);
		}

		if (debug) printf("HEADERY detected.\n");
		p += sizeof(data6b);
	} else {
		printf("ERROR: HEADERY not detected.\n");
		print_hex(p, 32);
		return 1;
	}

	int i;
	for (i = 0; i < 35; i++) {
		if (p[i] == 0x1f) {
			break;
		}
	}

	if (create_files) {
		f = file_globals;
	} else {
		f = stdout;
	}
	fprintf(f, "\"publisher_name\": ");
	print_text(f, p, i);
	fprintf(f, "\n");
	p += i;

	const uint8_t data7[] = {
		0x1f,0x41,0x5f,                           // set cursor to line 1, column 31
	};

	if (!memcmp(p, data7, sizeof(data7))) {
		if (debug) printf("HEADER3 detected.\n");
		p += sizeof(data7);
	} else {
		if (debug) printf("HEADER3 not detected.\n");
	}

	if (verbose || memcmp(p, "   0,00 DM", 7)) {
		printf("\"price\": ");
		print_text(stdout, p, 10);
		printf(",\n");
	}
	p += 10;

	const uint8_t data8[] = {
		0x1e,                                     // cursor home
		0x9b,0x30,0x40,                           // select palette #0
		0x9b,0x31,0x50,                           // protect line
		0x0a,                                     // cursor down
	};

	if (!memcmp(p, data8, sizeof(data8))) {
		if (debug) printf("HEADER4 detected.\n");
		p += sizeof(data8);
	} else {
		printf("ERROR: HEADER4 not detected.\n");
		print_hex(p, 32);
		return 1;
	}

	printf("\"links\": ");
	p = print_links(p);

	const uint8_t data10[] = {
//		0x39,                                     // "9"
		0x1f,0x2f,0x43,                           // serial limited mode
		// same as data5
		0x1f,0x2d,                                // set resolution to 40x24
		0x1f,0x57,0x41,                           // set cursor to line 23, column 1
		0x9b,0x31,0x51,                           // unprotect line
		0x1b,0x23,0x21,0x4c,                      // set fg color of line to 12
		0x1f,0x2f,0x44,                           // parallel limited mode
		0x1f,0x58,0x41,                           // set cursor to line 24, column 1
		0x9b,0x31,0x51,                           // unprotect line
		0x20,                                     // " "
		0x08,                                     // cursor left
		0x18,                                     // clear line
		0x1e,                                     // cursor home
		0x9b,0x31,0x51,                           // unprotect line
		0x20,                                     // " "
		0x08,                                     // cursor left
		0x18,                                     // clear line
		0x1f,0x2f,0x43,                           // serial limited mode
		0x1f,0x58,0x41,                           // set cursor to line 24, column 1
		0x9b,0x31,0x40,                           // select palette #1
		0x80,                                     // set fg color to #0
		0x08,                                     // cursor left
		0x9d,                                     // Hintergrundfarbe setzen bzw. inverse Polarität
		0x08,                                     // cursor left
	};
//		0x87,                                     // set fg color to #7
	const uint8_t data10b[] = {
		0x1f,0x58,0x53,                           // set cursor to line 24, column 19
	};


	found = 0;
	p_old = p;
	do {

		if (!memcmp(p, data10, sizeof(data10))) {
			found = 1;
			break;
		}
		p++;
	} while(p <= buffer + total_length - sizeof(data10));

	if (found) {
		if (verbose) {
			printf("payload: ");
			print_hex(p_old, p - p_old);
		}
		if (create_files) {
			f = fopen(filename_payload, "w");
			fwrite(p_old, 1, p - p_old, f);
			fclose(f);
		}

		if (debug) printf("FOOTER1 detected.\n");
		p += sizeof(data10);
	} else {
		printf("ERROR: FOOTER1 not detected.\n");
		print_hex(p, 32);
		return 1;
	}

	found = 0;
	p_old = p;
	do {

		if (!memcmp(p, data10b, sizeof(data10b))) {
			found = 1;
			break;
		}
		p++;
	} while(p <= buffer + total_length - sizeof(data10b));

	if (found) {
		if (verbose) {
			printf("publisher_color_3: ");
			print_hex(p_old, p - p_old);
		}

		if (debug) printf("FOOTERX detected.\n");
		p += sizeof(data10b);
	} else {
		printf("ERROR: FOOTERX not detected.\n");
		print_hex(p, 32);
		return 1;
	}

	if (verbose) {
		printf("page_number_2: ");
		print_text(stdout, p, 22);
	}
	p += 22;

	if (!memcmp(p, data6, sizeof(data6))) {
		if (debug) printf("FOOTER2 detected.\n");
		p += sizeof(data6);
	} else {
		printf("ERROR: FOOTER2 not detected.\n");
		print_hex(p, 32);
		return 1;
	}

	found = 0;
	p_old = p;
	do {

		if (!memcmp(p, data6b, sizeof(data6b))) {
			found = 1;
			break;
		}
		p++;
	} while(p <= buffer + total_length - sizeof(data6b));

	if (found) {
		if (verbose) {
			printf("publisher_color_4: ");
			print_hex(p_old, p - p_old);
		}

		if (debug) printf("FOOTERY detected.\n");
		p += sizeof(data6b);
	} else {
		printf("ERROR: FOOTERY not detected.\n");
		print_hex(p, 32);
		return 1;
	}

	found = 0;
	p_old = p;
	do {

		if (!memcmp(p, data7, sizeof(data7))) {
			found = 1;
			break;
		}
		p++;
	} while(p <= buffer + total_length - sizeof(data7));

	if (found) {
		if (verbose) {
			printf("publisher_name_2: ");
			print_text(stdout, p_old, p - p_old);
		}

		if (debug) printf("FOOTER3 detected.\n");
		p += sizeof(data7);
	} else {
		printf("ERROR: FOOTER3 not detected.\n");
		print_hex(p, 32);
		return 1;
	}

	if (verbose) {
		printf("price_2: ");
		print_text(stdout, p, 10);
	}
	p += 10;

	const uint8_t data11[] = {
		0x1e,                                     // cursor home
		0x9b,0x30,0x40,                           // select palette #0
		0x9b,0x31,0x50,                           // protect line
		0x0a,                                     // cursor down
	};

	const uint8_t data11b[] = {
		0x1f,0x58,0x41,                           // set cursor to x=24 y=1
		0x11,                                     // show cursor
		0x1a,                                     // end of page
	};

	if (!memcmp(p, data11, sizeof(data11))) {
		if (debug) printf("FOOTER4 detected.\n");
		p += sizeof(data11);
	} else {
		printf("ERROR: FOOTER4 not detected.\n");
		print_hex(p, 32);
		return 1;
	}

	if (!memcmp(p, data2b, sizeof(data2))) {
		printf("again: yes\n");
		goto again;
	}

	if (!memcmp(p, data5, sizeof(data5))) {
		printf("again2: yes\n");
		goto again;
	}

	if (!memcmp(p, data11b, sizeof(data11b))) {
		if (debug) printf("FOOTER4 detected.\n");
		p += sizeof(data11b);
	} else {
		printf("ERROR: FOOTER5 not detected.\n");
		print_hex(p, 32);
		return 1;
	}

	if (p != buffer + total_length) {
		if (debug) {
			printf("WARNING: trailing bytes!\n");
			print_hex(p, 32);
		}
	}

	if (debug) printf("OK!\n");

	if (create_files) {
		fprintf(file_globals, "}\n");
	}

	return 0;
}
