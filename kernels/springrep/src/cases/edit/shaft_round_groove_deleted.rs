//! A shaft of radius 5 from z = 0 to z = 20 with a round-bottomed groove, the torus of tube 1.5 about
//! its rim at z = 10, cut into it; the groove's face then deleted, the cylinder running on across.
use springrep::boolean::difference;
use springrep::primitive::torus;
use springrep::topo::Brep;

use crate::shared::edit::{delete_named, shaft};
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let groove = torus(&at([0., 0., 10.]), 5., 1.5, "groove").map_err(e)?;
    let grooved = difference(&shaft()?, &groove).map_err(e)?;
    delete_named(&grooved, &["groove.tube"])
}
