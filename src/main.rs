mod algebra;
mod colors;
mod mesh;
mod rasterizer;

use crate::algebra::{Matrix, ThreeD};
use crate::colors::RGBA;
use crate::mesh::Mesh;
use crate::rasterizer::{CullMode, DrawCommand, Rasterizer};
use minifb::{Scale, ScaleMode, Window, WindowOptions};
use std::f64::consts::PI;
use std::time::Instant;

fn main() {
    let mut window = Window::new(
        "Rasterizer",
        800,
        600,
        WindowOptions {
            borderless: true,
            title: true,
            resize: true,
            scale: Scale::X1,
            scale_mode: ScaleMode::Stretch,
            topmost: false,
            transparency: false,
            none: false,
        },
    )
    .unwrap();
    window.set_target_fps(0);

    let mut view = Rasterizer::new(window.get_size().0, window.get_size().1);
    let mut last_frame_start = Instant::now();
    let mut rotation = 0.;

    view.clear(RGBA::new(0.8, 0.9, 1.1));
    let mesh = Mesh::cube();

    while window.is_open() {
        let now = Instant::now();
        let dt = (now - last_frame_start).as_millis();
        last_frame_start = now;
        if dt > 0 {
            println!("Spent {} ms, {} fps ", dt, 1000 / dt);
        }

        rotation += 0.001;

        view.clear(RGBA::new(0.8, 0.9, 1.1));

        let transform =
            Matrix::perspective(0.01, 10., PI / 3., view.width as f64 / view.height as f64)
                * Matrix::translate(ThreeD::new(0.,0.,-5.))
                * Matrix::rotate_zx(rotation)
                * Matrix::rotate_xy(rotation * 1.31);


        view.draw(DrawCommand {
            mesh: &mesh,
            cull_mode: CullMode::ClockWise,
            transform,
        });

        view.update(&mut window);
    }
}
