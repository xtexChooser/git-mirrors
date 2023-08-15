#include <boot/boot.h>

char *core_init(boot_info_t *bootinfo);

static void itoa(char *buf, int base, int d) {
	char *p = buf;
	char *p1, *p2;
	unsigned long ud = d;
	int divisor = 10;

	/* If %d is specified and D is minus, put ‘-’ in the head. */
	if (base == 'd' && d < 0) {
		*p++ = '-';
		buf++;
		ud = -d;
	} else if (base == 'x')
		divisor = 16;

	/* Divide UD by DIVISOR until UD == 0. */
	do {
		int remainder = ud % divisor;

		*p++ = (remainder < 10) ? remainder + '0' : remainder + 'a' - 10;
	} while (ud /= divisor);

	/* Terminate BUF. */
	*p = 0;

	/* Reverse BUF. */
	p1 = buf;
	p2 = p - 1;
	while (p1 < p2) {
		char tmp = *p1;
		*p1 = *p2;
		*p2 = tmp;
		p1++;
		p2--;
	}
}

char *core_init(boot_info_t *bootinfo) {
	char *vbuf = (char *)0xB8000;
	char str[20];
	itoa(str, 'x', (int)&core_init);
	char *sptr = str;
	while (*sptr != 0) {
		*vbuf = *sptr;
		vbuf++;
		*vbuf = 7;
		vbuf++;
		sptr++;
	}
	*vbuf = ' ';
	vbuf++;
	*vbuf = 7;
	vbuf++;
	itoa(str, 'x', (int)&itoa);
	sptr = str;
	while (*sptr != 0) {
		*vbuf = *sptr;
		vbuf++;
		*vbuf = 7;
		vbuf++;
		sptr++;
	}
	*vbuf = ' ';
	vbuf++;
	
	*vbuf = 7;
	vbuf++;
	return "TEST RETURN MESSAGE";
	while (1) {
	}
}
