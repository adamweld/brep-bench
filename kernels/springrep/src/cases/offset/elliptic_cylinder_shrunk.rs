//! The ellipse x²/25 + y²/9 ≤ 1 extruded from z = 0 to z = 10, shrunk by 1: its ends move in to
//! z = 1 and z = 9 and its side onto the ellipse's inner offset curve, its edges sharp.
use springrep::topo::Brep;

use crate::shared::offset::{elliptic_cylinder, shrunk};

pub fn build() -> Result<Brep, String> {
    shrunk(&elliptic_cylinder()?, 1.)
}
