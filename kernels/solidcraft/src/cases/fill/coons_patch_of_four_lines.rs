use crate::solids::*;
use solidcore_record::Seg2;
use solidcraft_kernel::{boundary_surface, BoundaryCurve, Continuity, CurveSource};

/// The upright plane through `origin` along `along`, its sketch y up.
fn upright(origin: Vec3, along: Vec3) -> Plane {
    Plane { origin, x: along, y: Vec3::Z }
}

/// The straight edge in `plane` from (0, `z0`) to (10, `z1`) in its sketch.
fn edge<'a>(label: &str, plane: Plane, z0: f64, z1: f64) -> BoundaryCurve<'a> {
    let segs = vec![Seg2::Line { a: v2(0.0, z0), b: v2(10.0, z1) }];
    BoundaryCurve { label: label.into(), source: CurveSource::Sketch { plane, segs }, continuity: Continuity::G0 }
}

/// The block on the square 0 ≤ x, y ≤ 10 capped by the bilinearly blended Coons patch of the skew
/// quadrilateral through (0, 0, 5), (10, 0, 5), (10, 10, 15) and (0, 10, 5): the kernel's boundary
/// surface of two curves each way, the edges on y = 0 and y = 10 (along +x) and those on x = 0
/// and x = 10 (along +y). Under it, the four upright sides and the floor, each a planar patch of a
/// sketch region, all stitched into one solid.
pub fn build() -> Result<Body, crate::Refusal> {
    let (south, north) = (upright(Vec3::ZERO, Vec3::X), upright(v3(0.0, 10.0, 0.0), Vec3::X));
    let (west, east) = (upright(Vec3::ZERO, Vec3::Y), upright(v3(10.0, 0.0, 0.0), Vec3::Y));
    let patch = boundary_surface(
        &[edge("y = 0", south, 5.0, 5.0), edge("y = 10", north, 5.0, 15.0)],
        &[edge("x = 0", west, 5.0, 5.0), edge("x = 10", east, 5.0, 15.0)],
    )?;
    let rising = polygon(&[(0.0, 0.0), (10.0, 0.0), (10.0, 15.0), (0.0, 5.0)]);
    let sides = [
        solidcraft_kernel::patch_region(&south, &rect(0.0, 0.0, 10.0, 5.0))?,
        solidcraft_kernel::patch_region(&west, &rect(0.0, 0.0, 10.0, 5.0))?,
        solidcraft_kernel::patch_region(&north, &rising)?,
        solidcraft_kernel::patch_region(&east, &rising)?,
        solidcraft_kernel::patch_region(&Plane::XY, &rect(0.0, 0.0, 10.0, 10.0))?,
    ];
    let faces: Vec<&Body> = [&patch].into_iter().chain(&sides).collect();
    Ok(solidcraft_kernel::stitch(&faces, 1e-6)?)
}
