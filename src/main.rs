pub mod camera;
pub mod hittable;
pub mod material;
pub mod ray;
pub mod sphere;

use hittable::Hittable;
use image::{ImageBuffer, Rgb, RgbImage};
use nalgebra::Vector3;
use rand::Rng;
use ray::Ray;
use rayon::iter::ParallelIterator;
use rayon_progress::ProgressAdaptor;

use crate::{
    camera::Camera,
    hittable::HittableList,
    material::{Dielectric, Lambertian, Metal},
    sphere::Sphere,
};

fn color(r: f32, g: f32, b: f32) -> Vector3<f32> {
    Vector3::new(r, g, b)
}

fn write_color_component(component: &f32, samples_per_pixel: i32) -> u8 {
    // Divide the color by the number of samples and gamma-correct for gamma=2.0
    let scale = 1.0 / samples_per_pixel as f32;
    let c = (component * scale).sqrt();

    // Write the translated [0,255] value of each color component.
    return (256.0 * c.clamp(0.0, 0.999)) as u8;
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

fn random_scene() -> HittableList {
    let mut rng = rand::thread_rng();
    let mut world = HittableList::new();

    let ground_material = Box::new(Lambertian::new(color(0.5, 0.5, 0.5)));
    world.add(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen_range(0.0..1.0);
            let center = Vector3::new(
                a as f32 + 0.9 * rng.gen_range(0.0..1.0),
                0.2,
                b as f32 + 0.9 * rng.gen_range(0.0..1.0),
            );

            if (center - Vector3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                if choose_mat < 0.8 {
                    world.add(Sphere::new(
                        center,
                        0.2,
                        Box::new(Lambertian::new(random_vector(0.0, 1.0))),
                    ))
                } else if choose_mat < 0.95 {
                    world.add(Sphere::new(
                        center,
                        0.2,
                        Box::new(Metal::new(random_vector(0.5, 1.0), rng.gen_range(0.0..0.5))),
                    ))
                } else {
                    world.add(Sphere::new(center, 0.2, Box::new(Dielectric::new(1.5))));
                }
            }
        }
    }

    world.add(Sphere::new(
        Vector3::new(0.0, 1.0, 0.0),
        1.0,
        Box::new(Dielectric::new(1.5)),
    ));

    world.add(Sphere::new(
        Vector3::new(-4.0, 1.0, 0.0),
        1.0,
        Box::new(Lambertian::new(color(0.4, 0.2, 0.1))),
    ));

    world.add(Sphere::new(
        Vector3::new(4.0, 1.0, 0.0),
        1.0,
        Box::new(Metal::new(color(0.7, 0.6, 0.5), 0.0)),
    ));

    return world;
}

fn main() {
    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f32 / aspect_ratio) as u32;
    let samples_per_pixel = 500;
    let max_depth = 50;
    let number_of_pixels = image_height as usize * image_width as usize;

    // World
    let world = random_scene();

    // Camera
    let lookfrom = Vector3::new(13.0, 2.0, 3.0);
    let lookat = Vector3::new(0.0, 0.0, 0.0);
    let vup = Vector3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let mut buffer: RgbImage = ImageBuffer::new(image_width, image_height);
    let it = ProgressAdaptor::new(buffer.par_enumerate_pixels_mut());
    let progress = it.items_processed();
    it.for_each(|(x, y, pixel)| {
        let pixel_color: Vector3<f32> = (0..samples_per_pixel)
            .map(|_| {
                let mut rng = rand::thread_rng();
                let u = (x as f32 + rng.gen::<f32>()) / (image_width - 1) as f32;
                let v =
                    ((image_height - y - 1) as f32 + rng.gen::<f32>()) / (image_height - 1) as f32;
                let r = cam.get_ray(u, v);
                ray_color(&r, &world, max_depth)
            })
            .sum();
        *pixel = Rgb([
            write_color_component(&pixel_color[0], samples_per_pixel),
            write_color_component(&pixel_color[1], samples_per_pixel),
            write_color_component(&pixel_color[2], samples_per_pixel),
        ]);

        print!("\r{}%", (progress.get() * 100) / number_of_pixels);
    });

    println!("Done.");

    buffer
        .save("image.png")
        .map_err(|err| println!("Error when writing image file: {:?}", err))
        .ok();
}
