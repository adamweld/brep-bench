//! The box from the origin to (20, 12, 8), its four upright edges rounded to radius 3, then its
//! whole top rim rounded to radius 1 (four quarter cylinders and four corner tori, one tangent
//! chain); then the chain's eight faces deleted together: the box with rounded upright edges.
//!
//! The solid is made as the case says (the rim's eight edges picked one by one); then the kernel's
//! `delete_faces_and_heal` deletes the faces.
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{delete_faces_and_heal, fillet_edges, make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(0.0, 0.0, 0.0), 20.0, 12.0, 8.0)?;
    let uprights = [p(0.0, 0.0, 4.0), p(20.0, 0.0, 4.0), p(20.0, 12.0, 4.0), p(0.0, 12.0, 4.0)];
    let rounded = fillet_edges(&block, &uprights, None, 3.0, false, None)?;
    let (q, q3) = (0.5f64.sqrt(), 3.0 * 0.5f64.sqrt());
    let rim = [
        p(10.0, 0.0, 8.0), p(20.0, 6.0, 8.0), p(10.0, 12.0, 8.0), p(0.0, 6.0, 8.0),
        p(3.0 - q3, 3.0 - q3, 8.0), p(17.0 + q3, 3.0 - q3, 8.0), p(17.0 + q3, 9.0 + q3, 8.0),
        p(3.0 - q3, 9.0 + q3, 8.0),
    ];
    let chained = fillet_edges(&rounded, &rim, None, 1.0, false, None)?;
    // each face of the chain halfway round its tube
    let (s, a) = (q, 1.0 - q);
    let corner = |cx: f64, cy: f64, dx: f64, dy: f64| p(cx + dx * (2.0 + s), cy + dy * (2.0 + s), 7.0 + s);
    let points = [
        p(10.0, a, 7.0 + s), p(10.0, 12.0 - a, 7.0 + s), p(a, 6.0, 7.0 + s), p(20.0 - a, 6.0, 7.0 + s),
        corner(3.0, 3.0, -q, -q), corner(17.0, 3.0, q, -q), corner(17.0, 9.0, q, q), corner(3.0, 9.0, -q, q),
    ];
    let mut chain = Vec::new();
    for point in points {
        chain.push(face_at(&chained, point)?);
    }
    Ok(delete_faces_and_heal(&chained, &chain)?)
}
