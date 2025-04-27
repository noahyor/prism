#![feature(portable_simd)]

mod vector;
mod ray;
mod color;
mod hit;
mod sphere;
mod util;
mod interval;
mod camera;
mod writer;
mod material;
mod scene;
mod aabb;
mod anim;
mod bvh;
mod spline;
mod math;
mod texture;
mod image;
mod perlin;
mod debug;
mod renderer;
mod quality;
mod simd;
mod sieve;

#[cfg(test)]
mod tests;

use std::time::Instant;

use anim::Animation;
use bvh::BVHNode;
use camera::Camera;
use color::Color;
use hit::HittableList;
use material::{Dielectric, Lambertian, Metal};
use quality::QualityOptions;
use rand::random;
use renderer::{DefaultRenderer, ScreenUV, UV};
use scene::Scene;
use sphere::Sphere;
use texture::{CheckerTexture, ImageTexture};
use vector::Vector3;

const SCENE_ID: usize = 0;

const RENDERER: usize = 0;

const FILE_OUT: &str = "out.ppm";

fn main() {    
    print!("Generating Scene...  ");

    let scene = match SCENE_ID {
        0 => bouncing_spheres(),
        1 => checkered_spheres(),
        2 => earth(),
        _ => panic!("That Scene ID does not exist!"),
    };

    let renderer = match RENDERER {
        0 => DefaultRenderer::new(),
        1 => ScreenUV::new(),
        2 => UV::new(),
        _ => panic!("That renderer ID does not exist!"),
    };

    println!("Done!");

    println!(
        "This program will render {} objects into a {} by {} image ({}K pixels) into {}.",
        scene.objects(),
        scene.camera.img_height,
        scene.camera.img_width,
        scene.camera.total_pixels()/1000,
        FILE_OUT,
    );
    
    // print!("This operation is expected to take {}", get_time_str(scene.est_time()));

    // #[cfg(debug_assertions)]
    // println!(" (unoptimized).");

    // #[cfg(not(debug_assertions))]
    // println!(".");

    println!("Are you sure you want to continue? (Ctrl-c if you don't)");
    let _ = std::io::stdin().read_line(&mut String::new()).unwrap();
    
    println!("Rendering...");

    let begin = Instant::now();

    renderer.render(scene);

    let elapsed = begin.elapsed();

    println!("Done!");
    
    println!("Successfully rendered scene in {}",
        get_time_str(elapsed.as_secs() as usize),
    );
}

fn bouncing_spheres() -> Scene {
    let camera = Camera::new(
        QualityOptions::DEFAULT,
        16.0 / 9.0,
        Vector3(13.0, 2.0, 3.0),
        20.0,
        Vector3(0.0, 0.0, 0.0),
        Vector3(0.0, 1.0, 0.0),
        0.6,
        10.0,
        8,
    );

    let mut world = HittableList::new();

    let checker =
        CheckerTexture::from_const_col(
            0.32,
            Color { r: 0.2, g: 0.3, b: 0.1 },
            Color { r: 0.9, g: 0.9, b: 0.9 },
    ).to_box();
    world.add(Sphere::from_const_pos(
        0.0,
        -1000.0,
        0.0,
        1000.0,
        Lambertian::new(checker).to_dyn()
    ).as_box());
    
    for a in -11..11 {
        let a = a as f64;
        for b in -11..11 {
            let b = b as f64;
            let choose_mat: f64 = random();
            let center = Vector3(
                a + 0.9 * random::<f64>(),
                0.2,
                b + 0.9 * random::<f64>(),
            );

            if (center - Vector3(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let material = Lambertian::from_const_col(albedo).to_dyn();
                    let pos_2 =
                        center + Vector3::from(0.0, 0.0, 0.0);
                    world.add(Sphere::new(
                        Animation::linear(vec![center, pos_2], 0.2),
                        0.2, material
                    ).as_box());
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_with(0.5, 1.0);
                    let fuzz = random::<f64>() * 0.5;
                    let material = Metal {albedo, fuzz}.to_dyn();
                    world.add(Sphere::new_const_pos(
                        center,
                        0.2, material
                    ).as_box());
                } else {
                    let material = Dielectric::from(1.5).to_dyn();
                    world.add(
                        Sphere::new_const_pos(center,  0.2, material).as_box()
                    );
                }
            }
        }
    }

    let material = Dielectric::from(1.5).to_dyn();
    world.add(Sphere::from_const_pos(0.0, 1.0, 0.0, 1.0, material).as_box());
    
    let material = Lambertian::from_const_col(
        Color {r: 0.4, g: 0.2, b: 0.1}
    ).to_dyn();
    world.add(Sphere::from_const_pos(-4.0, 1.0, 0.0, 1.0, material).as_box());

    let material = Metal::from(0.7, 0.6, 0.5, 0.0).to_dyn();
    world.add(Sphere::from_const_pos(4.0, 1.0, 0.0, 1.0, material).as_box());

    let bvh = BVHNode::new(world.objects(), "debug.txt");

    Scene::new(camera, bvh)
}

