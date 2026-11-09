//! The box from the origin to (40, 30, 8), drilled through along z by holes of radius 3 about the
//! vertical lines through (15, 15) and (25, 15). Each wall is offset 3 into the plate, away from
//! its axis, to radius 6, so that the two holes overlap and merge.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{accept_sound, make_box_brep, make_cylinder_brep, offset_ruled_face, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let plate = make_box_brep(p(0.0, 0.0, 0.0), 40.0, 30.0, 8.0)?;
    let left = make_cylinder_brep(p(15.0, 15.0, -1.0), p(0.0, 0.0, 1.0), 3.0, 10.0)?;
    let right = make_cylinder_brep(p(25.0, 15.0, -1.0), p(0.0, 0.0, 1.0), 3.0, 10.0)?;
    let drilled = cut(&cut(&plate, &left)?, &right)?;
    // A negative distance pushes a hole's wall into the plate, away from its axis.
    let wall = face_at(&drilled, p(12.0, 15.0, 4.0))?;
    let one_widened = offset_ruled_face(&drilled, wall, -3.0)?;
    let wall = face_at(&one_widened, p(28.0, 15.0, 4.0))?;
    // The raw push does not look for the other hole, so ask, as the kernel's Push Face does, whether the result is sound.
    Ok(accept_sound(offset_ruled_face(&one_widened, wall, -3.0)?, "pushFace")?)
}
