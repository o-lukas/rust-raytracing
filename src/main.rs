pub mod ray;

use std::fs;

use nalgebra::Vector3;
use ray::Ray;

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

fn ray_color(r: &Ray) -> Vector3<f32> {
    let unit_direction = r.direction().normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * color(1.0, 1.0, 1.0) + t * color(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vector3::new(0.0, 0.0, 0.0);
    let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, focal_length);

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
            let u = i as f32 / (image_width - 1) as f32;
            let v = j as f32 / (image_height - 1) as f32;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&r);
            image_lines.push(write_color(&pixel_color));
        }
    }

    println!("Done.");

    fs::write("image.ppm", image_lines.join("\n"))
        .map_err(|err| println!("Error when writing image file: {:?}", err))
        .ok();
}
