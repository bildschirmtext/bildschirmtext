#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>

#define MIN(a,b) (((a)<(b))?(a):(b))
#define MAX(a,b) (((a)>(b))?(a):(b))

#define HEX_PER_LINE 16

static bool
is_printable(unsigned char c)
{
	return (c >= 0x20 && c <= 0x7F) || (c >= 0xa0);
}

int
main(int argc, char **argv)
{
	FILE *f = fopen(argv[1], "r");
	uint8_t buffer[10*1024];
	int total_length = fread(buffer, 1, sizeof(buffer), f);

	uint8_t *p = buffer;
	while (p < buffer + total_length) {
		int l = 1;
		char *d = "";
		char tmpstr[80];
		if (is_printable(p[0]) && p[1] == 0x12 && p[2] >= 0x41) {
			l = 3;
			snprintf(tmpstr, sizeof(tmpstr), "repeat '%c' %d times", p[0] < 0x80 ? p[0] : '.', p[2] - 0x40);
			d = tmpstr;
		} else if (*p == 0x08) {
			d = "cursor left";
		} else if (*p == 0x09) {
			d = "cursor right";
		} else if (*p == 0x0a) {
			d = "cursor down";
		} else if (*p == 0x0b) {
			d = "cursor up";
		} else if (*p == 0x0c) {
			d = "clear screen";
		} else if (*p == 0x0d) {
			d = "cursor to beginning of line";
		} else if (*p == 0x0e) {
			d = "G1 into left charset";
		} else if (*p == 0x0f) {
			d = "G0 into left charset";
		} else if (*p == 0x11) {
			d = "show cursor";
		} else if (*p == 0x14) {
			d = "hide cursor";
		} else if (*p == 0x18) {
			d = "clear line";
		} else if (*p == 0x19) {
			d = "switch to G2 for one character";
		} else if (*p == 0x1a) {
			d = "end of page";
		} else if (*p == 0x1d) {
			d = "switch to G3 for one character";
		} else if (*p == 0x1e) {
			d = "cursor home";
		} else if (p[0] == 0x1B && p[1] == 0x22 && p[2] == 0x40) {
			l = 3;
			d = "serial mode";
		} else if (p[0] == 0x1B && p[1] == 0x22 && p[2] == 0x41) {
			l = 3;
			d = "parallel mode";
//		} else if (p[0] == 0x1B && p[1] == 0x23 && p[2] == 0x20 && (p[3] & 0xF0) == 0x40) {
//			l = 4;
//			snprintf(tmpstr, sizeof(tmpstr), "set fg color of screen to %d", p[3] - 0x40);
//			d = tmpstr;
		} else if (p[0] == 0x1B && p[1] == 0x23 && p[2] == 0x20 && (p[3] & 0xF0) == 0x50) {
			l = 4;
			snprintf(tmpstr, sizeof(tmpstr), "set bg color of screen to %d", p[3] - 0x50);
			d = tmpstr;
		} else if (p[0] == 0x1B && p[1] == 0x23 && p[2] == 0x21 && (p[3] & 0xF0) == 0x40) {
			l = 4;
			snprintf(tmpstr, sizeof(tmpstr), "set fg color of line to %d", p[3] - 0x40);
			d = tmpstr;
		} else if (p[0] == 0x1B && p[1] == 0x23 && p[2] == 0x21 && (p[3] & 0xF0) == 0x50) {
			l = 4;
			snprintf(tmpstr, sizeof(tmpstr), "set bg color of line to %d", p[3] - 0x50);
			d = tmpstr;
		} else if (p[0] == 0x1B && p[1] == 0x28 && p[2] == 0x20 && p[3] == 0x40) {
			l = 4;
			d = "load DRCs into G0";
		} else if (p[0] == 0x1B && p[1] == 0x28 && p[2] == 0x40) {
			l = 3;
			d = "load G0 into G0";
		} else if (p[0] == 0x1B && p[1] == 0x28 && p[2] == 0x62) {
			l = 3;
			d = "load G2 into G0";
		} else if (p[0] == 0x1B && p[1] == 0x28 && p[2] == 0x63) {
			l = 3;
			d = "load G1 into G0";
		} else if (p[0] == 0x1B && p[1] == 0x28 && p[2] == 0x64) {
			l = 3;
			d = "load G3 into G0";
		} else if (p[0] == 0x1B && p[1] == 0x29 && p[2] == 0x20 && p[3] == 0x40) {
			l = 4;
			d = "load DRCs into G1";
		} else if (p[0] == 0x1B && p[1] == 0x29 && p[2] == 0x40) {
			l = 3;
			d = "load G0 into G1";
		} else if (p[0] == 0x1B && p[1] == 0x29 && p[2] == 0x62) {
			l = 3;
			d = "load G2 into G1";
		} else if (p[0] == 0x1B && p[1] == 0x29 && p[2] == 0x63) {
			l = 3;
			d = "load G1 into G1";
		} else if (p[0] == 0x1B && p[1] == 0x29 && p[2] == 0x64) {
			l = 3;
			d = "load G3 into G1";
		} else if (p[0] == 0x1B && p[1] == 0x2A && p[2] == 0x20 && p[3] == 0x40) {
			l = 4;
			d = "load DRCs into G2";
		} else if (p[0] == 0x1B && p[1] == 0x2A && p[2] == 0x40) {
			l = 3;
			d = "load G0 into G2";
		} else if (p[0] == 0x1B && p[1] == 0x2A && p[2] == 0x62) {
			l = 3;
			d = "load G2 into G2";
		} else if (p[0] == 0x1B && p[1] == 0x2A && p[2] == 0x63) {
			l = 3;
			d = "load G1 into G2";
		} else if (p[0] == 0x1B && p[1] == 0x2A && p[2] == 0x64) {
			l = 3;
			d = "load G3 into G2";
		} else if (p[0] == 0x1B && p[1] == 0x2B && p[2] == 0x20 && p[3] == 0x40) {
			l = 4;
			d = "load DRCs into G3";
		} else if (p[0] == 0x1B && p[1] == 0x2B && p[2] == 0x40) {
			l = 3;
			d = "load G0 into G3";
		} else if (p[0] == 0x1B && p[1] == 0x2B && p[2] == 0x62) {
			l = 3;
			d = "load G2 into G3";
		} else if (p[0] == 0x1B && p[1] == 0x2B && p[2] == 0x63) {
			l = 3;
			d = "load G1 into G3";
		} else if (p[0] == 0x1B && p[1] == 0x2B && p[2] == 0x64) {
			l = 3;
			d = "load G3 into G3";
		} else if (p[0] == 0x1B && p[1] == 0x6E) {
			l = 2;
			d = "G2 into left charset";
		} else if (p[0] == 0x1B && p[1] == 0x6F) {
			l = 2;
			d = "G3 into left charset";
		} else if (p[0] == 0x1B && p[1] == 0x7C) {
			l = 2;
			d = "G3 into right charset";
		} else if (p[0] == 0x1B && p[1] == 0x7D) {
			l = 2;
			d = "G2 into right charset";
		} else if (p[0] == 0x1B && p[1] == 0x7E) {
			l = 2;
			d = "G1 into right charset";
		} else if (p[0] == 0x1F && p[1] == 0x23 && p[2] == 0x20 && (p[3] & 0xF0) == 0x40 && (p[4] & 0xF0) == 0x40) {
			l = 5;
			char *res;
			switch (p[3] - 0x40) {
				case 6: res = "12x12"; break;
				case 7: res = "12x10"; break;
				case 0xA: res = "6x12"; break;
				case 0xB: res = "6x10"; break;
				case 0xC: res = "6x5"; break;
				case 0xF: res = "6x6"; break;
				default: res = "???"; break;
			}
			int col;
			switch (p[4] - 0x40) {
				case 1: col = 2; break;
				case 2: col = 4; break;
				case 4: col = 16; break;
				default: col = -1; break;
			}
			snprintf(tmpstr, sizeof(tmpstr), "define characters %s, %d colors", res, col);
			d = tmpstr;
		} else if (p[0] == 0x1F && p[1] == 0x23 && p[3] == 0x30) {
			l = 4;
			snprintf(tmpstr, sizeof(tmpstr), "define characters 0x%02x+", p[2]);
			d = tmpstr;
			uint8_t *q = p + 4;
			while (*q != 0x1f) {
				q++;
				l++;
			}
		} else if (p[0] == 0x1F && p[1] == 0x26 && p[2] == 0x20 && p[3] == 0x22 && p[4] == 0x20 && p[5] == 0x35 && p[6] == 0x40) {
			l = 7;
			d = "start defining colors for DRCs";
		} else if (p[0] == 0x1F && p[1] == 0x26 && p[2] == 0x20) {
			l = 3;
			d = "start defining colors";
		} else if (p[0] == 0x1F && p[1] == 0x26 && p[2] == 0x21) {
			l = 3;
			d = "reset palette";
		} else if (p[0] == 0x1F && p[1] == 0x26 && (p[2] & 0xF0) == 0x30 && (p[3] & 0xF0) == 0x30) {
			l = 4;
			snprintf(tmpstr, sizeof(tmpstr), "define colors %c%c+", p[2], p[3]);
			d = tmpstr;
			uint8_t *q = p + 4;
			while (*q != 0x1f) {
				q++;
				l++;
			}
		} else if (p[0] == 0x1F && p[1] == 0x26 && (p[2] & 0xF0) == 0x30) {
			l = 3;
			snprintf(tmpstr, sizeof(tmpstr), "define DRC color %i", p[2] - 0x30);
			d = tmpstr;
			uint8_t *q = p + 3;
			while (*q != 0x1f) {
				q++;
				l++;
			}
		} else if (p[0] == 0x1F && p[1] == 0x2D) {
			l = 2;
			d = "set resolution to 40x24";
		} else if (p[0] == 0x1F && p[1] == 0x2F && p[2] == 0x40) {
			l = 4;
			snprintf(tmpstr, sizeof(tmpstr), "service break to row %d", p[3] - 0x40);
			d = tmpstr;
		} else if (p[0] == 0x1F && p[1] == 0x2F && p[2] == 0x4f) {
			l = 3;
			d = "service break back";
		} else if (p[0] == 0x1F && p[1] == 0x2F && p[2] == 0x41) {
			l = 3;
			d = "serial mode";
		} else if (p[0] == 0x1F && p[1] == 0x2F && p[2] == 0x42) {
			l = 3;
			d = "parallel mode";
		} else if (p[0] == 0x1F && p[1] == 0x2F && p[2] == 0x43) {
			l = 3;
			d = "serial limited mode";
		} else if (p[0] == 0x1F && p[1] == 0x2F && p[2] == 0x44) {
			l = 3;
			d = "parallel limited mode";
		} else if (p[0] == 0x1F && p[1] == 0x3D && (p[2] & 0xF0) >= 0x30) {
			if (p[3] == 0x1f){
				l = 3;
				snprintf(tmpstr, sizeof(tmpstr), "define shortcut #%d", p[2] - 0x30);
				d = tmpstr;
			} else {
				l = 5;
				snprintf(tmpstr, sizeof(tmpstr), "define shortcut #%d \"%c%c\"", p[2] - 0x30, p[3], p[4]);
				d = tmpstr;
			}
		} else if (p[0] == 0x1F && p[1] >= 0x41 && p[2] >= 0x41) {
			l = 3;
			snprintf(tmpstr, sizeof(tmpstr), "set cursor to line %d, column %d", p[1] - 0x40, p[2] - 0x40);
			d = tmpstr;
		} else if (is_printable(p[0]) && p[1] != 0x12) {
			uint8_t *q = p;
			l = 0;
			tmpstr[0] = '"';
			while (l < HEX_PER_LINE && is_printable(q[0]) && q[1] != 0x12) {
				tmpstr[l++ + 1] = *q < 0x80 ? *q : '.';
				q++;
			}
			tmpstr[l + 1] = '"';
			tmpstr[l + 2] = 0;
			d = tmpstr;
		} else if (*p >= 0x80 && *p <= 0x87) {
			snprintf(tmpstr, sizeof(tmpstr), "set fg color to #%d", p[0] - 0x80);
			d = tmpstr;
		} else if (*p == 0x88) {
			d = "blink on";
		} else if (*p == 0x89) {
			d = "blink off";
		} else if (*p == 0x8a) {
			d = "transparency on";
		} else if (*p == 0x8b) {
			d = "transparency off";
		} else if (*p == 0x8c) {
			d = "normal size";
		} else if (*p == 0x8d) {
			d = "double height";
		} else if (*p == 0x8e) {
			d = "double width";
		} else if (*p == 0x8f) {
			d = "double width and height";
		} else if (*p >= 0x90 && *p <= 0x97) {
			snprintf(tmpstr, sizeof(tmpstr), "set bg color to #%d", p[0] - 0x90);
			d = tmpstr;
		} else if (*p == 0x98) {
			d = "hide";
		} else if (*p == 0x99) {
			d = "underline off";
		} else if (*p == 0x9a) {
			d = "underline on";
		} else if (p[0] == 0x9B && p[1] == 0x30 && p[2] == 0x40) {
			l = 3;
			d = "select palette #0";
		} else if (p[0] == 0x9B && p[1] == 0x30 && p[2] == 0x41) {
			l = 3;
			d = "invert blinking";
		} else if (p[0] == 0x9B && p[1] == 0x31 && p[2] == 0x40) {
			l = 3;
			d = "select palette #1";
		} else if (p[0] == 0x9B && p[1] == 0x31 && p[2] == 0x41) {
			l = 3;
			d = "blink palettes 0/1 or 2/3";
		} else if (p[0] == 0x9B && p[1] == 0x31 && p[2] == 0x51) {
			l = 3;
			d = "unprotect line";
		} else if (p[0] == 0x9B && p[1] == 0x31 && p[2] == 0x50) {
			l = 3;
			d = "protect line";
		} else if (p[0] == 0x9B && p[1] == 0x32 && p[2] == 0x40) {
			l = 3;
			d = "select palette #2";
		} else if (p[0] == 0x9B && p[1] == 0x32 && p[2] == 0x41) {
			l = 3;
			d = "fast blinking (on, off, off)";
		} else if (p[0] == 0x9B && p[1] == 0x32 && p[2] == 0x53) {
			l = 3;
			d = "start selection";
		} else if (p[0] == 0x9B && p[1] == 0x32 && p[2] == 0x54) {
			l = 3;
			d = "end selection";
		} else if (p[0] == 0x9B && p[1] == 0x33 && p[2] == 0x40) {
			l = 3;
			d = "select palette #3";
		} else if (p[0] == 0x9B && p[1] == 0x33 && p[2] == 0x41) {
			l = 3;
			d = "fast blinking (off, on, off)";
		} else if (p[0] == 0x9B && p[1] == 0x34 && p[2] == 0x41) {
			l = 3;
			d = "fast blinking (off, off, on)";
		} else if (p[0] == 0x9B && p[1] == 0x35 && p[2] == 0x41) {
			l = 3;
			d = "blinking shift right";
		} else if (p[0] == 0x9B && p[1] == 0x36 && p[2] == 0x41) {
			l = 3;
			d = "blinking shift left";
		} else if (*p == 0x9c) {
			d = "Hintergrundfarbe schwarz bzw. normale Polarität";
		} else if (*p == 0x9d) {
			d = "Hintergrundfarbe setzen bzw. inverse Polarität";
		} else if (*p == 0x9e) {
			d = "Mosaikzeichenwiederholung bzw. Hintergrund transparent";
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
