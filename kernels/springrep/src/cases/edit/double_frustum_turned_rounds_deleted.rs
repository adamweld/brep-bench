//! Two frustums on the z axis made one solid, radius 3 at z = 0 out to 5 at z = 4 and in to 2 at
//! z = 8, their ridge rounded to radius 1; the round then deleted, the cones meeting again in the
//! ridge.
//! The upper frustum is made turned 2 radians about the z axis, so the ridge is two edges and its
//! round two faces.
use springrep::blend::fillet;
use springrep::boolean::union;
use springrep::primitive::cone;
use springrep::topo::Brep;

use crate::shared::edit::{delete_feature, turned};
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let lower = cone(&at([0., 0., 0.]), 3., 5., 4., "lower").map_err(e)?;
    let upper = cone(&at([0., 0., 4.]), 5., 2., 4., "upper").map_err(e)?;
    let part = union(&lower, &turned(upper, 2.0)).map_err(e)?;
    let rounded = fillet(&part, &[("lower.side", "upper.side")], 1., "round").map_err(e)?;
    delete_feature(&rounded, "round")
}
