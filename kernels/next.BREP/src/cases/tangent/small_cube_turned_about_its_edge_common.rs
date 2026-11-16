//! A small cube of side 0.001 from the origin and a copy of itself turned 1.2° about the z axis, the
//! line of one of its own edges, their common part: a prism over the quadrilateral the two squares
//! share.
use crate::solids::boolean::{common, turned_about_z};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, BrepSolid};

const SIDE: f64 = 0.001;

pub fn build() -> Result<BrepSolid, Refusal> {
    let cube = make_box_brep(p(0.0, 0.0, 0.0), SIDE, SIDE, SIDE)?;
    let turned = turned_about_z(&cube, 1.2f64.to_radians())?;
    Ok(common(&cube, &turned)?)
}
