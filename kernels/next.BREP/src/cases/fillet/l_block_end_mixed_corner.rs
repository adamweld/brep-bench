//! An L-block, the union of the box from the origin to (20, 10, 6) and the box from the origin to
//! (6, 10, 20), its end y = 0 rounded across the inside corner as the case composes it, one round
//! after another: first the concave edge from (6, 0, 6) to (6, 10, 6) to radius 2, then the end
//! face's edge chain that makes (up the upright, round the concave round's quarter circle, along
//! the foot) to radius 2 in one fillet. Each edge is selected by a point on it.
use crate::solids::fillet::union;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{fillet_edges, make_box_brep, BrepSolid};
use std::f64::consts::FRAC_1_SQRT_2;

pub fn build() -> Result<BrepSolid, Refusal> {
    let foot = make_box_brep(p(0.0, 0.0, 0.0), 20.0, 10.0, 6.0)?;
    let upright = make_box_brep(p(0.0, 0.0, 0.0), 6.0, 10.0, 20.0)?;
    let block = union(&foot, &upright)?;
    let concave = fillet_edges(&block, &[p(6.0, 5.0, 6.0)], None, 2.0, false, None)?;
    // the chain on y = 0: up the upright, the quarter circle of radius 2 about (8, 0, 8) at 45°,
    // along the foot
    let arc = p(8.0 - 2.0 * FRAC_1_SQRT_2, 0.0, 8.0 - 2.0 * FRAC_1_SQRT_2);
    let chain = [p(6.0, 0.0, 14.0), arc, p(14.0, 0.0, 6.0)];
    Ok(fillet_edges(&concave, &chain, None, 2.0, false, None)?)
}
