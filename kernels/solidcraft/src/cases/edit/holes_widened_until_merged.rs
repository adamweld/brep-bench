//! A plate, the box from the origin to (40, 30, 8), drilled right through along z by two holes
//! of radius 3 on the lines through (15, 15) and (25, 15); then both walls offset by 3 into the
//! plate, to radius 6, so that the holes overlap and merge: ops' face offset. (SolidCraft's own
//! `offset_faces` runs the same SMLib offset and refuses it too; neither layer, nor the kernel's
//! `OffsetFacesExecutive` beneath them, has an option to merge offset faces that cross.)
use crate::bench::cut;
use crate::shared::edit::{drilled_box, move_faces};
use crate::Refusal;
use solidcore_ops::{make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let plate = cut(drilled_box([40.0, 30.0, 8.0], 15.0, 15.0, 3.0)?, make_cylinder([25.0, 15.0, -1.0], 3.0, 10.0)?)?;
    // a wall's outward normal points into its hole, toward the axis: into the plate is inward
    move_faces(&plate, &[[15.0, 18.0, 4.0], [25.0, 18.0, 4.0]], -3.0)?;
    Ok(plate)
}
