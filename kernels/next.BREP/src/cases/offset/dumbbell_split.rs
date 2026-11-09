//! Two 10 × 10 squares joined by a neck 4 long and 2 wide, extruded 6 along +z, shrunk by 1.5:
//! the neck disappears and both of the bodies left are kept.
use crate::solids::offset::parallel;
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let outline = polygon(&[
        p(0.0, 0.0, 0.0), p(10.0, 0.0, 0.0), p(10.0, 4.0, 0.0), p(14.0, 4.0, 0.0),
        p(14.0, 0.0, 0.0), p(24.0, 0.0, 0.0), p(24.0, 10.0, 0.0), p(14.0, 10.0, 0.0),
        p(14.0, 6.0, 0.0), p(10.0, 6.0, 0.0), p(10.0, 10.0, 0.0), p(0.0, 10.0, 0.0),
    ])?;
    let plate = extrude_profile_brep(&outline, p(0.0, 0.0, 1.0), 6.0)?;
    Ok(parallel(&plate, -1.5)?)
}
