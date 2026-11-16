//! A cup's skin thickened inward past its bend's radius. A sheet of revolution about the z axis:
//! a flat floor, the disk of radius 4 in the plane z = 0; a bend, the quarter of a torus whose
//! tube has radius 2 about the circle of radius 4 at z = 2; and a wall, the cylinder of radius 6 from
//! z = 2 up to z = 10, thickened by 3 to the inside, up from the floor and in from the wall.
//!
//! The kernel's thicken of a sheet of several faces (`thicken_sheet_faces`): the floor's line, the
//! bend's arc and the wall's line, each turned a whole turn about the z axis into a face
//! (`make_revolution`, `face_from_surface`), thickened together. Run from the axis outward and up,
//! the faces' normals are down and outward, so the inside is the negative thickness.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{face_from_surface, make_arc, make_line, make_revolution, thicken_sheet_faces, BrepSolid, OffsetCorners};
use std::f64::consts::{FRAC_PI_2, TAU};

const FLOOR: f64 = 4.0;
const BEND: f64 = 2.0;
const HEIGHT: f64 = 10.0;
const THICKNESS: f64 = 3.0;

pub fn build() -> Result<BrepSolid, Refusal> {
    let profile = [
        make_line(p(0.0, 0.0, 0.0), p(FLOOR, 0.0, 0.0))?,
        make_arc(p(FLOOR, 0.0, BEND), p(0.0, 0.0, -1.0), p(1.0, 0.0, 0.0), BEND, 0.0, FRAC_PI_2)?,
        make_line(p(FLOOR + BEND, 0.0, BEND), p(FLOOR + BEND, 0.0, HEIGHT))?,
    ];
    let mut sheet = Vec::new();
    for curve in &profile {
        sheet.push(face_from_surface(&make_revolution(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), curve, TAU)?)?);
    }
    Ok(thicken_sheet_faces(&sheet, -THICKNESS, OffsetCorners::Sharp)?)
}
