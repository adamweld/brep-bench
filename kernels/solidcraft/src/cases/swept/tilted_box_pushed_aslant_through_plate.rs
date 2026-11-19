//! A plate, the box from (−15, −15, 0) to (15, 15, 4), with a hole punched through it by a tilted
//! box: everything (−2, −1, −8) + a e₁ + b e₂ + c e₃ with |a| ≤ 2, |b| ≤ 1.5 and |c| ≤ 1, its
//! edges along e₁ = (2, 1, 2)/3, e₂ = (−2, 2, 1)/3 and e₃ = (1, 2, −2)/3, slid without turning
//! along (4, 2, 18).
//!
//! solidcraft-kernel has no sweep of a solid, so the slide is built, as in `box-slid-aslant`, from
//! the box and its faces by elementary geometry: a point the box passes over that is outside it
//! where it starts is reached first by a face it leads with, one whose outward normal points along
//! the slide. The slide makes 46/3 with e₁, 14/3 with e₂ and −28/3 with e₃, so those are the faces
//! a = 2, b = 1.5 and c = −1; the solid is the box where it starts and those three faces each slid
//! the whole way. The box is drawn on planes along its own edges, so it is placed by them.
use crate::solids::*;
use solidcraft_kernel::{PathSeg, sweep};

pub fn build() -> R<Body> {
    let plate = block((-15.0, -15.0, 0.0), (15.0, 15.0, 4.0))?;
    let slide = v3(4.0, 2.0, 18.0);
    let centre = v3(-2.0, -1.0, -8.0);
    let e1 = v3(2.0, 1.0, 2.0) * (1.0 / 3.0);
    let e2 = v3(-2.0, 2.0, 1.0) * (1.0 / 3.0);
    // −e₃ = e₁ × e₂, so (e₁, e₂, −e₃) is a right-handed frame; the box's face c = −1 faces −e₃
    let f3 = e1.cross(e2);
    // the box: its section in (a, b) extruded from c = 1 to c = −1 (along −e₃)
    let mut swept = extrude(&Plane { origin: centre, x: e1, y: e2 }, rect(-2.0, -1.5, 2.0, 1.5), -1.0, 1.0)?;
    // the leading faces, each drawn about its centre along the box's other two edges
    let faces = [
        (Plane { origin: centre + e1 * 2.0, x: e2, y: f3 }, rect(-1.5, -1.0, 1.5, 1.0)),
        (Plane { origin: centre + e2 * 1.5, x: f3, y: e1 }, rect(-1.0, -2.0, 1.0, 2.0)),
        (Plane { origin: centre + f3, x: e1, y: e2 }, rect(-2.0, -1.5, 2.0, 1.5)),
    ];
    for (plane, face) in &faces {
        let path = [PathSeg::Line { a: plane.origin, b: plane.origin + slide }];
        swept = union(&swept, &sweep(plane, face, &path)?)?;
    }
    cut(&plate, &swept)
}
