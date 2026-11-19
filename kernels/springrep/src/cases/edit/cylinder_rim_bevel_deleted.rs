//! A cylinder of radius 6 from z = 0 to z = 10, its top rim beveled 2; the bevel then deleted,
//! giving the plain cylinder back.
use springrep::blend::chamfer;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::edit::delete_feature;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let post = cylinder(&at([0., 0., 0.]), 6., 10., "post").map_err(e)?;
    let beveled = chamfer(&post, &[("post.end", "post.side")], 2., "bevel").map_err(e)?;
    delete_feature(&beveled, "bevel")
}
