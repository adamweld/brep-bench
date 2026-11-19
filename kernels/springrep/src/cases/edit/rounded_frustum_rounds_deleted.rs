//! The frustum from radius 6 at z = 0 to 3 at z = 6, both rims rounded to radius 3/2; both rounds
//! then deleted together, giving the plain frustum back.
use springrep::blend::fillet;
use springrep::primitive::cone;
use springrep::topo::Brep;

use crate::shared::edit::delete_feature;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let frustum = cone(&at([0., 0., 0.]), 6., 3., 6., "frustum").map_err(e)?;
    let rims = [("frustum.top", "frustum.side"), ("frustum.bottom", "frustum.side")];
    let rounded = fillet(&frustum, &rims, 1.5, "round").map_err(e)?;
    delete_feature(&rounded, "round")
}
