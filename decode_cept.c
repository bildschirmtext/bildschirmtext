#include <stdio.h>
#include <stdint.h>

#define MIN(a,b) (((a)<(b))?(a):(b))
#define MAX(a,b) (((a)>(b))?(a):(b))

#define HEX_PER_LINE 16

int
main(int argc, char **argv)
{
	FILE *f = fopen("/Users/mist/Documents/btx/corpus/20095a", "r");
	uint8_t buffer[10*1024];
	int total_length = fread(buffer, 1, sizeof(buffer), f);

	uint8_t *p = buffer;
	while (p < buffer + total_length) {
		int l = 1;
		char *d = "";
		char tmpstr[80];
		if (*p == 0x08) {
			d = "backspace";
		} else if (*p == 0x0a) {
			d = "down";
		} else if (*p == 0x0c) {
			d = "clear screen";
		} else if (*p == 0x14) {
			d = "home";
		} else if (*p == 0x18) {
			d = "clear line";
		} else if (p[0] == 0x1B && p[1] == 0x23 && p[2] == 0x20 && (p[3] & 0xF0) == 0x50) {
			l = 4;
			snprintf(tmpstr, sizeof(tmpstr), "set bg color of screen to %d", p[3] - 0x50);
			d = tmpstr;
		} else if (p[0] == 0x1F && p[1] == 0x26 && p[2] == 0x20) {
			l = 3;
			d = "start defining colors";
		} else if (p[0] == 0x1F && p[1] == 0x26 && (p[2] & 0xF0) == 0x30 && (p[3] & 0xF0) == 0x30) {
			l = 4;
			snprintf(tmpstr, sizeof(tmpstr), "define colors %c%c+", p[2], p[3]);
			d = tmpstr;
			uint8_t *q = p + 4;
			while (*q != 0x1f) {
				q++;
				l++;
			}
		} else if (p[0] == 0x1F && p[1] == 0x2F && p[2] == 0x40) {
			l = 4;
			snprintf(tmpstr, sizeof(tmpstr), "service break to row %d", p[3] - 0x40);
			d = tmpstr;
		} else if (p[0] == 0x1F && p[1] == 0x2F && p[2] == 0x4f) {
			l = 3;
			d = "service break back";
		} else if (p[0] == 0x1F && p[1] == 0x2F && p[2] == 0x43) {
			l = 3;
			d = "serial limited mode";
		} else if (p[0] == 0x1F && p[1] >= 0x41 && p[2] >= 0x41) {
			l = 3;
			snprintf(tmpstr, sizeof(tmpstr), "set cursor to x=%d y=%d", p[1] - 0x40, p[2] - 0x40);
			d = tmpstr;
		} else if (p[0] >= 0x20 && p[0] <= 0x7F) {
			uint8_t *q = p;
			l = 0;
			tmpstr[0] = '"';
			while (l < HEX_PER_LINE && q[0] >= 0x20 && q[0] <= 0x7F) {
				tmpstr[l++ + 1] = *q;
				q++;
			}
			tmpstr[l + 1] = '"';
			tmpstr[l + 2] = 0;
			d = tmpstr;
		} else if (*p >= 0x80 && *p <= 0x87) {
			snprintf(tmpstr, sizeof(tmpstr), "set fg color to #%d", p[0] - 0x80);
			d = tmpstr;
		} else if (*p >= 0x90 && *p <= 0x97) {
			snprintf(tmpstr, sizeof(tmpstr), "set bg color to #%d", p[0] - 0x90);
			d = tmpstr;
		} else if (*p == 0x98) {
			d = "hide";
		} else if (p[0] == 0x9B && p[1] == 0x30 && p[2] == 0x40) {
			l = 3;
			d = "select palette #0";
		} else if (p[0] == 0x9B && p[1] == 0x31 && p[2] == 0x40) {
			l = 3;
			d = "select palette #1";
		} else if (p[0] == 0x9B && p[1] == 0x32 && p[2] == 0x40) {
			l = 3;
			d = "select palette #2";
		} else if (p[0] == 0x9B && p[1] == 0x33 && p[2] == 0x40) {
			l = 3;
			d = "select palette #3";
		} else {
			d = "unknown";
		}

		while (l > 0) {
			int ll = MIN(l, HEX_PER_LINE);

			for (int i = 0; i < ll; i++) {
				printf("%02x ", *p++);
			}
			for (int i = 0; i < 3*(HEX_PER_LINE-ll); i++) {
				printf(" ");
			}
			if (d) {
				printf("# %s\n", d);
			} else {
				printf("\n");
			}
			d = 0;
			l -= ll;
		}
	}

	return 0;
}
