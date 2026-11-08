//! The plate from the origin to (40, 30, 5) joined to the square boss from (12, 7, 5) to
//! (28, 23, 15), rounded as the case composes it, one round after another: first the boss's four
//! upright edges to radius 3, then the root they leave, one closed chain of four lines and four
//! quarter circles, to radius 2 in one fillet. Each edge is selected by a point on it.
use crate::solids::fillet::union;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{fillet_edges, make_box_brep, BrepSolid};
use std::f64::consts::FRAC_1_SQRT_2;

pub fn build() -> Result<BrepSolid, Refusal> {
    let plate = make_box_brep(p(0.0, 0.0, 0.0), 40.0, 30.0, 5.0)?;
    let boss = make_box_brep(p(12.0, 7.0, 5.0), 16.0, 16.0, 10.0)?;
    let joined = union(&plate, &boss)?;
    let uprights = [p(12.0, 7.0, 10.0), p(28.0, 7.0, 10.0), p(28.0, 23.0, 10.0),
        p(12.0, 23.0, 10.0)];
    let rounded = fillet_edges(&joined, &uprights, None, 3.0, false, None)?;
    // the root: each side's middle, and each corner's quarter circle of radius 3 at 45°, about the
    // upright round's axis 3 inside the boss's corner
    let mut root = vec![p(20.0, 7.0, 5.0), p(28.0, 15.0, 5.0), p(20.0, 23.0, 5.0),
        p(12.0, 15.0, 5.0)];
    let k = 3.0 * FRAC_1_SQRT_2;
    for (cx, cy, sx, sy) in [(15.0, 10.0, -1.0, -1.0), (25.0, 10.0, 1.0, -1.0),
        (25.0, 20.0, 1.0, 1.0), (15.0, 20.0, -1.0, 1.0)] {
        root.push(p(cx + sx * k, cy + sy * k, 5.0));
    }
    Ok(fillet_edges(&rounded, &root, None, 2.0, false, None)?)
}
