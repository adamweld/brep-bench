//! A cone frustum on z, the disk of radius 6 at z = 0 up to the disk of radius 3 at z = 6, its
//! bottom rim rounded to radius 1.5, shelled inward to a wall 2 thick, open at its top: thicker
//! than the round, so the cavity's floor and side meet in a sharp edge.
use crate::sketch::faces_through;
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
    // cad-kernel shells a solid as the points within the wall of its faces, each face swept
    // inward along its normal, and refuses a round's torus swept past its tube's centre line ("no
    // point stands that far off it"), as a round 1.5 wide is by a wall 2 thick. So the frustum is
    // shelled first, while its rim is sharp, and rounded after, as a modeller orders the two
    // features when the shell fails: the round takes only material within 1.5 of the rim, inside
    // the wall, so the cavity is the same either way.
    let open = faces_through(&frustum, &[[0., 0., HEIGHT]])?;
    let body = shelled(&frustum, &open, THICKNESS, Side::In, Join::Round)?;
    // the bottom rim picked by a point on it; the ring outside the round cut off
    let round = roll(&body, [0., BASE_RADIUS, 0.], ROUND, tolerance(body.size()))?;
    Ok(combined(&body, &round.piece, Op::Cut, 0.)?)
}
