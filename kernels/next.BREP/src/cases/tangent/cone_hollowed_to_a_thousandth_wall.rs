//! A cone, its base of radius 5 at z = 0 and its apex 10 above it on the z axis, less a cone of base
//! radius 4.999 with the same base plane and the same apex. The first less the second.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_cone_brep, BrepSolid};

const OUTER: f64 = 5.0;
const INNER: f64 = 4.999;
const HEIGHT: f64 = 10.0;

pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let outer = make_cone_brep(p(0.0, 0.0, 0.0), z, OUTER, 0.0, HEIGHT)?;
    let inner = make_cone_brep(p(0.0, 0.0, 0.0), z, INNER, 0.0, HEIGHT)?;
    Ok(cut(&outer, &inner)?)
}
