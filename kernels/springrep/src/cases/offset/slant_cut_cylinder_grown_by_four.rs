//! The cylinder of radius 3 about the z axis from z = −10 up to the plane z = x·tan 45°, grown by
//! 4: its elliptic top edge becomes a canal surface, its bottom rim a piece of a torus.
//! springrep 1.0.1 refuses it: the rolling ball's centre along the moved ellipse does not settle.
use springrep::topo::Brep;

use crate::shared::offset::{grown, slant_cut_cylinder};

pub fn build() -> Result<Brep, String> {
    grown(&slant_cut_cylinder(45.)?, 4.)
}
