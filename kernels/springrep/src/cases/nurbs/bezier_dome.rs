//! A block on the square 0 ≤ x, y ≤ 20, floor z = 0, under a bicubic Bézier dome over an even
//! 4 × 4 grid of control points.
use springrep::topo::Brep;

use crate::shared::nurbs::bezier_dome;

pub fn build() -> Result<Brep, String> {
    bezier_dome()
}
