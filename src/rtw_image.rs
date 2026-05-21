// rtw_image.rs
// Rust port of rtw_stb_image.h from Ray Tracing in One Weekend.
//
// Dependencies (Cargo.toml):
//   [dependencies]
//   image = "0.25"          # replaces stb_image: decodes common formats, returns f32/u8 pixels
//
// The `image` crate is the idiomatic Rust replacement for stb_image. It supports
// JPEG, PNG, BMP, TGA, HDR/EXR, and more, and can decode directly to f32 (linear) data.

use image::{DynamicImage, GenericImageView, Pixel};
use std::env;
use std::path::{Path, PathBuf};

/// Holds image data for texture mapping.
///
/// Pixel values are stored as linear (gamma = 1) floating-point RGB in `fdata`,
/// and as 8-bit RGB bytes in `bdata`.  If loading fails, `width()` and `height()`
/// return 0 and `pixel_data()` returns magenta (255, 0, 255).
pub struct RtwImage {
    image_width: u32,
    image_height: u32,
    /// Linear floating-point pixel data  (R, G, B interleaved, row-major)
    fdata: Option<Vec<f32>>,
    /// Linear 8-bit pixel data (R, G, B interleaved, row-major)
    bdata: Option<Vec<u8>>,
}

impl RtwImage {
    /// Creates an empty `RtwImage` (no image loaded).
    pub fn new() -> Self {
        RtwImage {
            image_width: 0,
            image_height: 0,
            fdata: None,
            bdata: None,
        }
    }

    /// Loads image data from `image_filename`.
    ///
    /// Search order (mirrors the C++ original):
    /// 1. `$RTW_IMAGES/<filename>`
    /// 2. `<filename>` (relative to CWD)
    /// 3. `images/<filename>`
    /// 4. `../images/<filename>` … up to six levels of `../`
    ///
    /// On failure an error message is printed to stderr and the image remains empty.
    pub fn from_file(image_filename: &str) -> Self {
        let mut img = Self::new();

        let candidates: Vec<PathBuf> = {
            let mut v = Vec::new();

            // 1. RTW_IMAGES env var
            if let Ok(dir) = env::var("RTW_IMAGES") {
                v.push(PathBuf::from(&dir).join(image_filename));
            }

            // 2. Bare filename
            v.push(PathBuf::from(image_filename));

            // 3–8. images/ under successive parent directories
            let mut prefix = PathBuf::new();
            for _ in 0..7 {
                v.push(prefix.join("images").join(image_filename));
                prefix = PathBuf::from("..").join(prefix);
            }

            v
        };

        for path in &candidates {
            if img.load(path) {
                return img;
            }
        }

        eprintln!("ERROR: Could not load image file '{image_filename}'.");
        img
    }

    /// Attempts to load the image at `path`.  Returns `true` on success.
    pub fn load(&mut self, path: &Path) -> bool {
        match image::open(path) {
            Ok(dyn_img) => {
                self.ingest(dyn_img);
                true
            }
            Err(_) => false,
        }
    }

    /// Returns the image width, or 0 if no image is loaded.
    pub fn width(&self) -> u32 {
        if self.fdata.is_none() {
            0
        } else {
            self.image_width
        }
    }

    /// Returns the image height, or 0 if no image is loaded.
    pub fn height(&self) -> u32 {
        if self.fdata.is_none() {
            0
        } else {
            self.image_height
        }
    }

    /// Returns the RGB bytes `[r, g, b]` of the pixel at `(x, y)`.
    ///
    /// Coordinates are clamped to valid range.  If no image is loaded, returns
    /// magenta `[255, 0, 255]`.
    pub fn pixel_data(&self, x: u32, y: u32) -> [u8; 3] {
        const MAGENTA: [u8; 3] = [255, 0, 255];

        let bdata = match &self.bdata {
            Some(b) => b,
            None => return MAGENTA,
        };

        let x = Self::clamp(x, 0, self.image_width);
        let y = Self::clamp(y, 0, self.image_height);
        let bytes_per_pixel: u32 = 3;
        let bytes_per_scanline = self.image_width * bytes_per_pixel;

        let offset = (y * bytes_per_scanline + x * bytes_per_pixel) as usize;
        [bdata[offset], bdata[offset + 1], bdata[offset + 2]]
    }

    // -------------------------------------------------------------------------
    // Private helpers
    // -------------------------------------------------------------------------

    /// Converts a loaded `DynamicImage` into internal `fdata` / `bdata` storage.
    fn ingest(&mut self, dyn_img: DynamicImage) {
        self.image_width = dyn_img.width();
        self.image_height = dyn_img.height();

        let total_pixels = (self.image_width * self.image_height) as usize;
        let total_components = total_pixels * 3;

        let mut fdata = Vec::with_capacity(total_components);
        let mut bdata = Vec::with_capacity(total_components);

        // `image` gives us pixels in row-major, left-to-right order.
        for (_x, _y, px) in dyn_img.pixels() {
            let rgba = px.to_rgba();
            // Normalise to [0.0, 1.0] linear — `image` stores sRGB u8 channels.
            let r = rgba[0] as f32 / 255.0;
            let g = rgba[1] as f32 / 255.0;
            let b = rgba[2] as f32 / 255.0;

            fdata.push(r);
            fdata.push(g);
            fdata.push(b);

            bdata.push(Self::float_to_byte(r));
            bdata.push(Self::float_to_byte(g));
            bdata.push(Self::float_to_byte(b));
        }

        self.fdata = Some(fdata);
        self.bdata = Some(bdata);
    }

    /// Clamps `x` to `[low, high)`.
    #[inline]
    fn clamp(x: u32, low: u32, high: u32) -> u32 {
        if x < low {
            return low;
        }
        if x < high {
            return x;
        }
        high.saturating_sub(1)
    }

    /// Converts a linear float in `[0.0, 1.0]` to a `u8` in `[0, 255]`.
    #[inline]
    fn float_to_byte(value: f32) -> u8 {
        if value <= 0.0 {
            return 0;
        }
        if value >= 1.0 {
            return 255;
        }
        (256.0 * value) as u8
    }
}

impl Default for RtwImage {
    fn default() -> Self {
        Self::new()
    }
}
