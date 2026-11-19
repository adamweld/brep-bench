//! A cube, the box from the origin to (10, 10, 10), the three edges meeting at its corner
//! (10, 10, 10) rounded to radius 2, an eighth of a ball where they meet; then all four faces
//! deleted, the faces x = 10, y = 10 and z = 10 run on until they meet in the edges and the
//! corner again.
use crate::shared::edit::faces_nearest;
use crate::solids::*;

pub fn build() -> R<Body> {
    let edges = [v3(5.0, 10.0, 10.0), v3(10.0, 5.0, 10.0), v3(10.0, 10.0, 5.0)];
    let cube = block((0.0, 0.0, 0.0), (10.0, 10.0, 10.0))?;
    let rounded = solidcraft_kernel::fillet(&cube, &edges, 2.0)?;
    // the three rounds and the ball: the faces nearest where the edges and the corner were
    let rounds = faces_nearest(&rounded, &[edges[0], edges[1], edges[2], v3(10.0, 10.0, 10.0)])?;
    solidcraft_kernel::delete_faces(&rounded, &rounds)
}
