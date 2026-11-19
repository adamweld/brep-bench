//! The frustum from radius 6 at z = 0 to 3 at z = 6, its top rim rounded to radius 1; the round
//! then deleted, the side and the top meeting again.
use springrep::blend::fillet;
use springrep::primitive::cone;
use springrep::topo::Brep;

use crate::shared::edit::delete_feature;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let frustum = cone(&at([0., 0., 0.]), 6., 3., 6., "frustum").map_err(e)?;
    let rounded = fillet(&frustum, &[("frustum.top", "frustum.side")], 1., "round").map_err(e)?;
    delete_feature(&rounded, "round")
}
