use std::fs;

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
            let r = i as f32 / (image_width - 1) as f32;
            let g = j as f32 / (image_height - 1) as f32;
            let b = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            image_lines.push(format!("{} {} {}", ir, ig, ib));
        }
    }

    println!("Done.");

    fs::write("image.ppm", image_lines.join("\n"))
        .map_err(|err| println!("Error when writing image file: {:?}", err))
        .ok();
}
