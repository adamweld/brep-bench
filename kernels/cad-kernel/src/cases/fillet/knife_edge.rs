//! A wedge from z = 0 to 6 on the triangle (0, 0), (10, 0), (10, 10 tan 5°), its knife edge of 5°
//! along the z axis rounded to radius 0.1.
use crate::sketch::polygon;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::prism;
use cad_kernel::fillet::roll;
use cad_kernel::geom::tolerance;
use cad_kernel::topo::Brep;

const HEIGHT: f64 = 6.;
const ROUND: f64 = 0.1;

pub fn build() -> Result<Brep, String> {
    let rise = 10. * 5_f64.to_radians().tan();
    // the floor, counter-clockwise from above so it faces up, swept up; its sides named, since a
    // round reads faces of one name as pieces of one face (the caps are `far` and `near`)
    let mut floor = polygon(&[[0., 0., 0.], [10., 0., 0.], [10., rise, 0.]]);
    floor.names = vec![["bottom", "back", "blade"].map(String::from).to_vec()];
    let wedge = prism(&floor, 0., HEIGHT)?;
    // the knife edge is picked by its middle; the ball rolls in between the blade's two faces
    let round = roll(&wedge, [0., 0., HEIGHT / 2.], ROUND, tolerance(wedge.size()))?;
    // a convex edge's round takes its piece away
    let op = if round.concave { Op::Union } else { Op::Cut };
    Ok(combined(&wedge, &round.piece, op, 0.)?)
}
