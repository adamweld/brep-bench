//! The L-shaped plate on the outline (0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15), 8 thick,
//! grown by 1: its convex edges rounded, the inside corner's edge sharp.
use springrep::topo::Brep;

use crate::shared::offset::{grown, l_block};

pub fn build() -> Result<Brep, String> {
    grown(&l_block()?, 1.)
}
