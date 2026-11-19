//! A prism from z = 0 to 6 on the pentagon (0, 0), (8, 0), (8, 5), (4, 5 + 4 tan 0.5°), (0, 5): a
//! roof of two faces each tilted 0.5° up to a ridge along x = 4, meeting at 179°. The ridge rounded
//! to radius 180.
use crate::sketch::polygon;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::prism;
use cad_kernel::fillet::roll;
use cad_kernel::geom::tolerance;
use cad_kernel::topo::Brep;

const HEIGHT: f64 = 6.;
const ROUND: f64 = 180.;

pub fn build() -> Result<Brep, String> {
    let ridge = 5. + 4. * 0.5_f64.to_radians().tan();
    // the floor, counter-clockwise from above so it faces up, swept up
    let floor = polygon(&[[0., 0., 0.], [8., 0., 0.], [8., 5., 0.], [4., ridge, 0.], [0., 5., 0.]]);
    let block = prism(&floor, 0., HEIGHT)?;
    // the ridge is picked by its middle; the ball rolls between the two roof faces
    let round = roll(&block, [4., ridge, HEIGHT / 2.], ROUND, tolerance(block.size()))?;
    // a convex edge's round takes its piece away
    let op = if round.concave { Op::Union } else { Op::Cut };
    Ok(combined(&block, &round.piece, op, 0.)?)
}
