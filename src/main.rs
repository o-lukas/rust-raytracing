pub mod camera;
pub mod hittable;
pub mod material;
pub mod ray;
pub mod sphere;

use std::{f32::consts::PI, fs};

use hittable::Hittable;
use nalgebra::Vector3;
use rand::Rng;
use ray::Ray;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

use crate::{camera::Camera, hittable::HittableList, material::Lambertian, sphere::Sphere};

fn color(r: f32, g: f32, b: f32) -> Vector3<f32> {
    Vector3::new(r, g, b)
}

fn write_color_component(component: &f32, samples_per_pixel: i32) -> i32 {
    // Divide the color by the number of samples and gamma-correct for gamma=2.0
    let scale = 1.0 / samples_per_pixel as f32;
    let c = (component * scale).sqrt();

    // Write the translated [0,255] value of each color component.
    return (256.0 * c.clamp(0.0, 0.999)) as i32;
}

fn random_vector(min: f32, max: f32) -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    Vector3::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}

fn random_vector_in_unit_sphere() -> Vector3<f32> {
    loop {
        let p = random_vector(-1.0, 1.0);
        if p.dot(&p) >= 1.0 {
            continue;
        }

        return p;
    }
}

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Vector3<f32> {
    // If we've exceeded the ray bounce limi, no more light is gathered
    if depth <= 0 {
        return color(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, f32::MAX) {
        if let Some((attenuation, scattered)) = rec.material().scatter(&r, &rec) {
            return attenuation.zip_map(&ray_color(&scattered, world, depth - 1), |l, r| l * r);
        }

        return color(0.0, 0.0, 0.0);
    }

    let unit_direction = r.direction().normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * color(1.0, 1.0, 1.0) + t * color(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let r = (PI / 4.0).cos();
    let mut world = HittableList::new();

    let material_left = Box::new(Lambertian::new(color(0.0, 0.0, 1.0)));
    let material_right = Box::new(Lambertian::new(color(1.0, 0.0, 0.0)));

    world.add(Sphere::new(Vector3::new(-r, 0.0, -1.0), r, material_left));
    world.add(Sphere::new(Vector3::new(r, 0.0, -1.0), r, material_right));

    // Camera
    let cam = Camera::new(90.0, aspect_ratio);

    // Render
    let mut image_lines = vec![
        "P3".to_string(),
        image_width.to_string(),
        image_height.to_string(),
        "255".to_string(),
    ];

    let image = (0..(image_height - 1))
        .into_par_iter()
        .rev()
        .flat_map(|j| {
            (0..image_width)
                .flat_map(|i| {
                    println!(
                        "Row {}/{}: Column {}/{}",
                        j,
                        image_height,
                        i + 1,
                        image_width
                    );

                    let pixel_color: Vector3<f32> = (0..samples_per_pixel)
                        .map(|_| {
                            let mut rng = rand::thread_rng();
                            let u = (i as f32 + rng.gen::<f32>()) / (image_width - 1) as f32;
                            let v = (j as f32 + rng.gen::<f32>()) / (image_height - 1) as f32;
                            let r = cam.get_ray(u, v);
                            ray_color(&r, &world, max_depth)
                        })
                        .sum();
                    pixel_color
                        .iter()
                        .map(|comp| write_color_component(comp, samples_per_pixel))
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<i32>>();

    for chunk in image.chunks(3) {
        image_lines.push(format!("{} {} {}", chunk[0], chunk[1], chunk[2]));
    }

    println!("Done.");

    fs::write("image.ppm", image_lines.join("\n"))
        .map_err(|err| println!("Error when writing image file: {:?}", err))
        .ok();
}
