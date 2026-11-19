//! Two cylinders of radius 0.003 and length 0.02 crossing at right angles, along the x axis from
//! x = −0.01 to 0.01 and along the y axis from y = −0.01 to 0.01: the Steinmetz bicylinder common
//! to both, a few microns across if a unit is a millimetre.
use crate::solids::boolean::common;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let along_x = make_cylinder_brep(p(-0.01, 0.0, 0.0), p(1.0, 0.0, 0.0), 0.003, 0.02)?;
    let along_y = make_cylinder_brep(p(0.0, -0.01, 0.0), p(0.0, 1.0, 0.0), 0.003, 0.02)?;
    Ok(common(&along_x, &along_y)?)
}
