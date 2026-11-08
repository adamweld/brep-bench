//! The box from the origin to (20, 10, 10) with the edge where its top meets its end x = 20
//! blended by a conic of rho 0.5, a parabola, tangent to the top and the end each 3 from the edge.
//!
//! next.BREP's blends are circular (constant or variable radius), with no conic blend. The blend's
//! section is the same in every plane y = c, so the blended block is its section extruded along y:
//! the rectangle 20 by 10 in the plane y = 0 with the corner (20, 10) replaced by the quadratic
//! Bézier curve from (17, 10) over (20, 10) to (20, 7), extruded 10 along +y.
use crate::solids::p;
use crate::solids::sweep::bezier;
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, make_line, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let section = vec![
        make_line(p(0.0, 0.0, 0.0), p(20.0, 0.0, 0.0))?,
        make_line(p(20.0, 0.0, 0.0), p(20.0, 0.0, 7.0))?,
        bezier(&[p(20.0, 0.0, 7.0), p(20.0, 0.0, 10.0), p(17.0, 0.0, 10.0)])?,
        make_line(p(17.0, 0.0, 10.0), p(0.0, 0.0, 10.0))?,
        make_line(p(0.0, 0.0, 10.0), p(0.0, 0.0, 0.0))?,
    ];
    Ok(extrude_profile_brep(&section, p(0.0, 1.0, 0.0), 10.0)?)
}
