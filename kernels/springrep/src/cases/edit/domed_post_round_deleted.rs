//! A post of radius 4 from z = 0 to z = 6 capped by the ball of radius 5 about (0, 0, 3) above
//! z = 6, the rim where they meet rounded to radius 1; the round then deleted, the cylinder and
//! the dome meeting again in the circle of radius 4 at z = 6.
use springrep::blend::fillet;
use springrep::boolean::{difference, union};
use springrep::math::Vec3;
use springrep::primitive::{block, cylinder, sphere};
use springrep::topo::Brep;

use crate::shared::edit::delete_feature;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let post = cylinder(&at([0., 0., 0.]), 4., 6., "post").map_err(e)?;
    let ball = sphere(&at([0., 0., 3.]), 5., "dome").map_err(e)?;
    let below = block(&at([-6., -6., -3.]), Vec3::new(12., 12., 9.), "cut").map_err(e)?;
    let dome = difference(&ball, &below).map_err(e)?;
    let part = union(&post, &dome).map_err(e)?;
    let rounded = fillet(&part, &[("post.side", "dome.skin")], 1., "round").map_err(e)?;
    // every face of the round, however many its seams split it into
    delete_feature(&rounded, "round")
}
