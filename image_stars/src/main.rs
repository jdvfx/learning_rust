use image::{ImageBuffer, RgbImage};
use rand::thread_rng;
use rand::Rng;
// cli args
use std::env;
// exit if cli args incorrect
use std::process;

const WIDTH: u32 = 1980;
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

    let mut image: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);

    for _i in 0..stars {
        let mut rng = thread_rng();
        let x: u32 = rng.gen_range(0, &WIDTH);
        let y: u32 = rng.gen_range(0, &HEIGHT);

        let b: u8 = rng.gen_range(0, 255);
        *image.get_pixel_mut(x, y) = image::Rgb([b, b, b]);
    }
    image.save("stars.png").unwrap();
}
