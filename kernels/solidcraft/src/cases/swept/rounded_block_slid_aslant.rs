//! The solid a rounded block fills as it slides along (8, 5, 3) without turning: everything within
//! 1 of the box from (1, 1, 1) to (9, 5, 3).
//!
//! No layer has a sweep of a solid, so the sweep is built by elementary geometry. The rounded
//! block is the box grown by the ball of radius 1, and growing commutes with sliding: what it
//! sweeps is everything within 1 of what the box sweeps. The box's sweep is, as in
//! `box-slid-aslant`, the box and its faces that lead the slide (x = 9, y = 5 and z = 3) slid the
//! whole way, which ops sweeps as profiles carried along a line (`sweep_placed`, translated); the
//! grown solid is the kernel's offset with its convex edges rounded (`offset_rounded`), a raw
//! solidcore operation that ops does not pass on.
use crate::bench::{fuse_all, line, polygon};
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::sweep::{sweep_placed, SweepFrame};
use solidcore_ops::{make_box, Body};

pub fn build() -> Result<Body, Refusal> {
    let slide = [8.0, 5.0, 3.0];
    let mut pieces = vec![make_box([1.0, 1.0, 1.0], [8.0, 4.0, 2.0])?];
    let faces = [
        [[9.0, 1.0, 1.0], [9.0, 5.0, 1.0], [9.0, 5.0, 3.0], [9.0, 1.0, 3.0]],
        [[1.0, 5.0, 1.0], [9.0, 5.0, 1.0], [9.0, 5.0, 3.0], [1.0, 5.0, 3.0]],
        [[1.0, 1.0, 3.0], [9.0, 1.0, 3.0], [9.0, 5.0, 3.0], [1.0, 5.0, 3.0]],
    ];
    for face in &faces {
        // the path from the face's first corner along the slide
        let [x, y, z] = face[0];
        let path = line(face[0], [x + slide[0], y + slide[1], z + slide[2]]);
        pieces.push(sweep_placed(&polygon(face), &[path], SweepFrame::Translate, true)?.0);
    }
    offset_rounded(&fuse_all(pieces)?, 1.0)
}
