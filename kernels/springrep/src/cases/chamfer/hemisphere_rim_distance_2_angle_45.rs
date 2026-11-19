//! The upper half of a ball of radius 5, its rim chamfered 2 in across the flat face at 45° to it.
//!
//! springrep chamfers by one distance or two, not by a distance and an angle, so the chamfer is
//! made as what it is: the chamfer's line, from the point 2 along the face it is measured on and
//! at 45° to that face, turned about the axis into a cone, and the ring between the edge and that
//! cone cut away with a boolean.
use springrep::topo::Brep;

use crate::shared::chamfer::hemisphere_rim_by_angle;

pub fn build() -> Result<Brep, String> {
    hemisphere_rim_by_angle(2., 45f64.to_radians())
}
