#[derive(Clone, Copy)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}
impl Vector2 {
    pub fn new(x: f64, y: f64) -> Vector2 {
        Vector2 { x, y }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn fill(&mut self, v: f64) -> &Vector2 {
        self.x = v;
        self.y = v;
        self
    }
}

impl std::ops::Add<&Vector2> for &Vector2 {
    type Output = Vector2;
    fn add(self, s: &Vector2) -> Vector2 {
        Vector2::new(self.x + s.x, self.y + s.y)
    }
}

impl std::ops::Sub<&Vector2> for &Vector2 {
    type Output = Vector2;
    fn sub(self, s: &Vector2) -> Vector2 {
        Vector2::new(self.x - s.x, self.y - s.y)
    }
}

impl std::ops::Mul<f64> for &Vector2 {
    type Output = Vector2;
    fn mul(self, r: f64) -> Vector2 {
        Vector2::new(self.x * r, self.y * r)
    }
}

impl std::ops::Mul<&Vector2> for f64 {
    type Output = Vector2;
    fn mul(self, r: &Vector2) -> Vector2 {
        Vector2::new(self * r.x, self * r.y)
    }
}

impl std::ops::Mul<&Vector2> for &Vector2 {
    type Output = f64;
    fn mul(self, r: &Vector2) -> f64 {
        self.x * r.x + self.y * r.y
    }
}

impl std::ops::AddAssign<&Vector2> for Vector2 {
    fn add_assign(&mut self, s: &Vector2) {
        self.x += s.x;
        self.y += s.y;
    }
}

impl std::ops::SubAssign<&Vector2> for Vector2 {
    fn sub_assign(&mut self, s: &Vector2) {
        self.x -= s.x;
        self.y -= s.y;
    }
}

impl std::ops::MulAssign<f64> for Vector2 {
    fn mul_assign(&mut self, r: f64) {
        self.x *= r;
        self.y *= r;
    }
}

impl std::cmp::PartialEq<Vector2> for Vector2 {
    fn eq(&self, v: &Vector2) -> bool {
        self.x == v.x && self.y == v.y
    }
    fn ne(&self, v: &Vector2) -> bool {
        self.x != v.x || self.y != v.y
    }
}

#[test]
fn eq_test() {
    let a = Vector2::new(2.2, 3.5);
    let b = Vector2::new(0.0, 0.0);
    let c = Vector2::new(2.2, 3.5);
    let d = Vector2::new(0.0, 0.0);
    assert!(a == c);
    assert!(b == d);
    assert!(!(a == b));
    assert!(!(a == d));
}

#[test]
fn ne_test() {
    let a = Vector2::new(2.2, 3.5);
    let b = Vector2::new(0.0, 0.0);
    let c = Vector2::new(-2.2, 3.5);
    let d = Vector2::new(2.2, -3.5);
    assert!(a != b);
    assert!(a != c);
    assert!(a != d);
    assert!(b != c);
    assert!(b != d);
    assert!(c != d);
}

#[test]
fn add_test() {
    let a = Vector2::new(2.2, 3.5);
    let b = Vector2::new(0.0, 0.0);
    let c = &a + &b;
    let av = Vector2::new(2.2, 3.5);
    let bv = Vector2::new(0.0, 0.0);
    let cv = Vector2::new(2.2, 3.5);
    assert!(a == av);
    assert!(b == bv);
    assert!(c == cv);

    let a = Vector2::new(2.2, 3.5);
    let b = Vector2::new(1.4, 7.9);
    let c = &a + &b;
    let av = Vector2::new(2.2, 3.5);
    let bv = Vector2::new(1.4, 7.9);
    let cv = Vector2::new(3.6, 11.4);
    assert!(a == av);
    assert!(b == bv);
    assert!(c == cv);
}

#[test]
fn sub_test() {
    let a = Vector2::new(0.0, 0.0);
    let b = Vector2::new(1.0, 2.0);
    let c = &a - &b;
    let av = Vector2::new(0.0, 0.0);
    let bv = Vector2::new(1.0, 2.0);
    let cv = Vector2::new(-1.0, -2.0);
    assert!(a == av);
    assert!(b == bv);
    assert!(c == cv);

    let a = Vector2::new(1.0, 2.0);
    let b = Vector2::new(0.0, 0.0);
    let c = &a - &b;
    let av = Vector2::new(1.0, 2.0);
    let bv = Vector2::new(0.0, 0.0);
    let cv = Vector2::new(1.0, 2.0);
    assert!(a == av);
    assert!(b == bv);
    assert!(c == cv);

    let a = Vector2::new(2.0, 3.0);
    let b = Vector2::new(1.0, 2.0);
    let c = &a - &b;
    let av = Vector2::new(2.0, 3.0);
    let bv = Vector2::new(1.0, 2.0);
    let cv = Vector2::new(1.0, 1.0);
    assert!(a == av);
    assert!(b == bv);
    assert!(c == cv);
}

#[test]
fn mul_test() {
    let a = Vector2::new(1.0, 2.0);
    let b = &a * 1.0;
    let av = Vector2::new(1.0, 2.0);
    let bv = Vector2::new(1.0, 2.0);
    assert!(a == av);
    assert!(b == bv);

    let a = Vector2::new(1.0, 2.0);
    let b = &a * 0.0;
    let av = Vector2::new(1.0, 2.0);
    let bv = Vector2::new(0.0, 0.0);
    assert!(a == av);
    assert!(b == bv);

    let a = Vector2::new(1.0, 2.0);
    let b = &a * -1.0;
    let av = Vector2::new(1.0, 2.0);
    let bv = Vector2::new(-1.0, -2.0);
    assert!(a == av);
    assert!(b == bv);

    let a = Vector2::new(1.0, 2.0);
    let b = &a * 5.0;
    let av = Vector2::new(1.0, 2.0);
    let bv = Vector2::new(5.0, 10.0);
    assert!(a == av);
    assert!(b == bv);
}
