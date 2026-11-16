//! The half of the ball of radius 5 about the origin above z = 0, its rim rounded to radius 1,
//! hollowed to a wall 2 thick, closed: thicker than the round, so the cavity, the ball of radius 3
//! above z = 2, has a sharp edge.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{kept, sphere};
use cad_kernel::fillet::roll;
use cad_kernel::geom::tolerance;
use cad_kernel::offset::{shelled, Join, Side};
use cad_kernel::topo::Brep;

const RADIUS: f64 = 5.;
const ROUND: f64 = 1.;
const THICKNESS: f64 = 2.;

pub fn build() -> Result<Brep, String> {
    // the ball kept above the plane z = 0
    let half = kept(&sphere([0., 0., 0.], [0., 0., 1.], RADIUS)?, [0., 0., 0.], [0., 0., 1.], 0.)?;
    // cad-kernel hollows a solid as the points within the wall of its faces, each face swept
    // inward along its normal. It rolls the round between the flat and the ball as a canal
    // fitted by a B-spline, whose sweep it refuses ("a bspline has no offset in closed form"),
    // and it would refuse the exact torus too, swept 2 past a tube of radius 1 ("no point stands
    // that far off it"). So the half ball is hollowed first, while its rim is sharp, and rounded
    // after, as a modeller orders the two features when the shell fails: the round takes only
    // material within 1 of the rim, inside the wall, so the cavity is the same either way.
    let body = shelled(&half, &[], THICKNESS, Side::In, Join::Round)?;
    // the rim picked by a point on it; the ring outside the round cut off
    let round = roll(&body, [0., RADIUS, 0.], ROUND, tolerance(body.size()))?;
    Ok(combined(&body, &round.piece, Op::Cut, 0.)?)
}
