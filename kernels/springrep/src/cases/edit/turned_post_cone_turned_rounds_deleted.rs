//! A turned post: the cylinder of radius 5 from z = 0 to z = 6 and on it the frustum from radius 5
//! to radius 2 at z = 10, its shoulder rounded to radius 1; the round then deleted, the cylinder
//! and the cone meeting again in the shoulder.
//! The cone is made turned 1 radian about the z axis, so the shoulder is two edges and its round
//! two faces.
use springrep::blend::fillet;
use springrep::boolean::union;
use springrep::primitive::{cone, cylinder};
use springrep::topo::Brep;

use crate::shared::edit::{delete_feature, turned};
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let post = cylinder(&at([0., 0., 0.]), 5., 6., "post").map_err(e)?;
    let top = cone(&at([0., 0., 6.]), 5., 2., 4., "cone").map_err(e)?;
    let part = union(&post, &turned(top, 1.0)).map_err(e)?;
    let rounded = fillet(&part, &[("post.side", "cone.side")], 1., "round").map_err(e)?;
    delete_feature(&rounded, "round")
}
