use crate::solids::*;
use solidcraft_kernel::{PathSeg, SpaceBezier};

/// The curve's control points.
const CURVE: [(f64, f64, f64); 4] =
    [(0.0, 0.0, 0.0), (12.0, 16.0, 0.0), (26.0, 8.0, 0.0), (24.0, 0.0, 16.0)];

/// The disc of radius 8 in the plane square to the curve where it leaves the origin, along
/// (3, 4, 0), swept along the cubic, a Bézier span in space (`PathSeg::Bezier`).
pub fn build() -> R<Body> {
    let curve = SpaceBezier::cubic(CURVE.map(|(x, y, z)| v3(x, y, z)))
        .ok_or_else(|| KernelError::Invalid("the cubic".into()))?;
    let start = Plane { origin: Vec3::ZERO, x: v3(-0.8, 0.6, 0.0), y: Vec3::Z };
    solidcraft_kernel::sweep(&start, &disc((0.0, 0.0), 8.0), &[PathSeg::Bezier(curve)])
}
