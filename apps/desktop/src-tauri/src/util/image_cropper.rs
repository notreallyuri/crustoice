use crate::structures::crop::{CropData, TempCroppedImage};
use image::codecs::gif::{GifDecoder, GifEncoder};
use image::AnimationDecoder;
use std::{
    fs,
    io::BufReader,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

impl TempCroppedImage {
    pub fn process(path: &str, crop: &CropData) -> Result<Self, String> {
        println!("Processing crop for image: {}", path);
        println!(
            "Crop parameters: x={}, y={}, width={}, height={}",
            crop.x, crop.y, crop.width, crop.height
        );

        let mut img =
            image::open(path).map_err(|e| format!("Failed to open original image: {}", e))?;

        let x = (crop.x * crop.scale_x).max(0.0) as u32;
        let y = (crop.y * crop.scale_y).max(0.0) as u32;
        let width = (crop.width * crop.scale_x).max(1.0) as u32;
        let height = (crop.height * crop.scale_y).max(1.0) as u32;

        let cropped_img = image::imageops::crop(&mut img, x, y, width, height).to_image();

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let ext = Path::new(path)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("png");

        if ext == "gif" {
            return Self::process_gif(path, crop);
        }

        let final_img: image::DynamicImage = if ext == "jpg" || ext == "jpeg" {
            image::DynamicImage::ImageRgba8(cropped_img)
                .into_rgb8()
                .into()
        } else {
            image::DynamicImage::ImageRgba8(cropped_img)
        };

        let temp_filename = format!("temp_{}.{}", timestamp, ext);
        let temp_path = std::env::temp_dir().join(temp_filename);

        final_img
            .save(&temp_path)
            .map_err(|e| format!("Failed to save cropped image: {}", e))?;

        Ok(Self { path: temp_path })
    }

    fn process_gif(path: &str, crop: &CropData) -> Result<Self, String> {
        let x = (crop.x * crop.scale_x).max(0.0) as u32;
        let y = (crop.y * crop.scale_y).max(0.0) as u32;
        let width = (crop.width * crop.scale_x).max(1.0) as u32;
        let height = (crop.height * crop.scale_y).max(1.0) as u32;

        let input =
            BufReader::new(fs::File::open(path).map_err(|e| format!("Failed to open GIF: {}", e))?);
        let decoder = GifDecoder::new(input).map_err(|e| format!("Failed to decode GIF: {}", e))?;
        let frames = decoder
            .into_frames()
            .collect_frames()
            .map_err(|e| format!("Failed to collect frames: {}", e))?;

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let temp_path = std::env::temp_dir().join(format!("temp_{}.gif", timestamp));
        let output = fs::File::create(&temp_path)
            .map_err(|e| format!("Failed to create temp GIF: {}", e))?;

        let mut encoder = GifEncoder::new(output);
        encoder
            .set_repeat(image::codecs::gif::Repeat::Infinite)
            .map_err(|e| format!("Failed to set repeat: {}", e))?;

        for frame in frames {
            let delay = frame.delay();
            let img = frame.into_buffer();

            let cropped = image::imageops::crop_imm(&img, x, y, width, height).to_image();
            let new_frame = image::Frame::from_parts(cropped, 0, 0, delay);

            encoder
                .encode_frame(new_frame)
                .map_err(|e| format!("Failed to encode frame: {}", e))?;
        }

        Ok(Self { path: temp_path })
    }
}

impl Drop for TempCroppedImage {
    fn drop(&mut self) {
        if self.path.exists() {
            let _ = fs::remove_file(&self.path);
            println!("Cleaned up temporary crop file: {:?}", self.path);
        }
    }
}
