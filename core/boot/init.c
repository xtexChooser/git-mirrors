void core_init();

void core_init() {
	*(char *)0xB8000 = 'A';
	*(char *)0xB8001 = 7;
	while (1) {
	}
}
