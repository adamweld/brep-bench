//! The lens common to two balls of radius 5 centred on (−3, 0, 0) and (3, 0, 0), each bounded by
//! the one NURBS face of `geometry/nurbs-sphere`, moved to its centre: its poles 5 above and below
//! the centre, its seam the meridian on the +x side of the centre in the plane y = 0.
use crate::bench::{common, translated};
use crate::cases::geometry::nurbs_sphere;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let left = translated(nurbs_sphere::build()?, [-3.0, 0.0, 0.0])?;
    let right = translated(nurbs_sphere::build()?, [3.0, 0.0, 0.0])?;
    common(left, right)
}
