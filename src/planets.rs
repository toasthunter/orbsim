// Contains the necessary definitions of the planet and planetary system data structures
// and their respective methods.

pub struct PlanetarySystem {

    sun: Planet,
    earth: Planet,
    grav_coefficent: f64,
    time: f64,
    timescale: f64,

}

impl PlanetarySystem {

    pub fn new(sun: Planet, earth: Planet, grav_coefficent: f64, timescale: f64) -> Self {
        Self {sun, earth, grav_coefficent, time: 0.0, timescale}
    }
    
    // Calculates the gravitational acceleration of the Earth in x and y directions with respect to the Sun
    // The Sun is fixed in place for the sake of simplicity
    fn grav_accel(&self) -> (f64, f64) {

        (-self.grav_coefficent * self.sun.mass * (self.earth.x - self.sun.x).abs() / self.earth.distance(&self.sun).powf(3.0),
        -self.grav_coefficent * self.sun.mass * (self.earth.y - self.sun.y).abs() / self.earth.distance(&self.sun).powf(3.0))

    }

    // Steps the time forward in the system, and moves the planets accordingly.
    fn step_time(&mut self) {

        let (gx, gy) = self.grav_accel();

        let vx = self.earth.vx + gx * self.timescale;
        let x = self.earth.x * self.earth.vx * self.timescale;

        let vy = self.earth.vy + gy * self.timescale;
        let y = self.earth.y + self.earth.vy * self.timescale;

        self.earth.x = x;
        self.earth.y = y;
        self.earth.vx = vx;
        self.earth.vy = vy;

        self.time += self.timescale;

    }

}

pub struct Planet {

    pub mass: f64,
    pub radius: f64,
    pub x: f64,
    pub vx: f64,
    pub y: f64,
    pub vy: f64,

}

impl Planet {

    pub fn new(mass: f64, radius: f64, x: f64, vx: f64, y: f64, vy: f64) -> Self {
        Self {mass, radius, x, vx, y, vy}
    }

    pub fn distance(&self, other: &Self) -> f64 {
        ((self.x - other.x).abs().powf(2.0) + (self.y - other.y).abs().powf(2.0)).sqrt()
    }

}