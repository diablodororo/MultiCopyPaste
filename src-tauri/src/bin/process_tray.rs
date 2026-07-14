use image::{GenericImageView, ImageBuffer, Rgba, imageops::FilterType};
use std::path::Path;

fn main() {
    let input_path = "/Users/wandererdark/.gemini/antigravity-ide/brain/8e22fff3-7fb9-4a14-94dc-9fbca4e7a2f1/multicopypaste_tray_icon_fixed.png";
    let img = image::open(input_path).expect("Failed to open tray icon input image");
    let (w, h) = img.dimensions();

    // Define center search region to skip any outer mockup frame
    let min_x = (w as f32 * 0.2) as u32;
    let max_x = (w as f32 * 0.8) as u32;
    let min_y = (w as f32 * 0.2) as u32;
    let max_y = (w as f32 * 0.8) as u32;

    // Find bounding box of bright pixels inside center region
    let mut bbox_min_x = max_x;
    let mut bbox_max_x = min_x;
    let mut bbox_min_y = max_y;
    let mut bbox_max_y = min_y;

    for y in min_y..max_y {
        for x in min_x..max_x {
            let pixel = img.get_pixel(x, y);
            let brightness = (pixel[0] as f32 + pixel[1] as f32 + pixel[2] as f32) / 3.0;
            if brightness > 60.0 {
                if x < bbox_min_x { bbox_min_x = x; }
                if x > bbox_max_x { bbox_max_x = x; }
                if y < bbox_min_y { bbox_min_y = y; }
                if y > bbox_max_y { bbox_max_y = y; }
            }
        }
    }

    println!("Found bounding box: ({}, {}) -> ({}, {})", bbox_min_x, bbox_min_y, bbox_max_x, bbox_max_y);

    let crop_w = bbox_max_x - bbox_min_x + 1;
    let crop_h = bbox_max_y - bbox_min_y + 1;

    // Create square canvas with 15% padding
    let sq_size = (crop_w.max(crop_h) as f32 * 1.15) as u32;
    let mut canvas: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(sq_size, sq_size);

    let offset_x = (sq_size - crop_w) / 2;
    let offset_y = (sq_size - crop_h) / 2;

    for y in 0..crop_h {
        for x in 0..crop_w {
            let pixel = img.get_pixel(bbox_min_x + x, bbox_min_y + y);
            let brightness = (pixel[0] as f32 + pixel[1] as f32 + pixel[2] as f32) / 3.0;
            if brightness > 40.0 {
                let alpha = ((brightness - 40.0) * (255.0 / 215.0)).min(255.0) as u8;
                canvas.put_pixel(offset_x + x, offset_y + y, Rgba([255, 255, 255, alpha]));
            } else {
                canvas.put_pixel(offset_x + x, offset_y + y, Rgba([0, 0, 0, 0]));
            }
        }
    }

    // Resize to 64x64 for crisp menu bar icon
    let tray_icon_64 = image::imageops::resize(&canvas, 64, 64, FilterType::Lanczos3);
    tray_icon_64.save("icons/tray_icon.png").expect("Failed to save tray_icon.png");

    let tray_icon_128 = image::imageops::resize(&canvas, 128, 128, FilterType::Lanczos3);
    tray_icon_128.save("icons/tray_icon@2x.png").expect("Failed to save tray_icon@2x.png");

    // Save a preview with dark background to brain dir so user can preview it in walkthrough
    let mut preview: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(128, 128);
    for pixel in preview.pixels_mut() {
        *pixel = Rgba([30, 30, 35, 255]);
    }
    image::imageops::overlay(&mut preview, &tray_icon_128, 0, 0);
    preview.save("/Users/wandererdark/.gemini/antigravity-ide/brain/8e22fff3-7fb9-4a14-94dc-9fbca4e7a2f1/multicopypaste_tray_icon_preview.png").expect("Failed to save preview");

    println!("Tray icons generated successfully!");
}
