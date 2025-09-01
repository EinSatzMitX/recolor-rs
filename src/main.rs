/*
* Program flow:
*   - Read Image
*   - Iterate through each pixel and search the nearest color
*   - yeah thats kinda it
* */

use std::panic;

use image::ImageReader;

#[derive(Debug)]
enum RecolorError {}

fn remap_fn(src: &mut Vec<u8>, palette: Vec<[u8; 3]>) -> Result<(), RecolorError> {
    Ok(())
}

fn main() {
    let path = "image.png";

    let dyn_img = ImageReader::open(path).unwrap().decode().unwrap();
    let rgb_img = dyn_img.to_rgb8();
    let (w, h) = rgb_img.dimensions();

    let mut pixels: Vec<u8> = rgb_img.into_raw();

    if pixels.len() != (w as usize) * (h as usize) * 3 {
        panic!("unexpected pixel buffer size!");
    }

    let palette: Vec<[u8; 3]> = vec![[0, 0, 0], [255, 255, 255], [255, 0, 0]];

    remap_fn(&mut pixels, palette).unwrap();
}
