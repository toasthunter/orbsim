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

    fn new(sun: Planet, earth: Planet, grav_coefficent: f64, timescale: f64) -> Self {
        Self {sun, earth, grav_coefficent, time: 0.0, timescale}
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