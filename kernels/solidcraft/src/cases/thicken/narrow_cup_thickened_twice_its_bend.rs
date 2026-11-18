//! A narrow cup, a floor, a bend of radius 2 and a wall meeting tangentially, a sheet turned about
//! the z axis by SolidCraft's Revolve Surface (`revolve_sheet`), thickened by 4 to its inside by
//! Thicken (`thicken`), twice the bend's radius: the bend's offset turns inside out and leaves
//! the solid, the floor's and the wall's meeting at a concave edge instead.
use crate::solids::*;
use solidcore_record::Seg2;
use std::f64::consts::{FRAC_PI_2, TAU};

pub fn build() -> R<Body> {
    let line = |a: (f64, f64), b: (f64, f64)| Seg2::Line { a: v2(a.0, a.1), b: v2(b.0, b.1) };
    // the profile in (ρ, z): the floor out from the axis, the quarter circle about (3, 2) from
    // below it round to (5, 2), the wall up
    let profile = vec![
        line((0.0, 0.0), (3.0, 0.0)),
        Seg2::Arc { center: v2(3.0, 2.0), radius: 2.0, start: -FRAC_PI_2, sweep: FRAC_PI_2 },
        line((5.0, 2.0), (5.0, 10.0)),
    ];
    let cup = solidcraft_kernel::revolve_sheet(&xz(), &[profile], Vec2::ZERO, Vec2::Y, 0.0, TAU)?;
    // the turned faces' normals point out of the cup, away from the axis: against them
    solidcraft_kernel::thicken(&cup, -4.0, false)
}
