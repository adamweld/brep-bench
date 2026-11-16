//! An L-block with its sides drafted until they close over its arms: the outline through (0, 0),
//! (20, 0), (20, 5), (5, 5), (5, 15), (0, 15) extruded from z = 0 to z = 8, every side drafted 30°,
//! pull +z, about the neutral plane z = 0.
//!
//! The outline extruded, then cadrs's Draft of every side.
use crate::bench::{extrude, p3, v3};
use crate::shared::draft::{drafted, is_side};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, Pick, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let outline = Loop::polygon(&[
        [0.0, 0.0], [20.0, 0.0], [20.0, 5.0], [5.0, 5.0], [5.0, 15.0], [0.0, 15.0],
    ]);
    let block = extrude(k, &Profile::of(Plane::xy(0.0), vec![outline]), 8.0)?;
    let sides = k.faces_where(block, &is_side)?;
    drafted(k, block, sides, 30.0, v3(0.0, 0.0, 1.0), p3(0.0, 0.0, 0.0))
}
