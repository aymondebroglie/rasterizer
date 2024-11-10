mod algebra;
mod rasterizer;

use crate::algebra::{Matrix, ThreeD};
use crate::rasterizer::{CullMode, DrawCommand, Mesh, Rasterizer, ViewPort, RGBA};
use minifb::{Scale, ScaleMode, Window, WindowOptions};
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

    let positions = [
        ThreeD::new(0., 0.5, 0.),
        ThreeD::new(-0.5, -0.5, 0.),
        ThreeD::new(0.5, -0.5, 0.),
    ];

    let colors = [RGBA::red(), RGBA::green(), RGBA::blue()];

    view.clear(RGBA::new(0.8, 0.9, 1.1));

    while window.is_open() {
        let now = Instant::now();
        let dt = (now - last_frame_start).as_millis();
        last_frame_start = now;
        if dt > 0 {
            println!("Spent {} ms, {} fps ", dt, 1000 / dt);
        }

        view.draw(DrawCommand {
            mesh: Mesh {
                positions: positions.as_slice(),
                colors: &colors,
                indices : None
            },
            cull_mode: CullMode::None,
            transform: Matrix::identity(),
        });

        view.update(&mut window);
    }
}
