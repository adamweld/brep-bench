//! The tee of the cylinder of radius 6 about the x axis from x = −20 to x = 20 and the cylinder of
//! radius 3 about the z axis from z = 0 to z = 15, joined, the saddle-shaped junction where their
//! sides meet rounded to radius 1 by the kernel's fillet. The junction is selected by its point
//! over the direction 45° between +x and +y.
use crate::solids::fillet::union;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{fillet_edges, make_cylinder_brep, BrepSolid};
use std::f64::consts::FRAC_1_SQRT_2;

pub fn build() -> Result<BrepSolid, Refusal> {
    let bar = make_cylinder_brep(p(-20.0, 0.0, 0.0), p(1.0, 0.0, 0.0), 6.0, 40.0)?;
    let stem = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 3.0, 15.0)?;
    let tee = union(&bar, &stem)?;
    let (x, y) = (3.0 * FRAC_1_SQRT_2, 3.0 * FRAC_1_SQRT_2);
    let on_junction = p(x, y, (36.0 - y * y).sqrt());
    Ok(fillet_edges(&tee, &[on_junction], None, 1.0, false, None)?)
}
