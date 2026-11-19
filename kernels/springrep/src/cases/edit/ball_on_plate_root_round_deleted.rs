//! A disk plate of radius 8 with a ball of radius 3 about (0, 0, 1) standing out of it, the
//! concave edge at its root rounded to radius 1; the round then deleted, the plate's top and the
//! ball meeting again in a sharp edge.
use springrep::blend::fillet;
use springrep::boolean::union;
use springrep::primitive::{cylinder, sphere};
use springrep::topo::Brep;

use crate::shared::edit::delete_feature;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let plate = cylinder(&at([0., 0., -2.]), 8., 2., "plate").map_err(e)?;
    let ball = sphere(&at([0., 0., 1.]), 3., "ball").map_err(e)?;
    let part = union(&plate, &ball).map_err(e)?;
    let rounded = fillet(&part, &[("plate.end", "ball.skin")], 1., "round").map_err(e)?;
    delete_feature(&rounded, "round")
}
