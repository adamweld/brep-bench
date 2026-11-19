//! The cone frustum from radius 6 at z = 0 to radius 3 at z = 6, its top rim rounded to radius
//! 1.5, shrunk by 2: the round vanishes and the frustum shrinks with sharp edges.
//! springrep 1.0.1 refuses it: its face offset cannot follow a round moved past its radius ("an
//! offset … closes a torus to nothing").
use springrep::blend::fillet;
use springrep::primitive::cone;
use springrep::topo::Brep;

use crate::shared::offset::shrunk;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let frustum = cone(&at([0., 0., 0.]), 6., 3., 6., "frustum").map_err(e)?;
    let rounded = fillet(&frustum, &[("frustum.top", "frustum.side")], 1.5, "round").map_err(e)?;
    shrunk(&rounded, 2.)
}
