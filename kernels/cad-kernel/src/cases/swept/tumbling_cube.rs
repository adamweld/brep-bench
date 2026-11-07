//! The solid a cube — the box (2.4, −0.6, 1.4)–(3.6, 0.6, 2.6) — fills as it turns by t/4 about the
//! vertical line through (2, 0, 0), seen from a frame turning by t about the line through
//! (0, 0.5, 0.5) along x, as t runs from −75° to 75°.
//!
//! Refused by the kernel. Two turns about skew axes are no constant twist, so the sweep takes the
//! kernel's road for any other rigid motion (K2), which reads the tool as a convex solid of
//! revolution and refuses the cube (`Unbuilt::Curved` at its corner).
use crate::sketch::block;
use cad_kernel::geom::tolerance;
use cad_kernel::swept::closed::closed;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};

const ROLL: [f64; 2] = [-75f64.to_radians(), 75f64.to_radians()];

pub fn build() -> Result<Brep, String> {
    let cube = block([2.4, -0.6, 1.4], [3.6, 0.6, 2.6])?;
    // the cube's own turn and the frame's, each a screw that does not advance; the motion is the
    // first seen from the second (`Relative`: at each t the source's pose, then the observer's
    // undone — turned by t/4 about the first line, then by −t about the second)
    let own = Trajectory::Twist(Screw {
        origin: [2., 0., 0.], axis: [0., 0., 1.], ratio: 0.25, advance: 0.,
    });
    let frame = Trajectory::Twist(Screw {
        origin: [0., 0.5, 0.5], axis: [1., 0., 0.], ratio: 1., advance: 0.,
    });
    let path = Trajectory::Relative { source: Box::new(own), observer: Box::new(frame) };
    // no constant twist and no path of slides and turns: the kernel's road for any other rigid
    // motion (K2)
    closed(&cube, &path, ROLL, cube.size(), tolerance(cube.size()))
        .map_err(|why| format!("the cube's sweep not built: {why:?}"))
}
