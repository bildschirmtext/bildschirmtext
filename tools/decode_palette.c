#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>
#include <stdlib.h>

int debug = 0;
int verbose = 0;
int create_files = 1;

void
print_palette(uint8_t *p, int c)
{
	printf("[\n");
	uint8_t *q = p;
	bool first = true;
	for (; q < p + c;) {
		if (!first) {
			printf(",\n");
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
		printf("\t\"#%02x%02x%02x\"", r, g, b);
		q += 2;
	}
	printf("\n]\n");
}

int
main(int argc, char **argv)
{
	FILE *f = fopen(argv[1], "r");
	uint8_t buffer[10*1024];
	memset(buffer, 255, sizeof(buffer));
	int total_length = fread(buffer, 1, sizeof(buffer), f);
	fclose(f);

	printf("{\n\"palette\": ");
	print_palette(buffer, total_length);
	printf( "}\n");

	return 0;
}
