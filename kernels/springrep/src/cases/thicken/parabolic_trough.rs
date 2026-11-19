//! The parabolic cylinder z = x^2/8 for -4 <= x <= 4 and 0 <= y <= 10, a sheet, thickened by 1 to
//! its concave side, toward the focus: the strip between the parabola and its offset, extruded.
use springrep::math::Vec2;
use springrep::profile::{NamedSegment, Segment};
use springrep::topo::Brep;

use crate::shared::thicken::{line, swept_along_y, thickened};

pub fn build() -> Result<Brep, String> {
    // The sheet is the quadratic Bezier through (-4, 2), (0, -2), (4, 2), each (x, z), exactly the
    // parabola, swept along y: the B-spline surface of degree 2 by 1 with no interior knots. It is
    // a face of the solid above it closed by the planes through its straight edges along its
    // normals there, which lean in at 45 degrees and meet at (0, 6).
    let parabola = Segment::Bezier {
        ctrl: vec![Vec2::new(-4., 2.), Vec2::new(0., -2.), Vec2::new(4., 2.)],
        weights: vec![1., 1., 1.],
    };
    let trough = swept_along_y(
        vec![
            NamedSegment::new(parabola, "sheet"),
            line([4., 2.], [0., 6.], "right"),
            line([0., 6.], [-4., 2.], "left"),
        ],
        10.,
        "trough",
    )?;
    // the sheet's outward normal points away from the focus: thicken into the solid
    thickened(&trough, &["trough.sheet"], -1.)
}
