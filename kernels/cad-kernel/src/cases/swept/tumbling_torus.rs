//! The solid a torus (centre (3, 0, 2), axis along z, radii 1 and 0.5) fills as it turns by t/4
//! about the vertical line through (2, 0, 0) and then by −t about the line through (0, 0.5, 0.5)
//! along x, as t runs from −75° to 75°.
//!
//! Built by the kernel's sweep of a ring under any rigid motion (`swept::closed::ring`): a torus is
//! a tool of revolution whose meridian is a loop clear of its axis, so its contact under the
//! instant's twist is read meridian by meridian in closed form — here two curves at every
//! instant, the outer touching all round and the inner folding four times. Each curve sweeps a
//! tube, fitted; the inner one is dipped into the tool between its touching pieces and entered
//! only where it stands outside the torus at both end poses. The box is split by the tubes and the
//! torus where it starts and stops at once, its cells classified by membership, and the result
//! judged by the sweep's definition before it is returned.
use cad_kernel::build::torus;
use cad_kernel::swept::closed::ring;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};

const ROLL: [f64; 2] = [-75f64.to_radians(), 75f64.to_radians()];
/// How closely the torus is meshed for the sweep's membership reads: a two-hundred-fiftieth of its
/// tube's radius.
const BAR: f64 = 0.002;
/// The tolerance the tubes are fitted and met to: a fiftieth of a thousandth of the tube's radius.
const TOL: f64 = 1e-5;

pub fn build() -> Result<Brep, String> {
    let tool = torus([3., 0., 2.], [0., 0., 1.], 1., 0.5)?;
    // the torus's own turn and the frame's, each a screw that does not advance; the motion is the
    // first seen from the second (`Relative`: at each t the source's pose, then the observer's
    // undone — turned by t/4 about the first line, then by −t about the second)
    let own = Trajectory::Twist(Screw {
        origin: [2., 0., 0.], axis: [0., 0., 1.], ratio: 0.25, advance: 0.,
    });
    let frame = Trajectory::Twist(Screw {
        origin: [0., 0.5, 0.5], axis: [1., 0., 0.], ratio: 1., advance: 0.,
    });
    let path = Trajectory::Relative { source: Box::new(own), observer: Box::new(frame) };
    ring(&tool, &path, ROLL, BAR, TOL).map_err(|why| format!("the torus's sweep not built: {why:?}"))
}
