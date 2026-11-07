//! The cube from the origin to (20, 20, 20) less the cube from (10, 10, 10) to (20, 20, 20), the
//! notch's three concave edges, which meet at the inside corner (10, 10, 10), rounded to radius 3
//! all at once: the corner filled by an eighth of the ball of radius 3 about (13, 13, 13), which
//! each edge's round runs into tangentially.
//!
//! As cad-kernel composes rounds meeting at a corner (a recipe's lowered node): each edge's round
//! rolled against the notched cube (`fillet::roll`), cut short at the corner's section through the
//! ball's centre square to the edge (`build::cut_before`), and the corner's piece — its cell less
//! the ball — built where the description puts the ball (`fillet::corner`), all united with the
//! notched cube.
use crate::sketch::{block, named};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::cut_before;
use cad_kernel::fillet::{corner, roll};
use cad_kernel::geom::{tolerance, V};
use cad_kernel::topo::Brep;

const SIDE: f64 = 20.;
const NOTCH: f64 = 10.;
const ROUND: f64 = 3.;
/// The ball that closes the corner, as the description places it: `ROUND` from each notch face.
const CENTRE: V = [NOTCH + ROUND; 3];

pub fn build() -> Result<Brep, String> {
    let cube = block([0.; 3], [SIDE; 3])?;
    // (every face named apart: a roll reads faces of one name as pieces of one face)
    let notched = named(combined(&cube, &block([NOTCH; 3], [SIDE; 3])?, Op::Cut, 0.)?);
    let tol = tolerance(notched.size());
    let mut solid = notched.clone();
    // each concave edge, along the axis `k` from the corner, picked by its middle
    for k in 0..3 {
        let mut at = [NOTCH; 3];
        at[k] = 0.5 * (NOTCH + SIDE);
        let round = roll(&notched, at, ROUND, tol)
            .map_err(|e| format!("the edge at {at:?}: {e}"))?;
        // the run cut short at the corner's section, the plane through the centre square to it
        let mut along = [0.; 3];
        along[k] = 1.;
        let run = cut_before(&round.piece, CENTRE, along, 2. * ROUND, 0.)
            .map_err(|e| format!("the round at {at:?} cut at the corner: {e}"))?;
        let op = if round.concave { Op::Union } else { Op::Cut };
        solid = combined(&solid, &run, op, 0.)
            .map_err(|e| format!("the round at {at:?} applied: {e}"))?;
    }
    // the corner: from each notch face toward the ball is into the notch, +x, +y and +z
    let cell = corner([NOTCH; 3], CENTRE, ROUND, [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]], tol)?;
    Ok(combined(&solid, &cell, Op::Union, 0.).map_err(|e| format!("the corner applied: {e}"))?)
}
