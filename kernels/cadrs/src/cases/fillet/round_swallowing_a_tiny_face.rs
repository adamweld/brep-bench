//! A block from the origin to (10, 10, 5) whose vertical edge at x = 10, y = 10 is broken by a
//! flat at 45°, 0.0001 back along each side: the prism on (0, 0), (10, 0), (10, 9.9999),
//! (9.9999, 10), (0, 10). The flat's two edges are rounded together to radius 1; the round runs
//! from side to side and the flat is gone.
//!
//! The pentagon is sketched on the Top plane and extruded 5; the flat's two vertical edges are
//! picked by a point on each and rounded in one Fillet feature, radius 1.
use crate::bench::{extrude, p3};
use crate::shared::fillet::fillet;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, Pick, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let section =
        Loop::polygon(&[[0.0, 0.0], [10.0, 0.0], [10.0, 9.9999], [9.9999, 10.0], [0.0, 10.0]]);
    let block = extrude(k, &Profile::of(Plane::xy(0.0), vec![section]), 5.0)?;
    let ends = k.edges_at(block, &[p3(10.0, 9.9999, 2.5), p3(9.9999, 10.0, 2.5)])?;
    fillet(k, block, &ends, 1.0)
}
