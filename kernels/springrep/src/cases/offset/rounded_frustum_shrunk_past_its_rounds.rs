//! The cone frustum from radius 6 at z = 0 to radius 3 at z = 6, both rims rounded to radius 1.5,
//! shrunk by 2, past the rounds' radius: the rounds vanish and the frustum shrinks with sharp
//! edges. springrep 1.0.1 refuses it: its face offset cannot follow a round moved past its radius
//! ("an offset … closes a torus to nothing").
use springrep::topo::Brep;

use crate::shared::offset::{rounded_frustum, shrunk};

pub fn build() -> Result<Brep, String> {
    shrunk(&rounded_frustum()?, 2.)
}
