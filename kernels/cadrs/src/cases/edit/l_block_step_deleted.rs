//! The L outline through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15) extruded 8 along +z;
//! then its riser x = 5 and its tread y = 5 deleted together, the far end y = 15 and the far side x
//! = 20 run on to meet: the box from the origin to (20, 15, 8).
//!
//! The outline is extruded; then Delete face takes the riser and the tread.
use crate::bench::{extrude, p3};
use crate::shared::edit::deleted;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, Pick, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let corners = [[0.0, 0.0], [20.0, 0.0], [20.0, 5.0], [5.0, 5.0], [5.0, 15.0], [0.0, 15.0]];
    let b = extrude(k, &Profile::of(Plane::xy(0.0), vec![Loop::polygon(&corners)]), 8.0)?;
    let step = k.faces_at(b, &[p3(5.0, 10.0, 4.0), p3(12.0, 5.0, 4.0)])?;
    deleted(k, b, &step)
}
