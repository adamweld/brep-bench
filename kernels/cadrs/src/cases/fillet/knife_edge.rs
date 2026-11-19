//! A wedge 6 high on the triangle (0, 0), (10, 0), (10, 10 tan 5°): its edge along the z axis is a
//! knife edge where two faces meet at 5°, rounded to radius 0.1 (the round turns through 175°).
//!
//! The triangle is sketched on the Top plane and extruded 6; the knife edge is picked by the point
//! at its middle and filleted, radius 0.1.
use crate::bench::{extrude, p3};
use crate::shared::fillet::fillet;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, Pick, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let section = Loop::polygon(&[[0.0, 0.0], [10.0, 0.0], [10.0, 10.0 * 5f64.to_radians().tan()]]);
    let wedge = extrude(k, &Profile::of(Plane::xy(0.0), vec![section]), 6.0)?;
    let blade = k.edge_at(wedge, p3(0.0, 0.0, 3.0))?;
    fillet(k, wedge, &[blade], 0.1)
}
