//! A disk plate of radius 8 with a ball of radius 3 about (0, 0, 1) standing out of it, the groove
//! at its root filled by a frustum, the bevel; the bevel then deleted, the plate's top and the
//! ball meeting again in a sharp edge.
use springrep::boolean::union_all;
use springrep::primitive::{cone, cylinder, sphere};
use springrep::topo::Brep;

use crate::shared::edit::delete_named;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let plate = cylinder(&at([0., 0., -2.]), 8., 2., "plate").map_err(e)?;
    let ball = sphere(&at([0., 0., 1.]), 3., "ball").map_err(e)?;
    let bevel = cone(&at([0., 0., 0.]), 4., 3., 1., "bevel").map_err(e)?;
    let part = union_all(&[&plate, &ball, &bevel]).map_err(e)?;
    delete_named(&part, &["bevel.side"])
}
