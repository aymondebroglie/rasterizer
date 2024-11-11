use crate::algebra::{FourD, Matrix};
use crate::colors::RGBA;
use crate::mesh::Mesh;
use minifb::Window;
use std::mem::swap;

pub(crate) struct Rasterizer {
    pixels: Vec<u32>,
    pub(crate) width: usize,
    pub(crate) height: usize,
    view_port: ViewPort,
}

pub(crate) enum CullMode {
    None,
    ClockWise,
    CounterClockWise,
}

pub(crate) struct DrawCommand<'a> {
    pub(crate) mesh: &'a Mesh,
    pub(crate) cull_mode: CullMode,
    pub(crate) transform: Matrix,
}

pub(crate) struct ViewPort {
    pub(crate) x_min: usize,
    pub(crate) x_max: usize,
    pub(crate) y_min: usize,
    pub(crate) y_max: usize,
}

impl ViewPort {
    fn apply(&self, v: FourD) -> FourD {
        FourD::new(
            self.x_min as f64 + (self.x_max - self.x_min) as f64 * (0.5 + 0.5 * v.x()),
            self.y_min as f64 + (self.y_max - self.y_min) as f64 * (0.5 - 0.5 * v.y()),
            v.z(),
            v.w(),
        )
    }
}

impl Rasterizer {
    pub(crate) fn new(width: usize, height: usize) -> Self {
        let pixels = vec![0; width * height];
        Rasterizer {
            pixels,
            width,
            height,
            view_port: ViewPort {
                x_min: 0,
                x_max: width,
                y_min: 0,
                y_max: height,
            },
        }
    }
    pub(crate) fn clear(&mut self, color: RGBA) {
        self.pixels.fill(color.to_color());
    }

    fn set(&mut self, x: usize, y: usize, color: RGBA) {
        self.pixels[x + y * self.width] = color.to_color();
    }

    fn resize(&mut self, window: &Window) {
        let (new_width, new_height) = window.get_size();
        if new_width != self.width || new_height != self.height {
            self.width = new_width;
            self.height = new_height;
            self.pixels = vec![0; self.width * self.height];
            self.view_port = ViewPort {
                x_min: 0,
                x_max: self.width,
                y_min: 0,
                y_max: self.height,
            }
        }
    }

    pub(crate) fn update(&mut self, window: &mut Window) {
        self.resize(window);
        window
            .update_with_buffer(self.pixels.as_slice(), self.width, self.height)
            .unwrap();
    }

    pub(crate) fn draw(&mut self, cmd: DrawCommand) {
        let mut i: usize = 0;

        while i < cmd.mesh.len() {
            let mut v0 = cmd.transform * cmd.mesh.get(i).as_point();
            let mut v1 = cmd.transform * cmd.mesh.get(i + 1).as_point();
            let mut v2 = cmd.transform * cmd.mesh.get(i + 2).as_point();

            v0.perspective_divide();
            v1.perspective_divide();
            v2.perspective_divide();

            v0 = self.view_port.apply(v0);
            v1 = self.view_port.apply(v1);
            v2 = self.view_port.apply(v2);

            let c0 = cmd.mesh.get_color(i);
            let mut c1 = cmd.mesh.get_color(i + 1);
            let mut c2 = cmd.mesh.get_color(i + 2);

            let mut det012 = (v1 - v0).det2d(&(v2 - v0));
            let is_counter_clockwise = det012 < 0.;


            let skip = match cmd.cull_mode {
                CullMode::None => false,
                CullMode::ClockWise => !is_counter_clockwise,
                CullMode::CounterClockWise => is_counter_clockwise,
            };

            i += 3;

            if skip {
                continue;
            }

            if is_counter_clockwise {
                swap(&mut v1, &mut v2);
                swap(&mut c1, &mut c2);
                det012 = -det012;
            };

            let x_point_min = v0.x().min(v1.x()).min(v2.x()) as usize;
            let x_min = 0.max(self.view_port.x_min).max(x_point_min);

            let x_point_max = v0.x().max(v1.x()).max(v2.x()) as usize;
            let x_max = self.width.min(self.view_port.x_max).min(x_point_max);

            let y_point_min = v0.y().min(v1.y()).min(v2.y()) as usize;
            let y_min = 0.max(self.view_port.y_min).max(y_point_min);

            let y_point_max = v0.y().max(v1.y()).max(v2.y()) as usize;
            let y_max = self.height.min(self.view_port.y_max).min(y_point_max);

            for y in y_min..y_max {
                for x in x_min..x_max {
                    let point = FourD::new(x as f64 + 0.5, y as f64 + 0.5, 0., 0.);

                    let det0 = (v1 - v0).det2d(&(point - v0));
                    let det1 = (v2 - v1).det2d(&(point - v1));
                    let det2 = (v0 - v2).det2d(&(point - v2));

                    if det0 >= 0. && det1 >= 0. && det2 >= 0. {
                        let l0 = det1 / det012;
                        let l1 = det2 / det012;
                        let l2 = det0 / det012;
                        self.set(x, y, c0 * l0 + c1 * l1 + c2 * l2);
                    }
                }
            }

        }
    }
}
