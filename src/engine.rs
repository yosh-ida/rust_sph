use crate::util::{self};

use super::util::{kernel::*, particle::Particle, vector::Vector2};

const EFFECT_H: f64 = 1.5;
const W: f64 = 50.0;
const H: f64 = 50.0;
const GAMMA: f64 = 7.0;
const LO_0: f64 = 1.0;
const B: f64 = 1.0;

pub struct SPH {
    pub particles: Vec<Particle>,
    kernel: Poly6,
    gravity: Vector2,
    bw: usize,
    bh: usize,
    size: usize,
    block: Vec<Vec<usize>>,
    eta: f64,
    density0: f64,
    pressure_base: f64,
}

impl SPH {
    pub fn new(r: f64, h: f64, density0: f64, pressure_base: f64) -> SPH {
        let size = ((W / r) as usize) * ((H / r) as usize);
        let mut v = Vec::<Particle>::with_capacity(size);
        let z = Vector2::new(0.0, 0.0);
        for i in 4..(W / r / 2.0 - 4.0) as i32 {
            for j in 4..(H / r - 4.0) as i32 {
                let pos = Vector2::new((i as f64 + 0.5) * r, (j as f64 + 0.5) * r);
                let p = Particle {
                    position: pos,
                    velocity: z,
                    velocity2: z,
                    force: z,
                    accel: z,
                    pressure: 0.0,
                    density: 0.0,
                    mass: r * r * 1000.0,
                    temperature: 1.0,
                    viscosity: 1.0,
                    movable: true,
                    belong: 0,
                };
                v.push(p);
            }
        }

        for i in [
            0.0,
            1.0,
            2.0,
            3.0,
            W / r - 4.0,
            W / r - 3.0,
            W / r - 2.0,
            W / r - 1.0,
        ] {
            for j in 0..(H / r) as i32 {
                let pos = Vector2::new((i + 0.5) * r, (j as f64 + 0.5) * r);
                let p = Particle {
                    position: pos,
                    velocity: z,
                    velocity2: z,
                    force: z,
                    accel: z,
                    pressure: 0.0,
                    density: 0.0,
                    mass: r * r * 1000.0,
                    temperature: 1.0,
                    viscosity: 1.0,
                    movable: false,
                    belong: 0,
                };
                v.push(p);
            }
        }

        for i in 0..(H / r) as i32 {
            for j in [
                0.0,
                1.0,
                2.0,
                3.0,
                H / r - 4.0,
                H / r - 3.0,
                H / r - 2.0,
                H / r - 1.0,
            ] {
                let pos = Vector2::new((i as f64 + 0.5) * r, (j + 0.5) * r);
                let p = Particle {
                    position: pos,
                    velocity: z,
                    velocity2: z,
                    force: z,
                    accel: z,
                    pressure: 0.0,
                    density: 0.0,
                    mass: r * r * 1000.0,
                    temperature: 1.0,
                    viscosity: 1.0,
                    movable: false,
                    belong: 0,
                };
                v.push(p);
            }
        }

        util::console_log(format!("v 0:{}", v.len()).as_str());

        let bw = (W / h + 1.0) as usize;
        let bh = (H / h + 1.0) as usize;
        SPH {
            particles: v,
            kernel: Poly6::new(h),
            gravity: Vector2::new(0.0, 9.8),
            bw,
            bh,
            size: bw * bh,
            block: vec![vec![]; bw * bh],
            eta: 0.01 * h * h,
            density0,
            pressure_base,
        }
    }

    fn grid_check(&mut self) {
        for b in self.block.iter_mut() {
            b.clear();
        }
        util::console_log("grid: area");

        let h = self.kernel.h();
        util::console_log(
            format!(
                "area 0:{0}; belong: 0:{1}",
                self.size - 1,
                self.particles.len() - 1
            )
            .as_str(),
        );
        for (i, p) in self.particles.iter_mut().enumerate() {
            let x = (p.position.x / h) as usize;
            let y = (p.position.y / h) as usize;
            let b = x + self.bw * y;
            util::console_log(format!("{0} in {1} ({2}, {3})", i, b, x, y).as_str());
            self.block[b].push(i);
            p.belong = b;
        }
        util::console_log("grid: fin");
    }

