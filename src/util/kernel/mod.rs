use super::vector::Vector2;

pub trait KernelFunction {
    fn gradient(&self, v: &Vector2) -> Vector2;
    fn kernel(&self, r: &Vector2) -> f64;
    fn h(&self) -> f64;
}

pub struct Poly6 {
    h: f64,
    a: f64,
}

impl Poly6 {
    pub fn new(h: f64) -> Poly6 {
        Poly6 {
            h: h,
            a: 4.0 / std::f64::consts::PI / h.powf(8.0),
        }
    }
}

impl KernelFunction for Poly6 {
    fn gradient(&self, r: &Vector2) -> Vector2 {
        let rm = r.magnitude();
        if rm < self.h {
            let d = self.h * self.h - rm * rm;
            r * (-6.0 * self.a * d * d)
        } else {
            r * 0.0
        }
    }

    fn kernel(&self, r: &Vector2) -> f64 {
        let rm = r.magnitude();
        if rm < self.h {
            &self.a * (&self.h * &self.h - rm * rm).powf(3.0)
        } else {
            0.0
        }
    }

    fn h(&self) -> f64 {
        self.h
    }
}
