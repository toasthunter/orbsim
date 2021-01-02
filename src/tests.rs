// Unit testing happens here.

use super::planets::*;

#[test]
fn test_distance() {

    let mut earth = Planet::new(0.0, 0.0, 3.0, 0.0, 4.0, 0.0);

    let mut sun = Planet::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

    assert_eq!(sun.distance(&earth), 5.0);
    assert_eq!(earth.distance(&sun), 5.0);

    earth.pos.x = 6.0;
    earth.pos.y = 14.0;

    sun.pos.x = 1.0;
    sun.pos.y = 2.0;

    assert_eq!(earth.distance(&sun), 13.0);
    assert_eq!(sun.distance(&earth), 13.0);

}

#[test]
fn test_grav_accel() {

    let system = PlanetarySystem::new(
        Planet::new(1000.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        Planet::new(50.0, 0.0, 7.0, 0.0, 24.0, 0.0),
        0.1,
        0.0
    );

    assert_eq!(system.grav_accel(), (-0.0448, -0.1536));

}