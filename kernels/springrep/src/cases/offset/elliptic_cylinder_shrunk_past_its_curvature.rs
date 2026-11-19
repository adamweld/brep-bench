//! The ellipse x²/25 + y²/9 ≤ 1 extruded from z = 0 to z = 10, shrunk by 2.5, more than the
//! ellipse's least radius of curvature: the side's inner offset crosses itself near the ends of the
//! long axis, and the shrunk side meets itself there in a sharp edge.
//! springrep 1.0.1 refuses it: the side's offset, trimmed where it crosses itself, comes out off
//! its edges (1e-3 mm), which its checker refuses.
use springrep::topo::Brep;

use crate::shared::offset::{elliptic_cylinder, shrunk};

pub fn build() -> Result<Brep, String> {
    shrunk(&elliptic_cylinder()?, 2.5)
}
