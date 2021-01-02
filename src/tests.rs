// Unit testing happens here.

use super::planets::*;

#[test]
fn test_distance() {

    let mut earth = Planet::new(0.0, 0.0, 3.0, 0.0, 4.0, 0.0);

    let mut sun = Planet::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

    assert_eq!(sun.distance(&earth), 5.0);
    assert_eq!(earth.distance(&sun), 5.0);

    earth.x = 6.0;
    earth.y = 14.0;

    sun.x = 1.0;
    sun.y = 2.0;

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