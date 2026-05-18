use crate::command::{BorderColor, Mode};
use indicatif::ParallelProgressIterator;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::path::PathBuf;

use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgb, RgbImage};

pub fn resize_fit(
    img: &DynamicImage,
    max_w: u32,
    max_h: u32,
    border: u32,
    border_color: BorderColor,
) -> Result<RgbImage, image::ImageError> {
    let (orig_w, orig_h) = img.dimensions();
    let inner_w = max_w.saturating_sub(border * 2);
    let inner_h = max_h.saturating_sub(border * 2);
    let scale = (inner_w as f32 / orig_w as f32)
        .min(inner_h as f32 / orig_h as f32)
        .min(1.0);
    let scaled_w = (orig_w as f32 * scale).round() as u32;
    let scaled_h = (orig_h as f32 * scale).round() as u32;

    let resized = img.resize_exact(scaled_w, scaled_h, image::imageops::FilterType::Lanczos3);

    if border == 0 {
        return Ok(resized.to_rgb8());
    }

    let canvas_w = scaled_w + border * 2;
    let canvas_h = scaled_h + border * 2;
    let border_rgb = match border_color {
        BorderColor::White => Rgb([255u8, 255, 255]),
        BorderColor::Black => Rgb([0u8, 0, 0]),
    };
    let mut canvas: RgbImage = ImageBuffer::from_pixel(canvas_w, canvas_h, border_rgb);
    canvas.copy_from(&resized.to_rgb8(), border, border)?;
    Ok(canvas)
}

pub fn fit_canvas(
    img: &DynamicImage,
    canvas_w: u32,
    canvas_h: u32,
    border: u32,
    border_color: BorderColor,
) -> Result<RgbImage, image::ImageError> {
    let (orig_w, orig_h) = img.dimensions();
    let inner_w = canvas_w.saturating_sub(border * 2);
    let inner_h = canvas_h.saturating_sub(border * 2);
    let scale = (inner_w as f32 / orig_w as f32).min(inner_h as f32 / orig_h as f32);
    let scaled_w = (orig_w as f32 * scale).round() as u32;
    let scaled_h = (orig_h as f32 * scale).round() as u32;

    let resized = img.resize_exact(scaled_w, scaled_h, image::imageops::FilterType::Lanczos3);

    let border_rgb = match border_color {
        BorderColor::White => Rgb([255u8, 255, 255]),
        BorderColor::Black => Rgb([0u8, 0, 0]),
    };

    let mut canvas: RgbImage = ImageBuffer::from_pixel(canvas_w, canvas_h, border_rgb);
    canvas.copy_from(
        &resized.to_rgb8(),
        (canvas_w - scaled_w) / 2,
        (canvas_h - scaled_h) / 2,
    )?;
    Ok(canvas)
}

pub fn process_images(
    mode: Mode,
    output_path: PathBuf,
    prefix: String,
    suffix: String,
    border: u32,
    border_color: BorderColor,
    images: Vec<PathBuf>,
    output_is_file: bool,
) -> u64 {
    images
        .par_iter()
        .progress_count(images.len() as u64)
        .map(|path| {
            let img = match image::open(path) {
                Ok(img) => img,
                Err(e) => {
                    eprintln!("Failed to open {:?}: {}", path, e);
                    return 0;
                }
            };

            let (w, h) = img.dimensions();
            let result = match mode {
                Mode::Fit {
                    max_width,
                    max_height,
                } => resize_fit(
                    &img,
                    max_width.unwrap_or(w),
                    max_height.unwrap_or(h),
                    border,
                    border_color.clone(),
                ),
                Mode::Instagram => {
                    let (canvas_w, canvas_h) = if w >= h { (1080, 1080) } else { (1080, 1350) };
                    fit_canvas(&img, canvas_w, canvas_h, border, border_color.clone())
                }
            };

            let result = match result {
                Ok(img) => img,
                Err(e) => {
                    eprintln!("Failed to process {:?}: {}", path, e);
                    return 0;
                }
            };

            let file_path = if output_is_file {
                output_path.clone()
            } else {
                let Some(file_name) = path.file_stem() else {
                    eprintln!("Skipping {:?}: could not determine file name", path);
                    return 0;
                };

                let Some(extension) = path.extension() else {
                    eprintln!("Skipping {:?}: could not determine file extension", path);
                    return 0;
                };
                let file_name = format!(
                    "{}{}{}.{}",
                    prefix,
                    file_name.to_string_lossy(),
                    suffix,
                    extension.to_string_lossy()
                );

                output_path.join(file_name)
            };
            match result.save(&file_path) {
                Ok(_) => 1,
                Err(e) => {
                    eprintln!("Failed to save {:?}: {}", file_path, e);
                    0
                }
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::DynamicImage;

    fn solid_image(w: u32, h: u32) -> DynamicImage {
        DynamicImage::ImageRgb8(ImageBuffer::from_pixel(w, h, Rgb([128u8, 128, 128])))
    }

    #[test]
    fn resize_fit_no_border_preserves_aspect_ratio() {
        let img = solid_image(200, 100);
        let out = resize_fit(&img, 100, 100, 0, BorderColor::White).unwrap();
        assert_eq!(out.width(), 100);
        assert_eq!(out.height(), 50);
    }

    #[test]
    fn resize_fit_with_border_adds_padding() {
        let img = solid_image(100, 100);
        let out = resize_fit(&img, 100, 100, 10, BorderColor::White).unwrap();
        // image is scaled to 80×80 (inner), canvas is 100×100
        assert_eq!(out.width(), 100);
        assert_eq!(out.height(), 100);
    }

    #[test]
    fn resize_fit_does_not_upscale() {
        let img = solid_image(50, 50);
        let out = resize_fit(&img, 200, 200, 0, BorderColor::White).unwrap();
        assert_eq!(out.width(), 50);
        assert_eq!(out.height(), 50);
    }

    #[test]
    fn fit_canvas_centers_image() {
        let img = solid_image(100, 50);
        let out = fit_canvas(&img, 200, 200, 0, BorderColor::White).unwrap();
        assert_eq!(out.width(), 200);
        assert_eq!(out.height(), 200);
    }

    #[test]
    fn fit_canvas_with_border_shrinks_inner() {
        let img = solid_image(100, 100);
        let out = fit_canvas(&img, 200, 200, 20, BorderColor::White).unwrap();
        assert_eq!(out.width(), 200);
        assert_eq!(out.height(), 200);
        // border pixels should be white
        assert_eq!(out.get_pixel(5, 5), &Rgb([255u8, 255, 255]));
    }
}
