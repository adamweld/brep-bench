//! The block on the square 0 ≤ x, y ≤ 10 capped by the bilinearly blended Coons patch of four
//! parabolic arcs, each the quadratic Bézier of its three control points in an upright side:
//! arching up to z = 12 on y = 0 and x = 0, sagging to z = 8 on x = 10 and y = 10.
//!
//! The patch is solidcore-ops's boundary patch of the four arcs (`surface::boundary`), whose side
//! at each arc is that arc point for point at the same fraction of its parameter: the Coons
//! patch the case describes, blended on the arcs' own parameters. (SolidCraft's kernel's boundary
//! surface takes each curve by its arc length, so its Coons patch of these arcs, whose speeds
//! vary, is a different surface.) Under it, the four upright sides, each bounded by its arc, and
//! the floor, sewn to the patch into one solid.
use crate::shared::nurbs::{bezier_knots, polygon, segment, sheet, spline_curve, Face, SEW};
use crate::Refusal;
use solidcore_ops::stitch::sew;
use solidcore_ops::surface::boundary;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let knots = bezier_knots(2);
    // round the loop: on y = 0, x = 10, y = 10 and x = 0
    let arcs: [[[f64; 3]; 3]; 4] = [
        [[0.0, 0.0, 10.0], [5.0, 0.0, 14.0], [10.0, 0.0, 10.0]],
        [[10.0, 0.0, 10.0], [10.0, 5.0, 6.0], [10.0, 10.0, 10.0]],
        [[10.0, 10.0, 10.0], [5.0, 10.0, 6.0], [0.0, 10.0, 10.0]],
        [[0.0, 10.0, 10.0], [0.0, 5.0, 14.0], [0.0, 0.0, 10.0]],
    ];
    let curves: Vec<_> = arcs.iter().map(|a| spline_curve(&knots, a)).collect();
    let (patch, _) = Body::from_record(&boundary(&curves)?, true)?;
    // each side: down from the arc's end to the floor, along the floor, up to its start, the arc
    let floor = |p: [f64; 3]| [p[0], p[1], 0.0];
    let mut faces: Vec<Face> = arcs
        .iter()
        .zip(&curves)
        .map(|(a, arc)| {
            let (start, end) = (a[0], a[2]);
            Face::Flat(vec![segment(end, floor(end)), segment(floor(end), floor(start)), segment(floor(start), start), arc.clone()])
        })
        .collect();
    faces.push(Face::Flat(polygon(&[[0.0; 3], [10.0, 0.0, 0.0], [10.0, 10.0, 0.0], [0.0, 10.0, 0.0]])));
    let sewn = sew(&[&patch, &sheet(&faces)?], SEW)?;
    if !sewn.closed {
        return Err("the patch and the faces under it sewn do not close into a solid".into());
    }
    Ok(sewn.body)
}
