//! A frustum capped by a ball, its rim chamfered 1 down the cone at 60° to the cone's side.
//!
//! springrep chamfers by one distance or two, not by a distance and an angle, so the chamfer is
//! made as what it is: the chamfer's line, from the point 1 along the face it is measured on and
//! at 60° to that face, turned about the axis into a cone, and the ring between the edge and that
//! cone cut away with a boolean.
use springrep::topo::Brep;

use crate::shared::chamfer::capped_frustum_rim_by_angle;

pub fn build() -> Result<Brep, String> {
    capped_frustum_rim_by_angle(1., 60f64.to_radians())
}
