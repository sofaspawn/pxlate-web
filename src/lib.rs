use base64::{Engine as _, engine::general_purpose};
use image::{DynamicImage, Rgba};
use image::{GenericImageView, ImageBuffer, ImageFormat};
use wasm_bindgen::prelude::*;

use std::io::Cursor;

use std::collections::HashMap;

#[wasm_bindgen]
pub fn process_image(data: &[u8], pixel_size: u32, palette: &str) -> Result<String, JsValue> {
    // Decode image
    let img = image::load_from_memory(data)
        .map_err(|e| JsValue::from_str(&format!("Failed to load image: {}", e)))?;

    let small = downscale(img, pixel_size as usize);

    let pixelated = pxlate(small, pixel_size as usize, palette.to_string());

    let upscaled = upscale(pixelated, pixel_size as usize);

    // Encode image to PNG
    let mut cursor = Cursor::new(Vec::new());
    upscaled
        .write_to(&mut cursor, ImageFormat::Png)
        .map_err(|e| JsValue::from_str(&format!("Failed to write image: {}", e)))?;

    let buf = cursor.into_inner();

    // Convert to base64
    let b64 = general_purpose::STANDARD.encode(&buf);
    Ok(format!("data:image/png;base64,{}", b64))
}
fn upscale(img: DynamicImage, sfactor: usize) -> DynamicImage {
    let (width, height) = img.dimensions();
    //let (bigwidth, bigheight) = (width * 8, height * 8);
    //let (bigwidth, bigheight) = (width * 4, height * 4); // for a more detailed image
    let (bigwidth, bigheight) = (width * sfactor as u32, height * sfactor as u32);

    let upsclimg = img.resize_exact(bigwidth, bigheight, image::imageops::FilterType::Nearest);
    return upsclimg;
}

fn downscale(img: DynamicImage, sfactor: usize) -> DynamicImage {
    let (width, height) = img.dimensions();
    //let (smolwidth, smolheight) = (width / 4, height / 4); // for a more detailed image
    let (smolwidth, smolheight) = (width / sfactor as u32, height / sfactor as u32);

    let dwnsclimg = img.resize_exact(smolwidth, smolheight, image::imageops::FilterType::Nearest);
    return dwnsclimg;
}

fn color_diff(c1: Rgba<u8>, c2: Rgba<u8>) -> i32 {
    // because colors are perceived differently
    let r_diff = (c1[0] as i32 - c2[0] as i32).pow(2) * 3;
    let g_diff = (c1[1] as i32 - c2[1] as i32).pow(2) * 6;
    let b_diff = (c1[2] as i32 - c2[2] as i32).pow(2) * 1;

    ((r_diff + g_diff + b_diff) as f64).sqrt() as i32
}

