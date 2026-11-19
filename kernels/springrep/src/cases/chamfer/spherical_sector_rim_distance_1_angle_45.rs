//! A spherical sector (cos α = 3/5) of the ball of radius 5, its rim chamfered 1 down the cone at 45° to the
//! cone's side, turned toward the axis.
//!
//! springrep chamfers by one distance or two, not by a distance and an angle, so the chamfer is
//! made as what it is: the chamfer's line, from the point 1 down the cone and at 45° to it,
//! turned about the axis into a cone, and the ring between the rim and that cone cut away with a
//! boolean.
use springrep::topo::Brep;

use crate::shared::chamfer::sector_rim_by_angle;

pub fn build() -> Result<Brep, String> {
    sector_rim_by_angle(3. / 5., 1., 45f64.to_radians())
}
