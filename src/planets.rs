// Contains the necessary definitions of the planet and planetary system data structures
// and their respective methods.

extern crate piston_window;

pub use piston_window::*;

pub const WIDTH: f64 = 512.0;
pub const HEIGHT: f64 = 512.0;

pub struct PlanetarySystem {

    pub sun: Planet,
    pub earth: Planet,
    pub grav_coefficent: f64,
    pub time: f64,
    pub timescale: f64,

}

impl PlanetarySystem {

    pub fn new(sun: Planet, earth: Planet, grav_coefficent: f64, timescale: f64) -> Self {
        Self {sun, earth, grav_coefficent, time: 0.0, timescale}
    }
    
    // Calculates the gravitational acceleration of the Earth in x and y directions with respect to the Sun
    // The Sun is fixed in place for the sake of simplicity
    pub fn grav_accel(&self) -> (f64, f64) {

        (
        self.grav_coefficent * self.sun.mass * (self.sun.pos.x - self.earth.pos.x) / self.earth.distance(&self.sun).powf(3.0),
        self.grav_coefficent * self.sun.mass * (self.sun.pos.y - self.earth.pos.y) / self.earth.distance(&self.sun).powf(3.0)
        )

    }

    // Steps the time forward in the system, and moves the planets accordingly.
    pub fn step_time(&mut self) {

        let (gx, gy) = self.grav_accel();

        self.earth.vel.x += gx * self.timescale;
        self.earth.vel.y += gy * self.timescale;

        self.earth.pos.x += self.earth.vel.x * self.timescale;
        self.earth.pos.y += self.earth.vel.y * self.timescale;

        self.time += self.timescale;

    }

    // Render planets onto the screen
    pub fn render(&self, ctx: Context, gfx: &mut G2d) {

        clear([1.0, 1.0, 1.0, 1.0], gfx);

        ellipse([1.0, 0.64, 0.0, 1.0],
            [self.sun.pos.x - self.sun.radius / 2.0, HEIGHT - self.sun.pos.y - self.sun.radius / 2.0, self.sun.radius, self.sun.radius],
            ctx.transform,
            gfx);

        ellipse([0.0, 0.0, 1.0, 1.0],
            [self.earth.pos.x - self.earth.radius / 2.0, HEIGHT - self.earth.pos.y - self.earth.radius / 2.0, self.earth.radius, self.earth.radius],
            ctx.transform,
            gfx);

        line([1.0, 0.0, 0.0, 0.5],
            1.0,
            [self.sun.pos.x, HEIGHT - self.sun.pos.y, self.earth.pos.x, HEIGHT - self.earth.pos.y],
            ctx.transform,
            gfx);

    }

}

pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {

    fn new(x: f64, y: f64) -> Self {
        Self {x, y}
    }

    fn diff(&self, other: &Self) -> f64 {
        ((self.x - other.x).powf(2.0) + (self.y - other.y).powf(2.0)).sqrt() 
    }

}

pub struct Planet {

    pub mass: f64,
    pub radius: f64,
    pub pos: Vector,
    pub vel: Vector,

}

impl Planet {

    pub fn new(mass: f64, radius: f64, x: f64, vx: f64, y: f64, vy: f64) -> Self {
        Self {mass, radius, pos: Vector::new(x, y), vel: Vector::new(vx, vy)}
    }

    pub fn distance(&self, other: &Self) -> f64 {
        self.pos.diff(&other.pos)
    }

}