#![deny(clippy::all)]
#![forbid(unsafe_code)]

use image::io::Reader;

use rand::thread_rng;
use rand::Rng;

use image;

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;

fn main() -> Result<(), Error> {
    // env_logger::init();

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scaled_size = LogicalSize::new(WIDTH as f64 * 3.0, HEIGHT as f64 * 3.0);
        WindowBuilder::new()
            .with_title("Conway's Game of Life")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let pixels_grid = PixelGrid::new_empty(WIDTH as usize, HEIGHT as usize);

    let img_path = "/home/bunker/projects/image/src/im.png";
    let img = Reader::open(&img_path).unwrap().decode().unwrap().to_rgb8();

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            pixels_grid.draw(pixels.get_frame(), img.to_owned(), input.mouse_diff());
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }
            window.request_redraw();
        }
    });
}

#[derive(Clone, Debug)]
struct PixelGrid {}

impl PixelGrid {
    fn new_empty(width: usize, height: usize) -> Self {
        assert!(width != 0 && height != 0);
        Self {}
    }

    fn draw(&self, screen: &mut [u8], img: image::RgbImage, mousedif: (f32, f32)) {
        //
        // let mut rng = thread_rng();
        //
        let mut x: u32 = 0;
        let mut y: u32 = 0;
        for pix in screen.chunks_exact_mut(4) {
            //
            // let o: u8 = rng.gen_range(1, 100);
            let o: u8 = rand::random();

            let div: f32 = 2.0;
            let mx: u32 = (mousedif.0 / div) as u32;
            let my: u32 = (mousedif.1 / div) as u32;

            if o > 98 {
                if x > 190 {
                    x = 0;
                    y += 1 + my;
                } else {
                    x += 1 + mx;
                }
                if y > 190 {
                    y = 0;
                }

                let x = x.min(190);
                let x = x.max(0);
                let y = y.min(190);
                let y = y.max(0);

                // if x > 0 && y > 0 && x < 200 && y < 200 {
                let p = img.get_pixel(x, y);
                let rx = p[0];
                let ry = p[1];
                let rz = p[2];

                let color = [rx, ry, rz, 0xff];
                pix.copy_from_slice(&color);
                // }
            }
        }
    }
}
