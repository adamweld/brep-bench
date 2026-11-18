//! SolidCraft's Shell (`shell`) hollows a body inward to a wall of the thickness, opening the faces
//! picked by points on them.
use crate::solids::*;
use solidcore_record::Seg2;
use std::f64::consts::FRAC_PI_2;

/// solidcraft-kernel has no cone, and no round of an apex: the solid is its half section turned about
/// z, as SolidCraft's own cone feature revolves its section: the cone's side up to the circle
/// where the ball of radius 1 about (0, 0, 12 − √5) touches it, then that ball's arc over to the
/// axis.
pub fn build() -> R<Body> {
    let line = |a: (f64, f64), b: (f64, f64)| Seg2::Line { a: v2(a.0, a.1), b: v2(b.0, b.1) };
    let s5 = 5f64.sqrt();
    let center = 12.0 - s5;
    // the point of tangency is (2, 1)/√5 from the ball's center
    let start = 0.5f64.atan();
    let section = Loop2 {
        segs: vec![
            line((0.0, 0.0), (6.0, 0.0)),
            line((6.0, 0.0), (2.0 / s5, 12.0 - 4.0 / s5)),
            Seg2::Arc { center: v2(0.0, center), radius: 1.0, start, sweep: FRAC_PI_2 - start },
            line((0.0, center + 1.0), (0.0, 0.0)),
        ],
    };
    let cone = revolve_z(Region2 { outer: section, holes: vec![] }, 360.0)?;
    // the base, picked off the axis
    solidcraft_kernel::shell(&cone, &[v3(1.0, 1.0, 0.0)], 1.5)
}
