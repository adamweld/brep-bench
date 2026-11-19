//! The plate of radius 8 from z = −4 to z = 0 with a post of radius 3 up to z = 8 on it, the
//! concave edge at its foot filleted to radius 2, grown by 3: the fillet vanishes and the moved
//! post and plate meet in a sharp concave edge.
//! springrep 1.0.1 refuses it: its offset cannot follow the concave fillet moved past its radius
//! ("an offset … closes a torus to nothing").
use springrep::blend::fillet;
use springrep::boolean::union;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::offset::grown;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let plate = cylinder(&at([0., 0., -4.]), 8., 4., "plate").map_err(e)?;
    let post = cylinder(&at([0., 0., 0.]), 3., 8., "post").map_err(e)?;
    let joined = union(&plate, &post).map_err(e)?;
    let filleted = fillet(&joined, &[("plate.end", "post.side")], 2., "foot").map_err(e)?;
    grown(&filleted, 3.)
}
