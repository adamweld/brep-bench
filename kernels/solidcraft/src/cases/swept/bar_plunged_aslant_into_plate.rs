//! A plate, the box from (0, −5, 0) to (20, 5, 4), with a pocket cut into its underside by a bar
//! of square section stood on a corner (6 ≤ x ≤ 14 and |y| + |z + 6| ≤ 1) slid without turning by
//! 8 along (0.3, 0, √0.91), stopping inside the plate.
//!
//! solidcraft-kernel has no sweep of a solid, so the slide is built, as in `box-slid-aslant`, from
//! the bar and its faces by elementary geometry. A point the bar passes over that is outside the
//! bar where it starts is reached first by a face it leads with, one whose outward normal points
//! along the slide: its end x = 14 and the two faces of its section above its middle line, facing
//! (0, ±1, 1). So the solid is the bar where it starts and those three faces each slid the whole
//! way, a slanted prism apiece. The kernel's `sweep` along a straight path translates its profile.
use crate::solids::*;
use solidcraft_kernel::{PathSeg, sweep};

pub fn build() -> R<Body> {
    let plate = block((0.0, -5.0, 0.0), (20.0, 5.0, 4.0))?;
    let slide = v3(0.3, 0.0, 0.91f64.sqrt()) * 8.0;
    // the section, in (y, z), on the planes square to x
    let section = polygon(&[(0.0, -7.0), (1.0, -6.0), (0.0, -5.0), (-1.0, -6.0)]);
    let end = |x: f64| Plane { origin: v3(x, 0.0, 0.0), x: Vec3::Y, y: Vec3::Z };
    let mut swept = extrude(&end(6.0), section.clone(), 0.0, 8.0)?;
    // the leading faces: the end x = 14, and the upper faces of the section, each from its lower
    // corner (y = ±1, z = −6) up to the top corner (0, −5), along the bar's length
    let up = |y: f64| v3(0.0, -y, 1.0) * (1.0 / 2f64.sqrt());
    let faces = [
        (end(14.0), section),
        (Plane { origin: v3(6.0, 1.0, -6.0), x: Vec3::X, y: up(1.0) }, rect(0.0, 0.0, 8.0, 2f64.sqrt())),
        (Plane { origin: v3(6.0, -1.0, -6.0), x: Vec3::X, y: up(-1.0) }, rect(0.0, 0.0, 8.0, 2f64.sqrt())),
    ];
    for (plane, face) in &faces {
        let path = [PathSeg::Line { a: plane.origin, b: plane.origin + slide }];
        swept = union(&swept, &sweep(plane, face, &path)?)?;
    }
    cut(&plate, &swept)
}
