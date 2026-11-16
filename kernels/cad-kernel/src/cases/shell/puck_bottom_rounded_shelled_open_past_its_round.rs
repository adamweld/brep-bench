//! A puck, the cylinder of radius 6 on z from z = 0 to 10, its bottom rim rounded to radius 2,
//! shelled inward to a wall 3 thick, open at its top: thicker than the round, so the cavity's
//! floor and wall meet in a sharp edge.
use crate::sketch::faces_through;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::fillet::roll;
use cad_kernel::geom::tolerance;
use cad_kernel::offset::{shelled, Join, Side};
use cad_kernel::topo::Brep;

const RADIUS: f64 = 6.;
const HEIGHT: f64 = 10.;
const ROUND: f64 = 2.;
const THICKNESS: f64 = 3.;

pub fn build() -> Result<Brep, String> {
    let puck = prism(&disc([0., 0., 0.], [0., 0., 1.], RADIUS), 0., HEIGHT)?;
    // cad-kernel shells a solid as the points within the wall of its faces, each face swept
    // inward along its normal, and refuses a round's torus swept past its tube's centre line ("no
    // point stands that far off it"), as a round 2 wide is by a wall 3 thick. So the puck is
    // shelled first, while its rim is sharp, and rounded after, as a modeller orders the two
    // features when the shell fails: the round takes only material within 2 of the rim, inside
    // the wall, so the cavity is the same either way.
    let open = faces_through(&puck, &[[0., 0., HEIGHT]])?;
    let body = shelled(&puck, &open, THICKNESS, Side::In, Join::Round)?;
    // the bottom rim picked by a point on it; the ring outside the round cut off
    let round = roll(&body, [0., RADIUS, 0.], ROUND, tolerance(body.size()))?;
    Ok(combined(&body, &round.piece, Op::Cut, 0.)?)
}
