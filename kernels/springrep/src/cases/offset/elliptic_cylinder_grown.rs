//! The ellipse x²/25 + y²/9 ≤ 1 extruded from z = 0 to z = 10, grown by 1: its side moves out onto
//! the ellipse's offset curve and its rims become canal surfaces about the ellipses.
//! springrep 1.0.1 refuses it: its rolling-ball fillet does not take the moved rim, two edges of
//! the ellipse's halves.
use springrep::topo::Brep;

use crate::shared::offset::{elliptic_cylinder, grown};

pub fn build() -> Result<Brep, String> {
    grown(&elliptic_cylinder()?, 1.)
}