    fn calc_density(&mut self) {
        let w = self.bw as isize;
        let len = self.particles.len();
        let target: [isize; 9] = [-w - 1, -w, -w + 1, -1, 0, 1, w - 1, w, w + 1];
        for i in 0..len {
            let p = &self.particles[i];
            let b = p.belong;

            // util::console_log(format!("density: {}/{}", i, len).as_str());
            let mut a = 0.0;
            let r = &p.position;
            for j2 in target {
                let j = (j2 + b as isize) as usize;
                if j >= self.block.len() {
                    // util::console_log(format!("density: invalid block index {}.", j).as_str());
                    continue;
                }
                for k in &self.block[j] {
                    if *k == i {
                        continue;
                    }
                    if *k >= len {
                        util::console_log("density: invalid particle index.");
                        continue;
                    }
                    let q = &self.particles[*k];
                    a += q.mass * self.kernel.kernel(&(r - &q.position));
                }
            }
            // util::console_log(format!("density: {}/{}", i, len).as_str());
            let pressure = self.density2pressure(&a);
            let mut p = &mut self.particles[i];
            p.density = a;
            p.pressure = pressure;
            // util::console_log(format!("density: {}", i).as_str());
        }
        // util::console_log("density: fin");
    }

    fn density2pressure(&self, density: &f64) -> f64 {
        // B * ((density / LO_0).powf(GAMMA) - 1.0)
        self.pressure_base * (density - self.density0)
    }

    fn calc_power(&mut self) {
        let w = self.bw as isize;
        let target: [isize; 9] = [-w - 1, -w, -w + 1, -1, 0, 1, w - 1, w, w + 1];
        for i in 0..self.particles.len() {
            let p = &self.particles[i];
            if !p.movable {
                continue;
            }
            let r = &p.position;
            let b = p.belong;
            // let mut f = &self.gravity * p.mass;
            let mut f = self.gravity;

            for j2 in target {
                let j = (j2 + b as isize) as usize;

                if j >= self.size {
                    continue;
                }
                for k in self.block[j].iter() {
                    if *k == i {
                        continue;
                    }
                    let q = &self.particles[*k];
                    let dis = r - &q.position;
                    let grad = self.kernel.gradient(&dis);

                    //  圧力項
                    let m = (p.pressure / p.density / p.density
                        + q.pressure / q.density / q.density)
                        * q.mass;
                    let fk = &grad * m;
                    f -= &fk;

                    //  粘性項
                    let m = q.mass * (q.viscosity + p.viscosity) / (q.density * p.density)
                        * (&dis * &grad)
                        / (&dis * &dis + self.eta);
                    let mut fk = &p.velocity - &q.velocity;
                    fk *= m;
                    f += &fk;
                }
            }
            let mut p = &mut self.particles[i];
            p.force = f;
        }
    }

    pub fn step(&mut self, dt: f64) {
        // util::console_log("step");
        self.grid_check();
        util::console_log("grid");
        self.calc_density();
        util::console_log("density");
        self.calc_power();

        util::console_log("power");
        for i in self.particles.iter_mut() {
            if !i.movable {
                continue;
            }
            i.position += &(&(&i.velocity * dt) + &(&i.force * (dt * dt / 2.0)));
            i.velocity += &(&i.force * dt);
            // i.velocity2 += &(&i.force * dt);
            // i.position += &(&i.velocity2 * dt);
            // i.velocity = &i.velocity2 + &(&i.force * (dt / 2.0));
            if i.position.x < 4.5 {
                i.position.x = 4.5;
                i.velocity.x = 0.0;
            } else if i.position.x > W - 4.5 {
                i.position.x = W - 4.5;
                i.velocity.x = 0.0;
            }
            if i.position.y < 4.5 {
                i.position.y = 4.5;
                i.velocity.y = 0.0;
            } else if i.position.y > H - 4.5 {
                i.position.y = H - 4.5;
                i.velocity.y = 0.0;
            }
        }
    }
}

#[test]
fn test() {
    let mut sph = SPH::new(1.0, 1.5, 200.0, 100.0);
    for _i in 0..10 {
        sph.step(0.01);
    }
}
