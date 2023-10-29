use anyhow::Result;
use opencv::{
    core::{self, Mat, Scalar, Size, CV_8U},
    imgcodecs,
    imgproc,
};

pub fn image_write_text(input_image:&str, output_image:&str, data: Vec<&str>) -> Result<()>{
    let mut image = imgcodecs::imread(input_image, imgcodecs::IMREAD_COLOR)?;

    // Yazı rengi (örneğin, beyaz)
    let color = Scalar::new(255.0, 255.0, 255.0, 0.0);

    // Yazı tipi ve boyutu
    let font_face = imgproc::FONT_HERSHEY_SIMPLEX;
    let font_scale = 2.3;

    // Başlangıç yazı konumu
    let mut y = 120; // Y koordinatı

    for text in data {
        // Metni resmin üzerine ekleyin
        let org = core::Point::new(320, y); // Sabit X koordinatı, Y koordinatı artar
        imgproc::put_text(
            &mut image,
            text,
            org,
            font_face,
            font_scale,
            color,
            2,
            imgproc::LINE_8,
            false,
        )?;
        y += 80; // Y koordinatını sabit bir miktar artır
    }
	let org = core::Point::new(400, 800); // Sabit X koordinatı, Y koordinatı artar
	imgproc::put_text(
		&mut image,
		"50",
		org,
		font_face,
		2.5,
		color,
		2,
		imgproc::LINE_8,
		false,
	)?;
    imgcodecs::imwrite(output_image, &image, &core::Vector::<i32>::new())?;

    Ok(())
}

// fn main() -> Result<()> {
//     // let mut image = imgcodecs::imread("image/test.jpg", imgcodecs::IMREAD_COLOR)?;

//     // // Yazı rengi (örneğin, beyaz)
//     // let color = Scalar::new(255.0, 255.0, 255.0, 0.0);

//     // // Yazı tipi ve boyutu
//     // let font_face = imgproc::FONT_HERSHEY_SIMPLEX;
//     // let font_scale = 2.3;

//     // // Başlangıç yazı konumu
//     // let mut y = 120; // Y koordinatı

//     // // Metinleri dize değişkenlerine koyun
//     // let texts = vec!["0558302766", "0776428183", "0558302766","0552145584","0554569422"];

//     // for text in texts {
//     //     // Metni resmin üzerine ekleyin
//     //     let org = core::Point::new(320, y); // Sabit X koordinatı, Y koordinatı artar
//     //     imgproc::put_text(
//     //         &mut image,
//     //         &text,
//     //         org,
//     //         font_face,
//     //         font_scale,
//     //         color,
//     //         2,
//     //         imgproc::LINE_8,
//     //         false,
//     //     )?;
//     //     y += 80; // Y koordinatını sabit bir miktar artır
//     // }
// 	// let org = core::Point::new(400, 800); // Sabit X koordinatı, Y koordinatı artar
// 	// imgproc::put_text(
// 	// 	&mut image,
// 	// 	"50",
// 	// 	org,
// 	// 	font_face,
// 	// 	2.5,
// 	// 	color,
// 	// 	2,
// 	// 	imgproc::LINE_8,
// 	// 	false,
// 	// )?;
//     // imgcodecs::imwrite("image/final-output.jpg", &image, &core::Vector::<i32>::new())?;

//     // Ok(())
// }