fn pxlate(img: DynamicImage, sfactor: usize, palette_choice: String) -> DynamicImage {
    let dwnscl = downscale(img, sfactor);

    let mut color_palette: HashMap<String, Vec<Rgba<u8>>> = HashMap::new();

    color_palette.insert("retro".to_string(), vec![
        Rgba([0, 0, 0, 255]),
        Rgba([255, 255, 255, 255]),
        Rgba([136, 0, 0, 255]),
        Rgba([170, 255, 238, 255]),
        Rgba([204, 68, 204, 255]),
        Rgba([0, 204, 85, 255]),
        Rgba([0, 0, 170, 255]),
        Rgba([238, 238, 119, 255]),
        Rgba([221, 136, 85, 255]),
        Rgba([102, 68, 0, 255]),
        Rgba([255, 119, 119, 255]),
        Rgba([51, 51, 51, 255]),
        Rgba([119, 119, 119, 255]),
        Rgba([170, 255, 102, 255]),
        Rgba([0, 136, 255, 255]),
        Rgba([187, 187, 187, 255]),
    ]);

    color_palette.insert("onedark".to_string(), vec![
        Rgba([40, 44, 52, 255]),
        Rgba([171, 178, 191, 255]),
        Rgba([224, 108, 117, 255]),
        Rgba([152, 195, 121, 255]),
        Rgba([229, 192, 123, 255]),
        Rgba([97, 175, 239, 255]),
        Rgba([198, 120, 221, 255]),
        Rgba([86, 182, 194, 255]),
        Rgba([190, 80, 70, 255]),
        Rgba([92, 99, 112, 255]),
        Rgba([130, 137, 151, 255]),
        Rgba([209, 154, 102, 255]),
        Rgba([195, 232, 141, 255]),
        Rgba([56, 62, 71, 255]),
        Rgba([239, 241, 245, 255]),
        Rgba([75, 82, 94, 255]),
    ]);

    color_palette.insert("dracula".to_string(), vec![
        Rgba([40, 42, 54, 255]),
        Rgba([248, 248, 242, 255]),
        Rgba([255, 85, 85, 255]),
        Rgba([80, 250, 123, 255]),
        Rgba([241, 250, 140, 255]),
        Rgba([189, 147, 249, 255]),
        Rgba([255, 121, 198, 255]),
        Rgba([139, 233, 253, 255]),
        Rgba([255, 184, 108, 255]),
        Rgba([68, 71, 90, 255]),
        Rgba([98, 114, 164, 255]),
        Rgba([255, 110, 110, 255]),
        Rgba([95, 255, 135, 255]),
        Rgba([58, 60, 78, 255]),
        Rgba([241, 250, 140, 255]),
        Rgba([68, 71, 90, 255]),
    ]);

    color_palette.insert("monochrome".to_string(), vec![
        Rgba([0, 0, 0, 255]),
        Rgba([255, 255, 255, 255]),
        Rgba([85, 85, 85, 255]),
        Rgba([170, 170, 170, 255]),
        Rgba([212, 212, 212, 255]),
        Rgba([128, 128, 128, 255]),
        Rgba([192, 192, 192, 255]),
        Rgba([224, 224, 224, 255]),
        Rgba([160, 160, 160, 255]),
        Rgba([32, 32, 32, 255]),
        Rgba([96, 96, 96, 255]),
        Rgba([144, 144, 144, 255]),
        Rgba([208, 208, 208, 255]),
        Rgba([16, 16, 16, 255]),
        Rgba([240, 240, 240, 255]),
        Rgba([64, 64, 64, 255]),
    ]);

    color_palette.insert("monokai".to_string(), vec![
        Rgba([39, 40, 34, 255]),
        Rgba([248, 248, 242, 255]),
        Rgba([249, 38, 114, 255]),
        Rgba([166, 226, 46, 255]),
        Rgba([230, 219, 116, 255]),
        Rgba([102, 217, 239, 255]),
        Rgba([174, 129, 255, 255]),
        Rgba([161, 239, 228, 255]),
        Rgba([253, 151, 31, 255]),
        Rgba([69, 70, 64, 255]),
        Rgba([117, 113, 94, 255]),
        Rgba([249, 38, 114, 255]),
        Rgba([166, 226, 46, 255]),
        Rgba([56, 56, 48, 255]),
        Rgba([248, 248, 242, 255]),
        Rgba([117, 113, 94, 255]),
    ]);

    color_palette.insert("solarized".to_string(), vec![
        Rgba([0, 43, 54, 255]),
        Rgba([131, 148, 150, 255]),
        Rgba([220, 50, 47, 255]),
        Rgba([133, 153, 0, 255]),
        Rgba([181, 137, 0, 255]),
        Rgba([38, 139, 210, 255]),
        Rgba([211, 54, 130, 255]),
        Rgba([42, 161, 152, 255]),
        Rgba([203, 75, 22, 255]),
        Rgba([7, 54, 66, 255]),
        Rgba([88, 110, 117, 255]),
        Rgba([253, 246, 227, 255]),
        Rgba([238, 232, 213, 255]),
        Rgba([0, 43, 54, 255]),
        Rgba([253, 246, 227, 255]),
        Rgba([101, 123, 131, 255]),
    ]);

    //TODO: add the palettes in the color palette.
    //TODO: then let the user choose the palette and do the deed

    let palette: Vec<Rgba<u8>> = color_palette.get(&palette_choice).unwrap().clone();

    let mut matrix: Vec<Rgba<u8>> = dwnscl.pixels().map(|p| p.2).collect();

    for (i, pxl) in matrix.clone().iter().enumerate() {
        let (mut iclr, mut pxldiff) = (Rgba([0, 0, 0, 255]), 255);
        for clr in &palette {
            let diff = color_diff(*pxl, *clr);
            if diff < pxldiff {
                iclr = *clr;
            }
            pxldiff = min(pxldiff, diff);
        }
        matrix[i] = iclr;
    }

    let (smol_width, smol_height) = dwnscl.dimensions();

    let fin_img = ImageBuffer::from_fn(smol_width, smol_height, |x, y| {
        matrix[(y * smol_width + x) as usize] // Access the corresponding pixel
    });

    let upsclimg = upscale(DynamicImage::ImageRgba8(fin_img), sfactor);

    return upsclimg;
}

fn min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}
