//! The ball of radius 5 standing in a conical seat, the concave circle where they meet chamfered
//! 2 up the seat's cone at 30° to it, filling the groove.
//!
//! springrep chamfers by one distance or two, not by a distance and an angle, so the chamfer is
//! made as what it is: the chamfer's line, from the point 2 along the face it is measured on and
//! at 30° to that face, turned about the axis into a cone, and the ring between the edge and that
//! cone added with a boolean.
use springrep::topo::Brep;

use crate::shared::chamfer::conical_seat_by_angle;

pub fn build() -> Result<Brep, String> {
    conical_seat_by_angle(2., 30f64.to_radians())
}
