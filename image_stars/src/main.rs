use image::{ImageBuffer, RgbImage};
use rand::thread_rng;
use rand::Rng;
// cli args
use std::env;
// exit if cli args incorrect
use std::{cmp::max, cmp::min, process};

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

fn main() {
    // get CLI args
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!(
            "Usage: 'Cargo run 1000' or 'image_stars 1000' to generate an image with 1000 stars"
        );
        process::exit(1);
    }
    let stars = &args[1];
    let stars: u32 = stars.to_string().parse::<u32>().unwrap();

    let isbw = true;
    let ismono = true;

    let mut image: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);

    let v: Vec<(i8, i8)> = vec![(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)];

    for _i in 0..stars {
        let mut rng = thread_rng();
        let x: u32 = rng.gen_range(0, &WIDTH);
        let y: u32 = rng.gen_range(0, &HEIGHT);

        let mut r: u8 = rng.gen_range(0, 255);
        let mut g: u8 = rng.gen_range(0, 255);
        let mut b: u8 = rng.gen_range(0, 255);
        if ismono {
            if r > 150 {
                r = 255;
            } else {
                r = 0;
            }
        }
        if isbw {
            g = r;
            b = r;
        }

        let s: u8 = rng.gen_range(0, 100);
        let big_star_threshold = 96;
        if s > big_star_threshold {
            for j in v.iter() {
                let x_ = x as i32 + j.0 as i32;
                let y_ = y as i32 + j.1 as i32;
                let x__ = min(max(x_ as u32, 0), &WIDTH - 1);
                let y__ = min(max(y_ as u32, 0), &HEIGHT - 1);
                *image.get_pixel_mut(x__, y__) = image::Rgb([r, g, b]);
            }
        } else {
            *image.get_pixel_mut(x, y) = image::Rgb([r, g, b]);
        }
    }
    image.save("stars.png").unwrap();
}
