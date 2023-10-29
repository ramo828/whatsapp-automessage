mod modules;

fn main() {
    let texts = vec!["0558302766", "0776428183", "0558302766","0552145584","0556441486"];
	let _ = modules::image::image_write_text("image/test.jpg", "image/out.jpg", texts);
	let _ = modules::networks::init_server("127.0.0.1","8080");
}
