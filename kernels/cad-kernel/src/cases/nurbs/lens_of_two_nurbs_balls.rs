//! The common part of two balls of radius 5 about (−3, 0, 0) and (3, 0, 0), each bounded by the
//! standard biquadratic NURBS sphere of `geometry/nurbs-sphere` moved to its centre (its poles
//! above and below the centre, its seam the meridian on the +x side of it): the lens of two caps
//! of height 2 base to base.
use crate::cases::geometry::nurbs_sphere;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::geom::Rigid;
use cad_kernel::topo::Brep;

const OFF: f64 = 3.;

pub fn build() -> Result<Brep, String> {
    let ball = nurbs_sphere::build()?;
    let at = |x: f64| ball.moved(&Rigid { t: [x, 0., 0.], ..Rigid::identity() });
    Ok(combined(&at(-OFF), &at(OFF), Op::Common, 0.)?)
}
