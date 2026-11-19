//! The plate of radius 8 from z = −2 to z = 0 with a cone boss from radius 4 at z = 0 to radius 1
//! at z = 6 on it, the concave edge where they meet filleted to radius 1, grown by 2, past the
//! fillet's radius: the fillet vanishes and the moved top and cone meet in a sharp concave edge.
//! springrep 1.0.1 refuses it: its offset cannot follow the concave fillet moved past its radius
//! ("an offset … closes a torus to nothing").
use springrep::topo::Brep;

use crate::shared::offset::{cone_boss, grown};

pub fn build() -> Result<Brep, String> {
    grown(&cone_boss(1.)?, 2.)
}
