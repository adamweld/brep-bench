//! A dumbbell plate, two 10 × 10 squares joined by a neck 4 long and 2 wide, extruded 6 along +z,
//! shrunk by 1.5: every point at least 1.5 from its outside. The neck's moved walls cross and are
//! trimmed away by the kernel's offset inward (its self-intersections merged), leaving two bodies.
use crate::bench::polygon;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::sweep::{extrude, DraftCorner};
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let outline = polygon(&[
        [0.0, 0.0, 0.0],
        [10.0, 0.0, 0.0],
        [10.0, 4.0, 0.0],
        [14.0, 4.0, 0.0],
        [14.0, 0.0, 0.0],
        [24.0, 0.0, 0.0],
        [24.0, 10.0, 0.0],
        [14.0, 10.0, 0.0],
        [14.0, 6.0, 0.0],
        [10.0, 6.0, 0.0],
        [10.0, 10.0, 0.0],
        [0.0, 10.0, 0.0],
    ]);
    let plate = extrude(&outline, [0.0, 0.0, 1.0], 6.0, 0.0, DraftCorner::default(), true)?;
    offset_rounded(&plate, -1.5)
}
