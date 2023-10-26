use libs:: {sleep, sleep_ms};


fn main() {
	sleep(10);
	libs::ctrl_t();
	sleep_ms(1000);
	libs::simulate_typing("github.com/ramo828/");
	libs::enter();
	sleep_ms(1000);
	libs::step(15);
}
