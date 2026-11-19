//! The plate of radius 8 from z = −2 to z = 0 with the ball of radius 3 about (0, 0, 1) standing in
//! it, the concave edge where they meet filleted to radius 1, grown by 2, past the fillet's
//! radius: the fillet vanishes and the moved top and ball meet in a sharp concave edge. springrep
//! 1.0.1 refuses it: its offset cannot follow the concave fillet moved past its radius ("an offset
//! … closes a torus to nothing").
use springrep::topo::Brep;

use crate::shared::offset::{ball_on_plate, grown};

pub fn build() -> Result<Brep, String> {
    grown(&ball_on_plate(1.)?, 2.)
}
