/*
* Program flow:
*   - Read Image
*   - Iterate through each pixel and search the nearest color
*   - yeah thats kinda it
* */

use std::{fmt::Debug, panic, path::Path};

use image::{ImageBuffer, ImageReader, RgbImage};

#[derive(Debug)]
enum RecolorError {}

fn strip_alpha_blend(rgba: &[u8], bg: [u8; 3]) -> Vec<u8> {
    let mut rgb = Vec::with_capacity(rgba.len() / 4 * 3);
    for chunk in rgba.chunks_exact(4) {
        let (r, g, b, a) = (
            chunk[0] as f32,
            chunk[1] as f32,
            chunk[2] as f32,
            chunk[3] as f32 / 255.0,
        );
        let blended = [
            (r * a + bg[0] as f32 * (1.0 - a)) as u8,
            (g * a + bg[1] as f32 * (1.0 - a)) as u8,
            (b * a + bg[2] as f32 * (1.0 - a)) as u8,
        ];
        rgb.extend_from_slice(&blended);
    }
    rgb
}

/* Use Eucledian coordinates, for faster calculation */
fn get_nearest_color(pixel: [u8; 3], palette: &[[u8; 3]]) -> [u8; 3] {
    let mut best = palette[0];
    let mut best_dist = u32::MAX;

    for &color in palette {
        let dr = pixel[0] as i32 - color[0] as i32;
        let dg = pixel[1] as i32 - color[1] as i32;
        let db = pixel[2] as i32 - color[2] as i32;
        let dist = (dr * dr + dg * dg + db * db) as u32;

        if dist < best_dist {
            best = color;
            best_dist = dist;
        }
    }
    best
}

fn remap_fn(src: &mut [u8], palette: Vec<[u8; 3]>) -> Result<(), RecolorError> {
    for p in 0..(src.len() / 3) {
        let offset = p * 3;
        let pixel = [src[offset], src[offset + 1], src[offset + 2]];

        let lowest = get_nearest_color(pixel, &palette);

        src[offset] = lowest[0];
        src[offset + 1] = lowest[1];
        src[offset + 2] = lowest[2];
    }
    Ok(())
}

fn save_rgb_image<P: AsRef<Path>>(
    pixels: Vec<u8>,
    width: u32,
    height: u32,
    path: P,
) -> Result<(), RecolorError> {
    if pixels.len() != (width as usize) * (height as usize) * 3 {
        panic!("pixel buffer size does not match width*height*3");
    }

    // construct an ImageBuffer<Rgb<u8>, Vec<u8>>
    let img: RgbImage = ImageBuffer::from_vec(width, height, pixels)
        .ok_or_else(|| panic!("failed to create ImageBuffer"))
        .unwrap();

    // save to disk (format inferred from file extension)
    img.save(path).unwrap();

    Ok(())
}

fn main() {
    let path = "image.png";

    let dyn_img = ImageReader::open(path).unwrap().decode().unwrap();
    // let rgb_img = dyn_img.to_rgb8();
    let rgba = dyn_img.to_rgba8();
    let (w, h) = rgba.dimensions();

    // let mut pixels: Vec<u8> = rgb_img.into_raw();
    let mut pixels = strip_alpha_blend(&rgba.into_raw(), [40, 40, 40]); // Gruvbox bg

    if pixels.len() != (w as usize) * (h as usize) * 3 {
        panic!("unexpected pixel buffer size!");
    }

    // gruvbox palette
    let palette: Vec<[u8; 3]> = [
        [40, 40, 40],    // bg0: #282828
        [235, 219, 178], // fg:  #ebdbb2
        [251, 73, 52],   // red: #fb4934
        [184, 187, 38],  // green: #b8bb26
        [250, 189, 47],  // yellow: #fabd2f
        [131, 165, 152], // aqua: #83a598
        [211, 134, 155], // purple: #d3869b
        [142, 192, 124], // bright green: #8ec07c
        [235, 173, 52],  // orange: #ebad34
        [146, 131, 116], // gray: #928374
        [214, 93, 14],   // bright orange: #d65d0e
        [215, 153, 33],  // bright yellow: #d79921
        [69, 133, 136],  // teal: #458588
        [177, 98, 134],  // pink/purple alt: #b16286
        [204, 36, 29],   // bright red: #cc241d
        [152, 151, 26],  // bright green (alt): #98971a
    ]
    .to_vec();

    remap_fn(&mut pixels, palette).unwrap();
    save_rgb_image(pixels, w, h, "output.jpg").unwrap();
}

#[cfg(test)]
mod tests {
    use crate::get_nearest_color;

    #[test]
    fn test_smaller() {
        let palette: Vec<[u8; 3]> = vec![
            [0, 255, 0],
            [255, 255, 255],
            [200, 200, 200],
            [170, 170, 170],
        ];
        assert_eq!(
            get_nearest_color([254, 254, 254], &palette),
            [255, 255, 255]
        );
    }
}
