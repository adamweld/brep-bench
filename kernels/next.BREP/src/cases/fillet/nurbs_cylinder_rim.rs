//! A cylinder of radius 5 on the z axis from z = 0 to z = 10 whose side is one NURBS face, the
//! rational quadratic circle times a line (`solids::nurbs::nurbs_cylinder`, its seam in the half
//! plane y = 0, x ≥ 0), the circular edge where the side meets the top rounded to radius 1 by the
//! kernel's fillet. The edge is selected by a point on it off the seam.
use crate::solids::fillet::on_circle;
use crate::solids::nurbs::{nurbs_cylinder, CIRCLE_FROM_X};
use crate::Refusal;
use brep_kernel::{fillet_edges, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let cylinder = nurbs_cylinder(5.0, 0.0, 10.0, &CIRCLE_FROM_X)?;
    Ok(fillet_edges(&cylinder, &[on_circle(5.0, 10.0, 1.0)], None, 1.0, false, None)?)
}
