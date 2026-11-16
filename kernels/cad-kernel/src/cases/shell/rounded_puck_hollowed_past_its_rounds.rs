//! A puck, the cylinder of radius 6 on z from z = 0 to 10, both its rims rounded to radius 2,
//! hollowed to a wall 3 thick, closed: thicker than the rounds, so the cavity's edges are sharp.
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
    // cad-kernel hollows a solid as the points within the wall of its faces, each face swept
    // inward along its normal, and refuses a round's torus swept past its tube's centre line ("no
    // point stands that far off it"), as a round 2 wide is by a wall 3 thick. So the puck is
    // hollowed first, while its rims are sharp, and rounded after, as a modeller orders the two
    // features when the shell fails: the rounds take only material within 2 of the rims, inside
    // the wall, so the cavity is the same either way.
    let mut body = shelled(&puck, &[], THICKNESS, Side::In, Join::Round)?;
    // each rim picked by a point on it, rounded in turn: the ring outside the round cut off
    for z in [HEIGHT, 0.] {
        let round = roll(&body, [0., RADIUS, z], ROUND, tolerance(body.size()))?;
        body = combined(&body, &round.piece, Op::Cut, 0.)?;
    }
    Ok(body)
}
