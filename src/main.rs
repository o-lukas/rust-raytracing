pub mod ray;

use std::fs;

use nalgebra::Vector3;

fn color(r: f32, g: f32, b: f32) -> Vector3<f32> {
    Vector3::new(r, g, b)
}

fn write_color(pixel_color: &Vector3<f32>) -> String {
    format!(
        "{} {} {}",
        (pixel_color[0] * 255.999) as i32,
        (pixel_color[1] * 255.999) as i32,
        (pixel_color[2] * 255.999) as i32
    )
}

fn main() {
    // Image

    let image_width = 256;
    let image_height = 256;

    // Render
    let mut image_lines = vec![
        "P3".to_string(),
        image_width.to_string(),
        image_height.to_string(),
        "255".to_string(),
    ];

    for j in (0..image_height - 1).rev() {
        println!("Scanlines remaining: {}", j);

        for i in 0..image_width {
            let pixel_color = color(
                i as f32 / (image_width - 1) as f32,
                j as f32 / (image_height - 1) as f32,
                0.25,
            );
            image_lines.push(write_color(&pixel_color));
        }
    }

    println!("Done.");

    fs::write("image.ppm", image_lines.join("\n"))
        .map_err(|err| println!("Error when writing image file: {:?}", err))
        .ok();
}
