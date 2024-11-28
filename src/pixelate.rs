use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};

/// Applies pixelation effect to the provided image with the specified block size.
pub fn apply_pixelation(img: &DynamicImage, block_size: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let mut pixelated_image = ImageBuffer::new(width, height);

    for block_y in (0..height).step_by(block_size as usize) {
        for block_x in (0..width).step_by(block_size as usize) {
            let mut r_total = 0;
            let mut g_total = 0;
            let mut b_total = 0;
            let mut pixel_count = 0;

            for y in block_y..(block_y + block_size).min(height) {
                for x in block_x..(block_x + block_size).min(width) {
                    let pixel = img.get_pixel(x, y);
                    r_total += pixel[0] as u32;
                    g_total += pixel[1] as u32;
                    b_total += pixel[2] as u32;
                    pixel_count += 1;
                }
            }

            let r_avg = (r_total / pixel_count) as u8;
            let g_avg = (g_total / pixel_count) as u8;
            let b_avg = (b_total / pixel_count) as u8;

            for y in block_y..(block_y + block_size).min(height) {
                for x in block_x..(block_x + block_size).min(width) {
                    pixelated_image.put_pixel(x, y, Rgb([r_avg, g_avg, b_avg]));
                }
            }
        }
    }

    pixelated_image
}
