#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>

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
		printf("INCLUDE detected.\n");
		p += sizeof(data2);
	}
	uint8_t *q = p;
	for (int i = 0; i < 16; i++) {
		printf("%02x ", *q++);
	}
	printf("\n");

	return 0;
}
