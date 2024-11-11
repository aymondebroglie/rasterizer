use crate::algebra::ThreeD;
use crate::colors::RGBA;


pub(crate) struct Vertex {

    

}
pub(crate) struct Mesh {
    pub(crate) positions: Vec<ThreeD>,
    pub(crate) colors: Vec<RGBA>,
    pub(crate) indices: Option<Vec<usize>>,
}

impl Mesh {
    pub(crate) fn len(&self) -> usize {
        match &self.indices {
            None => self.positions.len(),
            Some(i) => i.len(),
        }
    }

    fn get_index(&self, i: usize) -> usize {
        match &self.indices {
            None => i,
            Some(indices) => indices[i],
        }
    }
    pub(crate) fn get(&self, i: usize) -> ThreeD {
        self.positions[self.get_index(i)]
    }

    pub(crate) fn get_color(&self, i: usize) -> RGBA {
        self.colors[self.get_index(i)]
    }

    pub(crate) fn rectangle() -> Self {
        let positions = vec![
            ThreeD::new(-0.5, -0.5, 0.),
            ThreeD::new(-0.5, 0.5, 0.),
            ThreeD::new(0.5, -0.5, 0.),
            ThreeD::new(0.5, 0.5, 0.),
        ];

        let colors = vec![
            RGBA::red(),
            RGBA::green(),
            RGBA::blue(),
            RGBA::new(1., 1., 1.),
        ];

        let indices = vec![0, 1, 2, 2, 1, 3];

        Mesh {
            positions,
            colors,
            indices: Some(indices),
        }
    }
    pub(crate) fn cube() -> Self {
        let positions = vec![
            // -X face
            ThreeD::new(-1., -1., -1.),
            ThreeD::new(-1., 1., -1.),
            ThreeD::new(-1., -1., 1.),
            ThreeD::new(-1., 1., 1.),
            // +X face
            ThreeD::new(1., -1., -1.),
            ThreeD::new(1., 1., -1.),
            ThreeD::new(1., -1., 1.),
            ThreeD::new(1., 1., 1.),
            // -Y face
            ThreeD::new(-1., -1., -1.),
            ThreeD::new(1., -1., -1.),
            ThreeD::new(-1., -1., 1.),
            ThreeD::new(1., -1., 1.),
            // +Y face
            ThreeD::new(-1., 1., -1.),
            ThreeD::new(1., 1., -1.),
            ThreeD::new(-1., 1., 1.),
            ThreeD::new(1., 1., 1.),
            // -Z face
            ThreeD::new(-1., -1., -1.),
            ThreeD::new(1., -1., -1.),
            ThreeD::new(-1., 1., -1.),
            ThreeD::new(1., 1., -1.),
            // +Z face
            ThreeD::new(-1., -1., 1.),
            ThreeD::new(1., -1., 1.),
            ThreeD::new(-1., 1., 1.),
            ThreeD::new(1., 1., 1.),
        ];

        let colors = vec![
            // -X face
            RGBA::new(0., 1., 1.),
            RGBA::new(0., 1., 1.),
            RGBA::new(0., 1., 1.),
            RGBA::new(0., 1., 1.),
            // +X ace
            RGBA::new(1., 0., 0.),
            RGBA::new(1., 0., 0.),
            RGBA::new(1., 0., 0.),
            RGBA::new(1., 0., 0.),
            // -Y face
            RGBA::new(1., 0., 1.),
            RGBA::new(1., 0., 1.),
            RGBA::new(1., 0., 1.),
            RGBA::new(1., 0., 1.),
            // +Y face
            RGBA::new(0., 1., 0.),
            RGBA::new(0., 1., 0.),
            RGBA::new(0., 1., 0.),
            RGBA::new(0., 1., 0.),
            // -Z face
            RGBA::new(1., 1., 0.),
            RGBA::new(1., 1., 0.),
            RGBA::new(1., 1., 0.),
            RGBA::new(1., 1., 0.),
            // +Z face
            RGBA::new(0., 0., 1.),
            RGBA::new(0., 0., 1.),
            RGBA::new(0., 0., 1.),
            RGBA::new(0., 0., 1.),
        ];

        let indices: Vec<usize> = vec![
            // -X face
            0, 2, 1, 1, 2, 3, // +X face
            4, 5, 6, 6, 5, 7, // -Y face
            8, 9, 10, 10, 9, 11, // +Y face
            12, 14, 13, 14, 15, 13, // -Z face
            16, 18, 17, 17, 18, 19, // +Z face
            20, 21, 22, 21, 23, 22,
        ];

        Mesh {
            positions,
            colors,
            indices: Some(indices),
        }
    }
}
