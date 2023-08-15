void core_init();

void core_init() {
	*(char *)0xB8000 = 'A';
	*(char *)0xB8001 = 7;
	*(short *)0xB8003 = 0x0808;
	*(short *)0xB8005 = *(short *)core_init;
	while (1) {
	}
}
