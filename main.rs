use libs:: {
	sleep_ms,
	sleep,
	};


fn main() {
	sleep(10);                                                // 10 Saniye gozle
	// libs::ctrl_t();                                        // CTRL+T
	// sleep_ms(1000);                                        // 1 Saniye gozle
	// libs::simulate_typing("github.com/ramo828/");          // Url yaz
	// libs::enter();                                         // Enteri klikle
	sleep_ms(1000);                                        // 1 Saniye gozle

	for _i in 1..=10 {
		libs::step();
        sleep_ms(200);
    }
}
