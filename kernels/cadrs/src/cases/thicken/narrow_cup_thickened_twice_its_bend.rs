//! A narrow cup's skin thickened inward by twice its bend's radius. A sheet of revolution about the
//! z axis, a narrow cup: a flat floor, the disk of radius 3 in the plane z = 0; a bend, the quarter
//! of a torus whose tube has radius 2 about the circle of radius 3 at z = 2, from the floor's edge
//! out and up to the circle of radius 5 at z = 2; and a wall, the cylinder of radius 5 from z = 2
//! up to z = 10, thickened by 4 to the inside, up from the floor and in from the wall.
//!
//! The profile sketched on the Front plane as an open chain and surface-revolved a whole turn about
//! the z axis; cadrs's Thicken thickens it with its arrow pointing up from the floor. cadrs
//! declines it: its Thicken has no slab for a torus face moved past its tube's center line. (The
//! other way to the same solid, a cylinder with its bottom edge rounded and shelled open at the
//! top, it declines for the same face.)
use crate::bench::{arc, p2, p3, v3};
use crate::shared::thicken::{open_chain, surface_revolve, thicken_toward};
use crate::Refusal;
use cadrs_kernel::{Axis, BodyId, Curve2, DefaultKernel, Plane};

const THICKNESS: f64 = 4.0;

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let profile = vec![
        Curve2::line(p2(0.0, 0.0), p2(3.0, 0.0)),
        arc(p2(3.0, 2.0), 2.0, 270.0, 90.0),
        Curve2::line(p2(5.0, 2.0), p2(5.0, 10.0)),
    ];
    let axis = Axis::new(p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0));
    let sheet = surface_revolve(k, &open_chain(Plane::xz(0.0), profile), axis, None)?;
    let on_sheet = p3(1.5, 0.0, 0.0);
    thicken_toward(k, sheet, on_sheet, v3(0.0, 0.0, 1.0), THICKNESS)
}
