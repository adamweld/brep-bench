//! A rod, the cylinder of radius 5 about z from z = 0 to 20, threaded partway up by the bar
//! 4 ≤ x ≤ 12, −8 ≤ y ≤ 8, −1 ≤ z ≤ 1, screwed about z — right-handed, rising 6 a turn — from a
//! turn of −90° to +900°, where its tip is at (−4, 0, 15), and then slid without turning 3 along
//! −x, straight out of its groove: a path of two pieces.
//!
//! cad-kernel's sweep of a solid (`swept::closed::cut_from`) takes a path of pieces only where
//! every piece is a slide or a turn (K1); a screw among them sends the path down its road for any
//! other motion (K2), which takes only a convex solid of revolution and refuses the bar
//! (`Unbuilt::Curved` at its corner). So the path's sweep is built as its pieces' sweeps: each
//! piece, a constant twist (the screw) or one taken up where the screw left the bar (`Placed`, the
//! slide), swept and cut from the rod in turn.
use crate::sketch::block;
use cad_kernel::build::{disc, prism};
use cad_kernel::geom::tolerance;
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Piece, Pose, Screw, Trajectory};
use std::f64::consts::TAU;

const ROD_RADIUS: f64 = 5.;
const ROD_LENGTH: f64 = 20.;
const LEAD: f64 = 6.;
/// The screw's stretch of the roll, in radians of its turn.
const SCREWED: [f64; 2] = [-90f64.to_radians(), 900f64.to_radians()];
/// How far the run-out slides, along −x.
const RUN_OUT: f64 = 3.;

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    let rod = prism(&disc([0., 0., 0.], up, ROD_RADIUS), 0., ROD_LENGTH)?;
    let bar = block([4., -8., -1.], [12., 8., 1.])?;
    let screw = Screw { origin: [0., 0., 0.], axis: up, ratio: 1., advance: LEAD };
    // the run-out over the next τ of the roll: a slide along −x of its whole length, taken up
    // where the screw left the bar (`Placed`: the screw's last pose, less the slide's own step at
    // the join, so the slide starts from nothing there)
    let [_, join] = SCREWED;
    let slide = Screw { origin: [0., 0., 0.], axis: [-1., 0., 0.], ratio: 0., advance: RUN_OUT };
    let still = [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]];
    let back = Pose { r: still, p: [RUN_OUT * join / TAU, 0., 0.] };
    let at = screw.pose(join).then(back);
    let pieces = vec![
        Piece { over: SCREWED, path: Trajectory::Twist(screw) },
        Piece { over: [join, join + TAU], path: Trajectory::Placed { twist: slide, at } },
    ];
    // the two pieces checked by the kernel to make one path, meeting at the same pose at the join
    let size = rod.size().max(bar.size());
    let path = Trajectory::pieces(pieces, tolerance(size), size)?;
    // what the bar passes through over the path is what it passes through over each piece: each
    // piece's sweep cut from the rod in turn, by the kernel's own sweep of a solid
    let Trajectory::Pieces(pieces) = &path else { unreachable!() };
    let mut cut = rod;
    for piece in pieces.pieces() {
        cut = cut_from(&cut, &bar, &piece.path, piece.over, bar.size(), tolerance(size))
            .map_err(|why| format!("the bar's sweep over {:?} not built: {why:?}", piece.over))?;
    }
    Ok(cut)
}
