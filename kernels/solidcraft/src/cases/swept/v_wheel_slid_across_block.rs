//! A block, the box from (0, −5, 0) to (20, 5, 10), with a V groove cut across its top by a wheel
//! with a sharp rim: the solid of revolution about the line through (−8, 0, 14) along y of the
//! triangle with corners (y, z) = (0, 8), (−3 tan 30°, 11) and (3 tan 30°, 11) on x = −8, slid
//! without turning along x by 36.
//!
//! solidcraft-kernel has no sweep of a solid, so the wheel's sweep is built by elementary
//! geometry. Split by the plane x = −8 through its axis, square to the slide, the wheel's back
//! half leads with its flat cut faces and its bore (whose normals point to the axis, forward);
//! its flanks face back. So it sweeps itself, its section through the axis slid 36 and its bore
//! slid 36; the front half likewise sweeps the section and its bore slid, and itself where it
//! stops. The halves lie wholly before x = −8 and past x = 28, the bore slid at z ≥ 11 and the
//! section's upper triangle above z = 17, all clear of the block; so the section's lower triangle
//! extruded along x by 36 is what the sweep takes from the block.
use crate::solids::*;

pub fn build() -> R<Body> {
    let stock = block((0.0, -5.0, 0.0), (20.0, 5.0, 10.0))?;
    let w = 3.0 * 30f64.to_radians().tan();
    let triangle = polygon(&[(0.0, 8.0), (w, 11.0), (-w, 11.0)]);
    let plane = Plane { origin: v3(-8.0, 0.0, 0.0), x: Vec3::Y, y: Vec3::Z };
    cut(&stock, &extrude(&plane, triangle, 0.0, 36.0)?)
}
