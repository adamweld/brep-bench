//! Two cylinders of radius 3 and length 20, both centred on c = (600000, −800000, 0), the first
//! along (2, 3, 6)/7 and the second along (3, −6, 2)/7, each running from c − 10u to c + 10u along
//! its own direction u: the Steinmetz bicylinder common to both, a million from the origin.
use crate::solids::boolean::common;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, BrepSolid, Vec3};

/// The cylinder of radius 3 along `direction` (scaled to length 7) from c − 10u to c + 10u.
fn cylinder(direction: Vec3) -> Result<BrepSolid, String> {
    let centre = p(600000.0, -800000.0, 0.0);
    let u = direction.scale(1.0 / 7.0);
    make_cylinder_brep(centre.sub(u.scale(10.0)), u, 3.0, 20.0)
}

pub fn build() -> Result<BrepSolid, Refusal> {
    let first = cylinder(p(2.0, 3.0, 6.0))?;
    let second = cylinder(p(3.0, -6.0, 2.0))?;
    Ok(common(&first, &second)?)
}
