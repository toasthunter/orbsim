mod planets;
mod parser;

use planets::*;

#[cfg(test)]
mod tests;

fn main() {

    let mut system = parser::parse_init("init.json")
        .expect("Error parsing initial parameters");

    let mut window: PistonWindow = WindowSettings::new("orbsim v0.2.0", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            system.render(c, g);
            system.step_time();
        });
    }

}
