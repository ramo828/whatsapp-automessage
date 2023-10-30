extern crate uinput;
extern crate mouse_keyboard_input;

use uinput::event::keyboard;
use std::thread;
use std::time::Duration;
use std::process::Command;
use mouse_keyboard_input::VirtualDevice;
use mouse_keyboard_input::key_codes::*;



pub fn scroll(max: i32, speed: u64) {
	let channel_size = 1024;
	let writing_interval = Duration::from_secs(1);
	let mut device_result = VirtualDevice::new(writing_interval, channel_size);
	thread::sleep(Duration::from_secs(10));

	for _ in 1..max {
		thread::sleep(Duration::from_millis(speed));
		let mut device = device_result.as_mut().expect("Hata nedeni");

		device.scroll_y(100);
	}
}

#[allow(dead_code)]

pub fn system(command: &str) -> String {
    let output = Command::new(command)
        .output()
        .expect("Komut çalıştırılamadı");

    String::from_utf8_lossy(&output.stdout).to_string()

}


#[allow(dead_code)]
pub fn simulate_typing(text: &str) {
	std::println!("Yazilacaq metin: {}", &text);
	let mut typing = key_device("dev1");
    for chr in text.chars() {
		sleep_ms(100);

        let karakter = match chr.to_ascii_uppercase() {
            'A' => keyboard::Key::A,
            'B' => keyboard::Key::B,
			'C' => keyboard::Key::C,
            'D' => keyboard::Key::D,
			'E' => keyboard::Key::E,
            'F' => keyboard::Key::F,
			'G' => keyboard::Key::G,
            'H' => keyboard::Key::H,
			'I' => keyboard::Key::I,
			'J' => keyboard::Key::J,
            'K' => keyboard::Key::K,
			'L' => keyboard::Key::L,
            'M' => keyboard::Key::M,
			'N' => keyboard::Key::N,
			'O' => keyboard::Key::O,
            'P' => keyboard::Key::P,
			'Q' => keyboard::Key::Q,
            'R' => keyboard::Key::R,
			'S' => keyboard::Key::S,
            'T' => keyboard::Key::T,
			'U' => keyboard::Key::U,
			'V' => keyboard::Key::V,
            'X' => keyboard::Key::X,
			'Y' => keyboard::Key::Y,
            'Z' => keyboard::Key::Z,
			'W' => keyboard::Key::W,
			'/' => keyboard::Key::Slash,
			'\\' => keyboard::Key::BackSlash,
			'.' => keyboard::Key::Dot,
			'0' => keyboard::Key::_0,
			'1' => keyboard::Key::_1,
			'2' => keyboard::Key::_2,
			'3' => keyboard::Key::_3,
			'4' => keyboard::Key::_4,
			'5' => keyboard::Key::_5,
			'6' => keyboard::Key::_6,
			'7' => keyboard::Key::_7,
			'8' => keyboard::Key::_8,
			'9' => keyboard::Key::_9,
            // Diğer karakterler için de aynı şekilde devam edin
            _ => {
                println!("Bu karakter desteklenmiyor: {}", chr);
                continue;
            }
			
        };
		// std::println!("{}", &chr);
        typing.click(&karakter).unwrap();
        // typing.release(&karakter).unwrap();
    }
    typing.synchronize().unwrap();
}

pub fn key_device(name: &str) -> uinput::Device {
    let my_device = uinput::default()
        .unwrap()
        .name(name)
        .unwrap()
        .event(uinput::event::Keyboard::All)
        .unwrap()
        .create()
        .unwrap();
    
    my_device
}
#[allow(dead_code)]

pub fn ctrl_alt_n() {
	let mut ctrlt =key_device("ctrl_alt_n");
	sleep_ms(200);
	ctrlt.press(&keyboard::Key::LeftControl).unwrap();
	ctrlt.press(&keyboard::Key::LeftAlt).unwrap();
	ctrlt.press(&keyboard::Key::N).unwrap();

	ctrlt.release(&keyboard::Key::LeftControl).unwrap();
	ctrlt.release(&keyboard::Key::LeftAlt).unwrap();
	ctrlt.release(&keyboard::Key::N).unwrap();
	ctrlt.synchronize().unwrap();

}
#[allow(dead_code)]

pub fn ctrl_alt_p() {
	let mut ctrlt =key_device("ctrl_alt_[]");
	sleep_ms(200);
	ctrlt.press(&keyboard::Key::LeftControl).unwrap();
	ctrlt.press(&keyboard::Key::LeftAlt).unwrap();
	ctrlt.press(&keyboard::Key::P).unwrap();

	ctrlt.release(&keyboard::Key::LeftControl).unwrap();
	ctrlt.release(&keyboard::Key::LeftAlt).unwrap();
	ctrlt.release(&keyboard::Key::P).unwrap();
	ctrlt.synchronize().unwrap();

}
#[allow(dead_code)]

pub fn enter() {
	let mut etab =key_device("enter_tab");
	sleep_ms(200);
	etab.click(&keyboard::Key::Enter).unwrap();
	etab.synchronize().unwrap();
}

pub fn Tab() {
	let mut etab =key_device("tab");
	sleep_ms(200);
	etab.click(&keyboard::Key::Tab).unwrap();
	etab.synchronize().unwrap();
}
#[allow(dead_code)]

pub fn step() {
	let mut step_dev = VirtualDevice::new(Duration::from_millis(500), 1024).unwrap();
	// device.click(KEY_DOWN).unwrap();




	// let mut step_dev =key_device("step");
    sleep_ms(200);
    step_dev.click(KEY_DOWN).unwrap();
	step_dev.synchronize().unwrap();

}
#[allow(dead_code)]

pub fn sleep(sec : u64) {
	thread::sleep(Duration::from_secs(sec));
}
#[allow(dead_code)]

pub fn sleep_ms(ms : u64) {
	thread::sleep(Duration::from_millis(ms));
}