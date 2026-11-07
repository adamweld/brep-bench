//! A block, the box (−6, −6, −8)–(36, 26, 0), with a groove cut into its top by a ring standing up
//! like a wheel — the torus centred at (0, 0, 0.5) about the y direction, of radii 1.5 and 0.5 —
//! drawn in a frame that rides along the cubic Bézier curve (0, 0, 0), (10, 0, 0), (20, 20, 0),
//! (30, 20, 0) — its x axis the curve's tangent, its z axis up — and carried rigidly by it over the
//! whole curve.
//!
//! Refused by the kernel. A frame along a path is no constant twist and no path of slides and
//! turns, so the sweep takes the kernel's road for any other rigid motion (K2), which reads the
//! tool as a convex solid of revolution and refuses the ring, a torus about an axis it does not
//! cross (`Unbuilt::Curved`).
use crate::sketch::block;
use cad_kernel::build::torus;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::nurbs::BSpline;
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Motion, Path, Trajectory};
use cad_kernel::KernelError;
use std::sync::Arc;

const CONTROL: [V; 4] = [[0., 0., 0.], [10., 0., 0.], [20., 20., 0.], [30., 20., 0.]];
/// How closely the ring is meshed for the sweep's membership reads: a hundredth of its tube's
/// radius.
const BAR: f64 = 0.005;

/// The frame riding along a curve in the plane z = 0: its origin the curve's point, its x axis the
/// curve's unit tangent, its z axis up; at each parameter its pose (a turn about z by the
/// tangent's angle, then the curve's point) and that pose's rate, the turn's by the tangent's
/// rate of turning, read off the kernel's own B-spline.
#[derive(Debug)]
struct Along(BSpline);

impl Path for Along {
    fn at(&self, t: f64) -> Result<Motion, KernelError> {
        let (point, d, dd) = self.0.d2(t);
        let speed2 = d[0] * d[0] + d[1] * d[1];
        let (c, s) = (d[0] / speed2.sqrt(), d[1] / speed2.sqrt());
        let turning = (d[0] * dd[1] - d[1] * dd[0]) / speed2;
        let r = [[c, -s, 0.], [s, c, 0.], [0., 0., 1.]];
        let dr = [[-s * turning, -c * turning, 0.], [c * turning, -s * turning, 0.], [0.; 3]];
        Ok(Motion::of_parts(r, dr, point, d))
    }
}

pub fn build() -> Result<Brep, String> {
    let stock = block([-6., -6., -8.], [36., 26., 0.])?;
    let ring = torus([0., 0., 0.5], [0., 1., 0.], 1.5, 0.5)?;
    // the curve as cad-kernel holds a Bézier segment: a cubic B-spline on clamped knots, run over
    // [0, 1]; at its start the riding frame is the world's own
    let curve = BSpline::new(3, vec![0., 0., 0., 0., 1., 1., 1., 1.], CONTROL.to_vec())?;
    // a frame along a path is no constant twist and no path of slides and turns: the motion as the
    // kernel reads any other, by evaluating it (K2)
    let path = Trajectory::General(Arc::new(Along(curve)));
    let size = stock.size().max(ring.size());
    cut_from(&stock, &ring, &path, [0., 1.], BAR, tolerance(size))
        .map_err(|why| format!("the ring's sweep not built: {why:?}"))
}
