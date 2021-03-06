use std::time::Instant;

use cgmath::Vector3;
use tiny_skia::*;

mod line;
use line::*;

use polynomial_optics::*;

fn main() {
    // let mut pixmap = Pixmap::new(500, 500).unwrap();

    // let line = Line::new_dashed(10., 10., 400., 300.);
    // line.draw(&mut pixmap);

    // pixmap.save_png("image.png").unwrap();

    let f = Polynom2d {
        coefficients: [[3.0, 2.0], [1.0, 4.0]],
    };

    let g = Polynom2d {
        coefficients: [[382., 47.], [3.86285, 1.0]],
    };

    println!("          f(x) = {}", f);
    println!("          g(x) = {}", g);
    println!("       f(3, 2) = {}", f.eval(3., 2.));

    println!("     f(x)+g(x) = {}", f + g);
    println!("     f(x)-g(x) = {}", f - g);

    println!("f(x)+g(x)-g(x) = {}", f + g - g);

    assert_eq!(f, f + g - g);

    let part = Monomial {
        coefficient: 1.0,
        exponents: [2, 4, 3],
    };
    let part2 = Monomial {
        coefficient: 0.5,
        exponents: [1, 3, 0],
    };
    println!("part: {}", part);
    println!("part2: {}", part2);

    let pol = Polynomial::new(vec![part, part2]);
    println!("pol: {}", pol);

    println!("multiplied with itself: {}", &pol * &pol);

    // ray tracing
    // let mut pixmap = Pixmap::new(400, 200).unwrap();
    let mut pixmap = Pixmap::new(4000, 2000).unwrap();
    pixmap.fill(Color::from_rgba8(0, 0, 0, 255));

    let mut line = Line::new(10., 10., 400., 300.);

    let radius = 3.0;
    let lens_entry = Element {
        radius,
        glass: Glass {
            ior: 1.5,
            coating: (),
        },
        position: -2.0,
        entry: true,
        spherical: true,
    };
    let lens_exit_pos = 1.0;
    let lens_exit = Element {
        radius,
        glass: Glass {
            ior: 1.5,
            coating: (),
        },
        position: lens_exit_pos,
        entry: false,
        spherical: true,
    };
    line.width = 3.0;
    // lens entry
    line.draw_circle(&mut pixmap, -radius as f32 - 2.0, 0., radius as f32);

    // lens exit
    line.color = Color::from_rgba8(127, 127, 127, 255);
    line.draw_circle(
        &mut pixmap,
        (-3.) * radius as f32 + lens_exit_pos as f32,
        0.,
        radius as f32,
    );
    line.width = 0.1;

    println!("lens: {:?}", lens_entry);
    //println!("ray: {:?}", ray);

    let lens = Lens::new(vec![lens_entry, lens_exit]);
    lens.draw(&mut pixmap);

    pixmap.save_png("image.png").unwrap();
}
