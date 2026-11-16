//! A cone frustum on z, the disk of radius 6 at z = 0 up to the disk of radius 3 at z = 6, both
//! its rims rounded to radius 1.5, hollowed to a wall 2 thick, closed: thicker than the rounds, so
//! the cavity's edges are sharp.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::cone;
use cad_kernel::fillet::roll;
use cad_kernel::geom::tolerance;
use cad_kernel::offset::{shelled, Join, Side};
use cad_kernel::topo::Brep;

const BASE_RADIUS: f64 = 6.;
const TOP_RADIUS: f64 = 3.;
const HEIGHT: f64 = 6.;
const ROUND: f64 = 1.5;
const THICKNESS: f64 = 2.;

pub fn build() -> Result<Brep, String> {
    let frustum = cone([0., 0., 0.], [0., 0., 1.], BASE_RADIUS, TOP_RADIUS, HEIGHT)?;
    // cad-kernel hollows a solid as the points within the wall of its faces, each face swept
    // inward along its normal, and refuses a round's torus swept past its tube's centre line ("no
    // point stands that far off it"), as a round 1.5 wide is by a wall 2 thick. So the frustum is
    // hollowed first, while its rims are sharp, and rounded after, as a modeller orders the two
    // features when the shell fails: the rounds take only material within 1.5 of the rims, inside
    // the wall, so the cavity is the same either way.
    let mut body = shelled(&frustum, &[], THICKNESS, Side::In, Join::Round)?;
    // each rim picked by a point on it, rounded in turn: the ring outside the round cut off
    for (radius, z) in [(TOP_RADIUS, HEIGHT), (BASE_RADIUS, 0.)] {
        let round = roll(&body, [0., radius, z], ROUND, tolerance(body.size()))?;
        body = combined(&body, &round.piece, Op::Cut, 0.)?;
    }
    Ok(body)
}
