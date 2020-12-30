[![Build Status](https://www.travis-ci.org/toasthunter/orbsim.svg?branch=master)](https://www.travis-ci.org/toasthunter/orbsim)

# 2D Physics simulation involving Sun-Earth orbitals
## Powered by [Piston](https://github.com/PistonDevelopers/piston)

This program visually simulates how the Earth orbits around the Sun, with realistic gravitational forces calculated from realistic masses.

The resolution of the movement is dependent on the timescale(dt). If the timescale is smaller, the movement will be smoother and more accurate(because this program is based on differential equations, whose resolution is dependent on dt).

In this simulation, for the sake of simplicity, the Sun is *fixed in place*, because the gravitational force of the Earth exerted on the Sun is negligibly small.