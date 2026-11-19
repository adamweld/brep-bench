//! The solid the regular tetrahedron with corners A = (5, 1, 1), B = (5, −1, −1), C = (3, 1, −1)
//! and D = (3, −1, 1) fills as the screw about the z axis that rises 5 a turn carries it from a
//! turn of 0° to one of 540°.
//!
//! solidcraft-kernel has no sweep of a solid, so the sweep is built by elementary geometry from
//! the kernel's helical sweep of planar regions (`sweep_helix`). A point the tetrahedron passes
//! over that is outside it where it starts is reached first through a face moving outward there.
//! A point p moves with (−p.y, p.x, 5/2π) per radian of the screw, so a face's outward normal n
//! meets that motion in a dot product linear across the face, and the face leads wherever it does
//! at all three corners. ACD (n along (−1, 1, 1)) and ABC (n along (1, 1, −1)) lead at all of
//! theirs; BCD and ABD trail at all of theirs. So the sweep is the tetrahedron where it starts,
//! with the screw sweeps of ACD and ABC, each crossing the motion, never moving along itself; the
//! tetrahedron where it ends is their far end.
use super::square_bar_screwed_through_rod::screw;
use crate::solids::*;
use solidcraft_kernel::loft_to_point;

/// The plane of the triangle `p`, from its first corner along its first side, and the triangle
/// drawn on it.
fn triangle(p: [Vec3; 3]) -> R<(Plane, Loop2)> {
    let bad = || KernelError::Invalid("the triangle's plane".into());
    let x = (p[1] - p[0]).normalized().ok_or_else(bad)?;
    let n = (p[1] - p[0]).cross(p[2] - p[0]).normalized().ok_or_else(bad)?;
    let plane = Plane { origin: p[0], x, y: n.cross(x) };
    let at = |q: Vec3| v2((q - p[0]).dot(plane.x), (q - p[0]).dot(plane.y));
    Ok((plane, Loop2::polygon(&[at(p[0]), at(p[1]), at(p[2])])))
}

pub fn build() -> R<Body> {
    let (a, b, c, d) = (v3(5.0, 1.0, 1.0), v3(5.0, -1.0, -1.0), v3(3.0, 1.0, -1.0), v3(3.0, -1.0, 1.0));
    let (base, lp) = triangle([b, c, d])?;
    let mut swept = loft_to_point(&base, &lp, a)?;
    for face in [[a, c, d], [a, b, c]] {
        let (plane, lp) = triangle(face)?;
        let region = Region2 { outer: lp, holes: vec![] };
        swept = union(&swept, &screw(&plane, &region, 5.0, 0.0, 540.0)?)?;
    }
    Ok(swept)
}
