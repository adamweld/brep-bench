//! No layer has a conic blend (the kernel's and ops's blends have a circular, curvature-continuous,
//! chord-sized or flat section; none takes a rho), so the block is cut by the corner outside the
//! parabola instead: the parabola, the quadratic Bézier through (17, 10), (20, 10) and (20, 7) in
//! x and z (a conic of weight 1), closed round the corner outside the block and extruded along y
//! past both ends.
use crate::solids::*;
use solidcore_record::Seg2;

pub fn build() -> R<Body> {
    let block = block((0.0, 0.0, 0.0), (20.0, 10.0, 10.0))?;
    let (on_top, on_end) = (v2(17.0, 10.0), v2(20.0, 7.0));
    let mut segs = vec![Seg2::Conic { a: on_top, apex: v2(20.0, 10.0), b: on_end, w: 1.0 }];
    let around = [on_end, v2(21.0, 7.0), v2(21.0, 11.0), v2(17.0, 11.0), on_top];
    segs.extend(around.windows(2).map(|p| Seg2::Line { a: p[0], b: p[1] }));
    let outside = Region2 { outer: Loop2 { segs }, holes: vec![] };
    // the plane of `xz` has its normal along −y: from y = 11 to y = −1
    cut(&block, &extrude(&xz(), outside, -11.0, 1.0)?)
}
