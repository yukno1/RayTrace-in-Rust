mod color;
mod vec3;

use crate::color::*;

fn main() {
    let image_width = 256;
    let image_height = 256;

    let out = std::io::stdout();

    println!("P3\n{image_width} {image_height}\n255");

    for j in 0..image_height {
        eprintln!("\rScanlines remaining: {} ", image_height - j);
        for i in 0..image_width {
            let pixel_color = Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.0,
            );

            write_color(&out, pixel_color);
        }
    }
    eprintln!("\rDone");
}
