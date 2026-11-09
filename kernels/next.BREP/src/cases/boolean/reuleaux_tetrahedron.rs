//! The solid common to four balls of radius 10 centred at the vertices of a regular tetrahedron of
//! side 10: a Reuleaux tetrahedron.
use crate::solids::boolean::common;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_sphere_brep, BrepSolid, Vec3};

fn ball(centre: Vec3) -> Result<BrepSolid, String> {
    make_sphere_brep(centre, 10.0, p(0.0, 0.0, 1.0))
}

pub fn build() -> Result<BrepSolid, Refusal> {
    let root3 = 3.0_f64.sqrt();
    let two = common(&ball(p(0.0, 0.0, 0.0))?, &ball(p(10.0, 0.0, 0.0))?)?;
    let three = common(&two, &ball(p(5.0, 5.0 * root3, 0.0))?)?;
    let apex = p(5.0, 5.0 * root3 / 3.0, 10.0 * (2.0_f64 / 3.0).sqrt());
    Ok(common(&three, &ball(apex)?)?)
}
