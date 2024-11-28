use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};

/// Detects horizontal and vertical symmetry in the image.
pub fn detect_symmetry(img: &DynamicImage) -> (bool, bool) {
    let (width, height) = img.dimensions();
    let mut horizontal_symmetry = true;
    let mut vertical_symmetry = true;

    // Check horizontal symmetry
    for y in 0..(height / 2) {
        for x in 0..width {
            if img.get_pixel(x, y) != img.get_pixel(x, height - 1 - y) {
                horizontal_symmetry = false;
                break;
            }
        }
        if !horizontal_symmetry {
            break;
        }
    }

    // Check vertical symmetry
    for x in 0..(width / 2) {
        for y in 0..height {
            if img.get_pixel(x, y) != img.get_pixel(width - 1 - x, y) {
                vertical_symmetry = false;
                break;
            }
        }
        if !vertical_symmetry {
            break;
        }
    }

    (horizontal_symmetry, vertical_symmetry)
}

/// Mirrors the image based on detected symmetry axes.
pub fn mirror_image(
    img: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    width: u32,
    height: u32,
    horizontal_symmetry: bool,
    vertical_symmetry: bool,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut mirrored_image = ImageBuffer::new(width, height);

    for x in 0..img.width() {
        for y in 0..img.height() {
            let pixel = img.get_pixel(x, y);

            // Original quadrant
            mirrored_image.put_pixel(x, y, *pixel);

            // Mirror vertically
            if horizontal_symmetry {
                mirrored_image.put_pixel(x, height - 1 - y, *pixel);
            }

            // Mirror horizontally
            if vertical_symmetry {
                mirrored_image.put_pixel(width - 1 - x, y, *pixel);
            }

            // Mirror both axes
            if horizontal_symmetry && vertical_symmetry {
                mirrored_image.put_pixel(width - 1 - x, height - 1 - y, *pixel);
            }
        }
    }

    mirrored_image
}
