use pixel8bit::pixelate::apply_pixelation;
use pixel8bit::symmetry::{detect_symmetry, mirror_image};
use image::DynamicImage;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: {} <input_path> <output_path> <block_size> <is_mirrored>", args[0]);
        return;
    }

    let input_path = &args[1];
    let output_path = &args[2];
    let block_size: u32 = args[3].parse().expect("Block size must be a positive integer");
    let is_mirrored: bool = args[4].parse().expect("is_mirrored must be true or false");

    let img = image::open(input_path).expect("Failed to load image");
    let pixelated_img = apply_pixelation(&img, block_size);

    if is_mirrored {
        let (horizontal, vertical) = detect_symmetry(&img);
        let mirrored_img = mirror_image(&pixelated_img, img.width(), img.height(), horizontal, vertical);
        mirrored_img.save(output_path).expect("Failed to save output image");
    } else {
        DynamicImage::ImageRgb8(pixelated_img).save(output_path).expect("Failed to save output image");
    }
}
