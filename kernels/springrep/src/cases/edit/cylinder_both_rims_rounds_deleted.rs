//! A cylinder of radius 6 from z = 0 to z = 10, both rims rounded to radius 2; both rounds then
//! deleted together, giving the plain cylinder back.
use springrep::blend::fillet;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::edit::delete_feature;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let post = cylinder(&at([0., 0., 0.]), 6., 10., "post").map_err(e)?;
    let rims = [("post.end", "post.side"), ("post.start", "post.side")];
    let rounded = fillet(&post, &rims, 2., "round").map_err(e)?;
    delete_feature(&rounded, "round")
}
