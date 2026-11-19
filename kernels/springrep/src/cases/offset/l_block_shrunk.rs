//! The L-shaped plate on the outline (0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15), 8 thick,
//! shrunk by 1: its convex edges sharp, the inside corner's edge a concave quarter cylinder.
use springrep::topo::Brep;

use crate::shared::offset::{l_block, shrunk};

pub fn build() -> Result<Brep, String> {
    shrunk(&l_block()?, 1.)
}
