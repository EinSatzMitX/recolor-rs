/*
* Program flow:
*   - Read Image
*   - Iterate through each pixel and search the nearest color
*   - yeah thats kinda it
* */

use clap::{ArgAction, Parser};
use image::{ImageBuffer, ImageReader, RgbImage};
use std::{fmt::Debug, panic, path::Path};

#[derive(Debug, Clone)]
struct Palette {
    name: String,
    colors: Vec<[u8; 3]>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    #[arg(short = 'i', long = "input", action = ArgAction::Set, required = true)]
    input: String,

    #[arg(short = 'o', long = "output", action = ArgAction::Set, required = true)]
    output: String,

    #[arg(short = 'p', long = "palette", action = ArgAction::Set, required = true)]
    palette: String,
}

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

#[allow(clippy::vec_init_then_push)]
fn get_palette(p_name: String) -> Option<Palette> {
    let mut palettes: Vec<Palette> = Vec::new();
    palettes.push(Palette {
        colors: [
            [40, 40, 40],
            [235, 219, 178],
            [251, 73, 52],
            [184, 187, 38],
            [250, 189, 47],
            [131, 165, 152],
            [211, 134, 155],
            [142, 192, 124],
            [235, 173, 52],
            [146, 131, 116],
            [214, 93, 14],
            [215, 153, 33],
            [69, 133, 136],
            [177, 98, 134],
            [204, 36, 29],
            [152, 151, 26],
        ]
        .to_vec(),
        name: "gruvbox".to_string(),
    });
    palettes.push(Palette {
        colors: [
            [244, 219, 214],
            [240, 198, 198],
            [245, 189, 230],
            [198, 160, 246],
            [237, 135, 150],
            [238, 153, 160],
            [245, 169, 127],
            [238, 212, 159],
            [166, 218, 149],
            [139, 213, 202],
            [145, 215, 227],
            [125, 196, 228],
            [138, 173, 244],
            [183, 189, 248],
            [202, 211, 245],
            [184, 192, 224],
            [165, 173, 203],
            [147, 154, 183],
            [128, 135, 162],
            [110, 115, 141],
            [91, 96, 120],
            [73, 77, 100],
            [54, 58, 79],
            [36, 39, 58],
            [30, 32, 48],
            [24, 25, 38],
        ]
        .to_vec(),
        name: "catpuccin-macchiato".to_string(),
    });

    // find by reference, then cloned() to return Option<Palette>
    palettes.iter().find(|p| p.name == p_name).cloned()
}
fn main() {
    let args = CliArgs::parse();
    let input = args.input;

    let dyn_img = ImageReader::open(input).unwrap().decode().unwrap();
    let rgba = dyn_img.to_rgba8();
    let (w, h) = rgba.dimensions();

    let mut pixels = strip_alpha_blend(&rgba.into_raw(), [40, 40, 40]);
    if pixels.len() != (w as usize) * (h as usize) * 3 {
        panic!("unexpected pixel buffer size!");
    }

    let palette = match get_palette(args.palette.to_lowercase()) {
        Some(v) => v.colors,
        None => panic!("Palette name not found!"),
    };

    remap_fn(&mut pixels, palette).unwrap();
    save_rgb_image(pixels, w, h, args.output).unwrap();
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
