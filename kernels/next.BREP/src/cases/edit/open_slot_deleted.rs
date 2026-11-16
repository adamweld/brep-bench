//! The box from the origin to (20, 10, 10) with a slot in its side y = 0: 4 wide from x = 8 to x =
//! 12, in to y = 5, closed by a half cylinder of radius 2 about the vertical line through (10, 5).
//! Then the slot's two flats and its half cylinder deleted, the side, the top and the bottom
//! closing over it: the box.
//!
//! The solid is made as the case says; then the kernel's `delete_faces_and_heal` deletes the faces.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{delete_faces_and_heal, make_box_brep, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(0.0, 0.0, 0.0), 20.0, 10.0, 10.0)?;
    // the slot's straight part, run on past the side, the top and the bottom, and its round end
    let straight = make_box_brep(p(8.0, -1.0, -1.0), 4.0, 6.0, 12.0)?;
    let round = make_cylinder_brep(p(10.0, 5.0, -1.0), p(0.0, 0.0, 1.0), 2.0, 12.0)?;
    let slotted = cut(&cut(&block, &straight)?, &round)?;
    // the two flats, and the round end 60° round from x = 12, clear of the seam at its crown
    let picks = [p(8.0, 2.0, 5.0), p(12.0, 2.0, 5.0), p(11.0, 5.0 + 3f64.sqrt(), 5.0)];
    let faces = picks.iter().map(|&q| face_at(&slotted, q)).collect::<Result<Vec<_>, _>>()?;
    Ok(delete_faces_and_heal(&slotted, &faces)?)
}