fn checkered_spheres() -> Scene {
    let camera = Camera::new(
        QualityOptions::DEFAULT,
        16.0/9.0,
        Vector3(13.0,2.0,3.0),
        20.0,
        Vector3(0.0,0.0,0.0),
        Vector3(0.0,1.0,0.0),
        0.0,
        10.0,
        8,
    );

    let mut world = HittableList::new();
    let checker = Lambertian::new(
        CheckerTexture::from_const_col(
            0.32,
            Color { r: 0.2, g: 0.3, b: 0.1 },
            Color { r: 0.9, g: 0.9, b: 0.9 },
        ).to_box()
    ).to_dyn();

    world.add(Sphere::new_const_pos(
        Vector3(0.0, -10.0, 0.0),
        10.0,
        checker.clone()
    ).as_box());

    world.add(Sphere::from_const_pos(
        0.0, 10.0, 0.0, 10.0, checker.clone()
    ).as_box());
    
    let root = BVHNode::new(world.objects(), "debug.txt");

    Scene::new(camera, root)
}

fn earth() -> Scene {
    let camera = Camera::new(QualityOptions::DEFAULT, 16.0/9.0, Vector3(0.0, 0.0, 12.0), 20.0, Vector3(0.0, 0.0, 0.0), Vector3(0.0, 1.0, 0.0), 0.0, 10.0, 8);

    let earth_texture = ImageTexture::new(
        "img/map.png".to_string()
    ).to_box();
    // let earth_texture = DebugTexture::new(debug::DebugType::UV).to_dyn();
    let earth_surface = Lambertian::new(earth_texture).to_dyn();
    let globe = Sphere::from_const_pos(
        0.0, 0.0, 0.0, 2.0, earth_surface
    ).as_box();

    Scene::new(camera, BVHNode::new(vec![globe], "debug.txt"))
}

fn get_time_str(seconds: usize) -> String {
    let minutes = seconds / 60;
    let hours = minutes / 60;
    let days = hours / 24;
    let weeks = days / 7;

    let seconds_str = if seconds % 60 == 1 {
        String::from("1 second")
    } else {
        format!("{} seconds", seconds % 60)
    };

    let minutes_str = if minutes % 60 == 1 {
        String::from("1 minute and ")
    } else if minutes >= 1 {
        format!("{} minutes and ", minutes % 60)
    } else {String::new()};

    let hours_str = if hours % 24 == 1 {
        String::from("1 hour, ")
    } else if hours >= 1 {
        format!("{} hours, ", hours % 24)
    } else {String::new()};

    let days_str = if days % 7 == 1 {
        String::from("1 day, ")
    } else if days >= 1 {
        format!("{} days, ", days % 7)
    } else {String::new()};

    let weeks_str = if weeks == 1 {
        String::from("1 week, ")
    } else if weeks >= 1 {
        format!("{} weeks, ", weeks)
    } else {String::new()};

    format!("{}{}{}{}{}", weeks_str, days_str, hours_str, minutes_str, seconds_str)
}