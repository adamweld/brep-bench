//! The cylinder of radius 3 about the z axis from z = −10 up to the plane z = x·tan 30°, grown by
//! 1: its elliptic top edge becomes a canal surface, its bottom rim a piece of a torus.
use springrep::topo::Brep;

use crate::shared::offset::{grown, slant_cut_cylinder};

pub fn build() -> Result<Brep, String> {
    grown(&slant_cut_cylinder(30.)?, 1.)
}
