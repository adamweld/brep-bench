//! A prism 6 high on the pentagon (0, 0), (8, 0), (8, 5), (4, 5 + 4 tan 0.5°), (0, 5): a block
//! whose top is two faces meeting at 179° along a ridge at x = 4. The ridge is rounded to radius
//! 180, an arc turning through 1°.
//!
//! The pentagon is sketched on the Top plane and extruded 6; the ridge is picked by the point at
//! its middle and filleted, radius 180.
use crate::bench::{extrude, p3};
use crate::shared::fillet::fillet;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, Pick, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let peak = 5.0 + 4.0 * 0.5f64.to_radians().tan();
    let section = Loop::polygon(&[[0.0, 0.0], [8.0, 0.0], [8.0, 5.0], [4.0, peak], [0.0, 5.0]]);
    let prism = extrude(k, &Profile::of(Plane::xy(0.0), vec![section]), 6.0)?;
    let ridge = k.edge_at(prism, p3(4.0, peak, 3.0))?;
    fillet(k, prism, &[ridge], 180.0)
}
