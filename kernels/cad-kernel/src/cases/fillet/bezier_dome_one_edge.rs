//! The block under the bicubic Bézier dome of `nurbs/bezier-dome` (the square 0 ≤ x, y ≤ 20,
//! its floor at z = 0), the edge where the dome meets the side y = 0 rounded to radius 1.
//!
//! `fillet::roll` stands the ball's centre off each face along its normal. A freeform patch has no
//! offset in closed form, and until 0.14.0 the kernel refused it ("no ball rolls on it yet");
//! since, the patch's offset is read by its signed distance (`Surface::Offset`), on the sheet
//! carried on past its domain by its own polynomials, and the ball rolls on it.
use crate::cases::nurbs::bezier_dome;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::fillet::roll;
use cad_kernel::geom::tolerance;
use cad_kernel::topo::Brep;

const ROUND: f64 = 1.;

pub fn build() -> Result<Brep, String> {
    let block = bezier_dome::build()?;
    // the edge picked by its middle: at x = 10 (u = ½) the patch's edge v = 0 stands at the
    // Bernstein mean of its heights there, (4 + 3·6 + 3·5 + 3)/8 = 5
    let round = roll(&block, [bezier_dome::SIDE / 2., 0., 5.], ROUND, tolerance(block.size()))?;
    let op = if round.concave { Op::Union } else { Op::Cut };
    Ok(combined(&block, &round.piece, op, 0.)?)
}
