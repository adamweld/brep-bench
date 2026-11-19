//! The ball of radius 5 standing in a plate, the concave circle where they meet chamfered 2 out
//! across the plate at 45° to it, filling the groove.
//!
//! springrep chamfers by one distance or two, not by a distance and an angle, so the chamfer is
//! made as what it is: the chamfer's line, from the point 2 along the face it is measured on and
//! at 45° to that face, turned about the axis into a cone, and the ring between the edge and that
//! cone added with a boolean.
use springrep::topo::Brep;

use crate::shared::chamfer::plate_rim_by_angle;

pub fn build() -> Result<Brep, String> {
    plate_rim_by_angle(2., 45f64.to_radians())
}
