#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>

void
print_hex(uint8_t *q, int c)
{
	for (int i = 0; i < c; i++) {
		if (i && !(i & 15)) {
			printf("\n");
		}
		printf("%02x ", *q++);
	}
	printf("\n");
}

int
main(int argc, char **argv)
{
	FILE *f = fopen(argv[1], "r");
	uint8_t buffer[10*1024];
	int total_length = fread(buffer, 1, sizeof(buffer), f);

	uint8_t *p = buffer;

	// skip remote echo of previous user entry that
	// ended up in the dump
	while ((*p >= '0' && *p <= '9') || *p == '#') {
		p++;
	}

	const uint8_t data1[] = { 0x14 };

	if (!memcmp(p, data1, sizeof(data1))) {
		printf("HIDE_CURSOR detected.\n");
		p += sizeof(data1);
	} else {
		printf("HIDE_CURSOR not detected.\n");
		return 1;
	}

	const uint8_t data2[] = {
		0x1f,0x2f,0x43,                          // serial limited mode
		0x0c,                                    // clear screen
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
		0x1f,0x26,0x20,                          // start defining colors
		0x1f,0x26,0x31,0x36,                     // define colors 16+
	};

	if (!memcmp(p, data2, sizeof(data2))) {
		printf("INCLUDE1 detected.\n");
		p += sizeof(data2);

		printf("palette definitions:\n");
		print_hex(p, 32);
		p += 32;

		const uint8_t data3[] = {
			0x1f,0x41,0x41,                           // set cursor to x=1 y=1
		};

		if (!memcmp(p, data3, sizeof(data3))) {
			printf("INCLUDE2 detected.\n");
			p += sizeof(data3);
		} else {
			printf("INCLUDE2 not detected.\n");
			return 1;
		}
	} else {
		printf("INCLUDE1 not detected.\n");
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
		0x87,                                     // set fg color to #7
		0x1f,0x58,0x53,                           // set cursor to line 24, column 19
	};

	int found = 0;
	uint8_t *p_old = p;
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

	if (found) {
		printf("include:\n");
		print_hex(p_old, p - p_old);
	}
	if (found == 1) {
		printf("CLS detected.\n");
		p += sizeof(data4);

		if (!memcmp(p, data5, sizeof(data5))) {
			printf("HEADER1 detected.\n");
			p += sizeof(data5);
		} else {
			printf("HEADER1 not detected.\n");
			return 1;
		}
	} else if (found == 2) {
		printf("HEADER1 detected.\n");
		p += sizeof(data5);
	} else {
		printf("CLS/HEADER1 not detected.\n");
		return 1;
	}

	printf("page number:\n");
	print_hex(p, 22);
	p += 22;

	const uint8_t data6[] = {
		0x1e,                                     // cursor home
		0x9b,0x31,0x40,                           // select palette #1
		0x80,                                     // set fg color to #0
		0x08,                                     // cursor left
		0x9d,                                     // Hintergrundfarbe setzen bzw. inverse Polarität
		0x08,                                     // cursor left
		0x87,                                     // set fg color to #7
		0x0d,                                     // cursor to beginning of line
	};

	if (!memcmp(p, data6, sizeof(data6))) {
		printf("HEADER2 detected.\n");
		p += sizeof(data6);
	} else {
		printf("HEADER2 not detected.\n");
		return 1;
	}

	printf("publisher:\n");
	print_hex(p, 30);
	p += 30;

	const uint8_t data7[] = {
		0x1f,0x41,0x5f,                           // set cursor to line 1, column 31
	};

	if (!memcmp(p, data7, sizeof(data7))) {
		printf("HEADER3 detected.\n");
		p += sizeof(data7);
	} else {
		printf("HEADER3 not detected.\n");
		return 1;
	}

	printf("price:\n");
	print_hex(p, 10);
	p += 10;

	const uint8_t data8[] = {
		0x1e,                                     // cursor home
		0x9b,0x30,0x40,                           // select palette #0
		0x9b,0x31,0x50,                           // protect line
		0x0a,                                     // cursor down
	};

	if (!memcmp(p, data8, sizeof(data8))) {
		printf("HEADER4 detected.\n");
		p += sizeof(data8);
	} else {
		printf("HEADER4 not detected.\n");
		return 1;
	}

	const uint8_t data9[] = {
		0x1f,0x2f,0x44,                           // parallel limited mode
		0x1f,                                     // set cursor to line X, column X
	};

	found = 0;
	p_old = p;
	do {

		if (!memcmp(p, data9, sizeof(data9))) {
			found = 1;
			break;
		}
		p++;
	} while(p <= buffer + total_length - sizeof(data9));

	if (found) {
		printf("links:\n");
		print_hex(p_old, p - p_old);

		printf("HEADER5 detected.\n");
		p += sizeof(data4);

	} else {
		printf("HEADER5 not detected.\n");
		return 1;
	}

	printf("cursor:\n");
	print_hex(p, 2);
	p += 2;

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
		0x87,                                     // set fg color to #7
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
		printf("payload:\n");
		print_hex(p_old, p - p_old);

		printf("FOOTER1 detected.\n");
		p += sizeof(data10);

	} else {
		printf("FOOTER1 not detected.\n");
		return 1;
	}

	printf("page number:\n");
	print_hex(p, 22);
	p += 22;

	if (!memcmp(p, data6, sizeof(data6))) {
		printf("FOOTER2 detected.\n");
		p += sizeof(data6);
	} else {
		printf("FOOTER2 not detected.\n");
		return 1;
	}

	printf("publisher:\n");
	print_hex(p, 30);
	p += 30;

	if (!memcmp(p, data7, sizeof(data7))) {
		printf("FOOTER3 detected.\n");
		p += sizeof(data7);
	} else {
		printf("FOOTER3 not detected.\n");
		return 1;
	}

	printf("price:\n");
	print_hex(p, 10);
	p += 10;

	const uint8_t data11[] = {
		0x1e,                                     // cursor home
		0x9b,0x30,0x40,                           // select palette #0
		0x9b,0x31,0x50,                           // protect line
		0x0a,                                     // cursor down
		0x1f,0x58,0x41,                           // set cursor to x=24 y=1
		0x11,                                     // show cursor
		0x1a,                                     // end of page
	};


	if (!memcmp(p, data11, sizeof(data11))) {
		printf("FOOTER4 detected.\n");
		p += sizeof(data11);
	} else {
		printf("FOOTER4 not detected.\n");
		return 1;
	}

	if (p != buffer + total_length) {
		printf("trailing bytes!\n");
		return 1;
	}

	printf("OK!\n");

	return 0;
}
