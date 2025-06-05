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
    let r_mean = (c1[0] as i32 + c2[0] as i32) / 2;
    let r = c1[0] as i32 - c2[0] as i32;
    let g = c1[1] as i32 - c2[1] as i32;
    let b = c1[2] as i32 - c2[2] as i32;

    let weight_r = 2 + r_mean / 256;
    let weight_g = 4;
    let weight_b = 2 + (255 - r_mean) / 256;

    ((weight_r * r * r + weight_g * g * g + weight_b * b * b) as f64).sqrt() as i32
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

    color_palette.insert("aesthetic".to_string(), vec![
        Rgba([102, 84, 94, 255]),   // muted mauve
        Rgba([163, 145, 147, 255]), // dusty rose
        Rgba([170, 111, 115, 255]), // faded red
        Rgba([238, 169, 144, 255]), // peach
        Rgba([246, 224, 181, 255]), // cream
        Rgba([202, 183, 174, 255]), // soft taupe
        Rgba([216, 190, 204, 255]), // pale lavender pink
        Rgba([186, 133, 157, 255]), // vintage rose
        Rgba([148, 112, 120, 255]), // antique mauve
        Rgba([234, 211, 206, 255]), // blush
        Rgba([224, 187, 228, 255]), // pastel purple
        Rgba([197, 163, 185, 255]), // dusty lilac
        Rgba([181, 139, 132, 255]), // soft clay
        Rgba([251, 221, 201, 255]), // light apricot
        Rgba([174, 146, 138, 255]), // warm gray
        Rgba([220, 194, 158, 255]), // beige sand
    ]);

    color_palette.insert("rainbowdash".to_string(), vec![
        Rgba([238, 64, 53, 255]),   // vibrant red
        Rgba([243, 119, 54, 255]),  // orange
        Rgba([253, 180, 73, 255]),  // goldenrod
        Rgba([253, 244, 152, 255]), // yellow
        Rgba([201, 232, 107, 255]), // yellow-green
        Rgba([123, 192, 67, 255]),  // green
        Rgba([64, 175, 125, 255]),  // sea green
        Rgba([34, 165, 175, 255]),  // teal
        Rgba([3, 146, 207, 255]),   // sky blue
        Rgba([47, 121, 190, 255]),  // azure
        Rgba([91, 94, 197, 255]),   // royal blue
        Rgba([132, 82, 196, 255]),  // indigo
        Rgba([175, 73, 215, 255]),  // purple
        Rgba([211, 76, 200, 255]),  // magenta
        Rgba([234, 84, 155, 255]),  // hot pink
        Rgba([241, 107, 122, 255]), // coral
    ]);

    color_palette.insert("citysunset".to_string(), vec![
        Rgba([238, 175, 97, 255]),   // golden orange
        Rgba([251, 144, 98, 255]),   // soft tangerine
        Rgba([243, 114, 100, 255]),  // coral
        Rgba([238, 93, 108, 255]),   // rose red
        Rgba([224, 82, 122, 255]),   // watermelon
        Rgba([211, 77, 133, 255]),   // raspberry pink
        Rgba([206, 73, 147, 255]),   // hot magenta
        Rgba([181, 60, 150, 255]),   // magenta-purple
        Rgba([157, 42, 145, 255]),   // plum
        Rgba([133, 28, 138, 255]),   // violet
        Rgba([106, 13, 131, 255]),   // deep purple
        Rgba([94, 16, 112, 255]),    // indigo purple
        Rgba([83, 18, 95, 255]),     // twilight purple
        //Rgba([71, 20, 78, 255]),     // night mauve
        Rgba([59, 21, 64, 255]),     // dusk violet
        //Rgba([47, 22, 52, 255]),     // after-sunset blackcurrant
    ]);

    color_palette.insert("instagramgradient".to_string(), vec![
        Rgba([254, 218, 117, 255]),  // golden yellow
        Rgba([253, 184, 85, 255]),   // sunflower orange
        Rgba([250, 150, 57, 255]),   // pumpkin
        Rgba([250, 126, 30, 255]),   // vivid orange
        Rgba([242, 104, 72, 255]),   // coral orange
        Rgba([228, 72, 95, 255]),    // pinkish red
        Rgba([214, 41, 118, 255]),   // magenta
        Rgba([188, 35, 140, 255]),   // fuchsia
        Rgba([167, 41, 161, 255]),   // orchid purple
        Rgba([150, 47, 191, 255]),   // violet
        Rgba([134, 58, 197, 255]),   // electric purple
        Rgba([111, 70, 204, 255]),   // blue-violet
        Rgba([98, 78, 208, 255]),    // periwinkle
        Rgba([89, 85, 211, 255]),    // indigo blue
        Rgba([84, 88, 212, 255]),    // cobalt
        Rgba([79, 91, 213, 255]),    // deep blue
    ]);

    let mut matrix: Vec<Rgba<u8>> = dwnscl.pixels().map(|p| p.2).collect();

    if palette_choice != "none" {
        let palette: Vec<Rgba<u8>> = color_palette.get(&palette_choice).unwrap().clone();

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
