//! A block, the box from (−3, −3, −3) to (3, 3, 3), less everything the cube from (−1, −1, −1) to
//! (1, 1, 1) passes through as it turns a whole turn about its long diagonal, the line through
//! (−1, −1, −1) and (1, 1, 1): a sealed void.
//!
//! solidcraft-kernel has no sweep of a solid, so the turn is built by elementary geometry: turned
//! a whole turn about an axis, a solid sweeps its shadow, the circles about the axis it meets,
//! revolved. The cube holds the diagonal, so the shadow at h along it (from the centre) reaches
//! out from the axis to the cube's farthest point in the plane square to it there, a corner of
//! the section on one of the cube's edges. Within 1/√3 of the ends (|h| ≥ 1/√3) those are the
//! three edges through the end, whose points are √2 (√3 − |h|) from the axis: two cones. Between,
//! they are the six edges skew to the diagonal, such as (−1, t, 1), at distance √(2 + 2h²): a
//! hyperbola, r² − 2h² = 2, which meets the cones' lines at r = √(8/3). The hyperbola's arc is
//! the conic through those two points with its vertex at (√2, 0): its tangents there meet at
//! (2/√(8/3), 0), and the conic's middle weight puts its midpoint at the vertex.
use crate::solids::*;
use solidcore_record::Seg2;

pub fn build() -> R<Body> {
    let block = block((-3.0, -3.0, -3.0), (3.0, 3.0, 3.0))?;
    let s3 = 3f64.sqrt();
    let (r0, h0) = ((8.0f64 / 3.0).sqrt(), 1.0 / s3); // where the cones meet the hyperbola
    let apex = 2.0 / r0; // where the hyperbola's tangents at (r0, ±h0) cross h = 0
    let vertex = 2f64.sqrt();
    let w = (r0 - vertex) / (vertex - apex);
    // the shadow in (r, h): out from the axis, along it from h = −√3 to √3
    let line = |a: (f64, f64), b: (f64, f64)| Seg2::Line { a: v2(a.0, a.1), b: v2(b.0, b.1) };
    let shadow = vec![
        line((0.0, -s3), (r0, -h0)),
        Seg2::Conic { a: v2(r0, -h0), apex: v2(apex, 0.0), b: v2(r0, h0), w },
        line((r0, h0), (0.0, s3)),
        line((0.0, s3), (0.0, -s3)),
    ];
    // a plane through the diagonal: its sketch y along the diagonal, its sketch x square to it
    let diagonal = v3(1.0, 1.0, 1.0) * (1.0 / s3);
    let across = v3(1.0, -1.0, 0.0) * (1.0 / 2f64.sqrt());
    let plane = Plane { origin: Vec3::ZERO, x: across, y: diagonal };
    let shadow = Region2 { outer: Loop2 { segs: shadow }, holes: vec![] };
    let void = one(solidcraft_kernel::revolve(&plane, &[shadow], Vec2::ZERO, Vec2::Y, std::f64::consts::TAU)?)?;
    cut(&block, &void)
}
