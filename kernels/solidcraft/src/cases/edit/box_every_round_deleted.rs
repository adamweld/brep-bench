//! A box from the origin to (20, 16, 12), every edge rounded to radius 2: twelve quarter
//! cylinders and eight eighths of a ball at the corners; then all twenty rounds deleted, the six
//! flat faces run on until they meet in the box's edges and corners.
use crate::shared::edit::faces_nearest;
use crate::solids::*;

pub fn build() -> R<Body> {
    let (x, y, z) = (20.0, 16.0, 12.0);
    let sharp = block((0.0, 0.0, 0.0), (x, y, z))?;
    // the middle of each of the twelve edges, and the eight corners
    let (mut edges, mut corners) = (Vec::new(), Vec::new());
    for a in [0.0, 1.0] {
        for b in [0.0, 1.0] {
            edges.extend([v3(x / 2.0, a * y, b * z), v3(a * x, y / 2.0, b * z), v3(a * x, b * y, z / 2.0)]);
            corners.extend([v3(a * x, b * y, 0.0), v3(a * x, b * y, z)]);
        }
    }
    let rounded = solidcraft_kernel::fillet(&sharp, &edges, 2.0)?;
    // the rounds: the faces nearest where the edges and the corners were
    let rounds = faces_nearest(&rounded, &[edges, corners].concat())?;
    solidcraft_kernel::delete_faces(&rounded, &rounds)
}
