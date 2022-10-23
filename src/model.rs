use std::f32::consts::PI;


pub(crate) struct Screen {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) buffer: Vec<Vec<(u8, u8, u8)>>,
    pub(crate) zbuffer: Vec<Vec<f32>>,
}

impl Screen {
    pub (crate) fn clear(&mut self) {
        for width in self.buffer.iter_mut() {
            for i in width.iter_mut() {
                *i = (0, 0, 0);
            }
        }
        for width in self.zbuffer.iter_mut() {
            for i in width.iter_mut() {
                *i = 0.;
            }
        }
    }
    pub(crate) fn new(width: usize, height: usize) -> Self {
        let buffer = vec![vec![(0, 0, 0); width]; height];
        let zbuffer = vec![vec![0.0; width]; height];
        Self {
            width,
            height,
            buffer,
            zbuffer,
        }
    }
}

pub(crate) struct Scene {
    pub(crate) rs: f32,
    pub(crate) re: f32,
    pub(crate) rm: f32,
    pub(crate) rse: f32,
    pub(crate) rem: f32,
    pub(crate) ge: f32, //gamma earth
    pub(crate) gm: f32, //gamma moon
}

impl Scene {
    pub(crate) fn new(rs: f32, re: f32, rm: f32, rse: f32, rem: f32, ge: f32, gm: f32) -> Self {
        Self {
            rs,
            re,
            rm,
            rse,
            rem,
            ge,
            gm,
        }
    }
    pub(crate) fn at_t(&self, t: f32) -> Scene {
        Scene::new(
            self.rs,
            self.re,
            self.rm,
            self.rse,
            self.rem,
            t / 720. * 2. * PI,
            t / 60. * 2. * PI,
        )
    }
}


pub(crate) struct Vector {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
}

impl Vector {
    pub(crate) fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub(crate) fn empty() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
    pub(crate) fn dot(&self, rhs: &Vector) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    pub(crate) fn cross(&self, rhs: &Vector) -> Vector {
        Vector::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
    pub(crate) fn add(&self, rhs: &Vector) -> Vector {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }

    pub(crate) fn times(&self, factor: f32) -> Vector {
        Vector::new(self.x * factor, self.y * factor, self.z * factor)
    }

    pub(crate) fn normalized(&self) -> Vector {
        let mag = self.mag();
        Vector::new(self.x / mag, self.y / mag, self.z / mag)
    }

    pub(crate) fn inverted(&self) -> Vector {
        self.times(-1.)
    }

    pub(crate) fn mag(&self) -> f32 {
        return (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
    }
}
