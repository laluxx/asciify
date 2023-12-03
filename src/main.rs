use colored::*;
use image::{imageops, Rgba};
use std::env;

const HEIGHT: u32 = 192;
const WIDTH: u32 = 100;
const CHAR: char = '█';

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_image>", args[0]);
        return;
    }

    let img_path = &args[1];
    let img = match image::open(img_path) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("Failed to open image '{}': {}", img_path, e);
            return;
        }
    };

    // Resize the image
    let resized_img = imageops::resize(&img, HEIGHT, WIDTH, imageops::FilterType::Nearest);

    for y in 0..resized_img.height() {
        for x in 0..resized_img.width() {
            let pixel = resized_img.get_pixel(x, y);
            print!("{}", colorize_full_block(*pixel));
        }
        println!();
    }
}

// Colorize based on the color of the pixel
fn colorize_full_block(pixel: Rgba<u8>) -> ColoredString {
    let color = Color::TrueColor {
        r: pixel[0],
        g: pixel[1],
        b: pixel[2],
    };

    CHAR.to_string().color(color)
}
