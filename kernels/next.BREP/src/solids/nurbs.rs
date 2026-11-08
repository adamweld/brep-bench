//! Helpers shared by the NURBS cases that need a solid whose side is one rational B-spline face: a
//! cylinder written as the tensor product of the rational quadratic circle and a line.
#![allow(unused_imports)]
use super::*;
use crate::solids::geometry::{uv_line, Records};
use brep_kernel::{make_plane, BrepSolid, NurbsSurface, Vec4};
use std::f64::consts::FRAC_1_SQRT_2 as H;

/// The standard rational quadratic unit circle's nine control points, counter-clockwise from
/// (1, 0) and back: its weights are 1, √2/2, … on the knots (0, 0, 0, ¼, ¼, ½, ½, ¾, ¾, 1, 1, 1).
pub const CIRCLE_FROM_X: [(f64, f64); 9] = [(1.0, 0.0), (1.0, 1.0), (0.0, 1.0), (-1.0, 1.0),
    (-1.0, 0.0), (-1.0, -1.0), (0.0, -1.0), (1.0, -1.0), (1.0, 0.0)];

/// The solid cylinder of `radius` about the z axis from z = `z0` to z = `z1`, its side one NURBS
/// face: around the axis (u) the rational quadratic circle of the nine control points `circle`
/// (counter-clockwise, its first and last the same, weights 1, √2/2, … alternating) scaled by
/// `radius`, along it (v) degree 1 from z0 to z1. The side is closed on its seam, the line through
/// its first control point, and the ends are flat discs, each bounded by its rim, the side's edge
/// v = 0 or v = 1. No cylinder is used.
///
/// next.BREP has no operation that makes a face from a surface, so the solid is stated in the
/// kernel's own topology records (`solids::geometry::Records`) and validated by the kernel.
pub fn nurbs_cylinder(radius: f64, z0: f64, z1: f64, circle: &[(f64, f64); 9])
    -> Result<BrepSolid, String> {
    let weights = [1.0, H, 1.0, H, 1.0, H, 1.0, H, 1.0];
    let net = circle
        .iter()
        .zip(weights)
        .map(|(&(cx, cy), w)| {
            [z0, z1].iter().map(|&z| Vec4::from_point(p(radius * cx, radius * cy, z), w)).collect()
        })
        .collect();
    let round = vec![0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0];
    let side = NurbsSurface::new(2, 1, round, vec![0.0, 0.0, 1.0, 1.0], net)?;

    let mut b = Records::new();
    let foot = b.vertex(side.evaluate(0.0, 0.0)?);
    let head = b.vertex(side.evaluate(0.0, 1.0)?);
    let bottom = b.edge(side.iso_curve_v(0.0)?, foot, foot)?;
    let top = b.edge(side.iso_curve_v(1.0)?, head, head)?;
    let seam = b.edge(side.iso_curve_u(0.0)?, foot, head)?;
    // the side, its normal Su × Sv outward, bounded by the bottom rim, the seam up, the top rim
    // backwards and the seam down
    b.face(side, vec![
        (bottom, true, uv_line(0.0, 0.0, 1.0, 0.0)?),
        (seam, true, uv_line(1.0, 0.0, 1.0, 1.0)?),
        (top, false, uv_line(1.0, 1.0, 0.0, 1.0)?),
        (seam, false, uv_line(0.0, 1.0, 0.0, 0.0)?),
    ]);
    let (ex, ey) = (p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0));
    let span = 2.0 * radius;
    // the top, its normal x × y up, bounded by its rim counter-clockwise; the bottom, its normal
    // y × x down, by its rim run backwards
    b.flat(make_plane(p(-radius, -radius, z1), ex, ey, span, span)?, &[(top, true)])?;
    b.flat(make_plane(p(-radius, -radius, z0), ey, ex, span, span)?, &[(bottom, false)])?;
    b.solid(0)
}
