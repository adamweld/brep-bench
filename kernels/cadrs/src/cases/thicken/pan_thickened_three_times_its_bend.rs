//! A pan with a tight bend thickened inward by three times the bend's radius. A sheet of revolution
//! about the z axis, a pan with a tight bend: a flat floor, the disk of radius 5 in the plane z =
//! 0; a bend, the quarter of a torus whose tube has radius 1 about the circle of radius 5 at z = 1,
//! from the floor's edge out and up to the circle of radius 6 at z = 1; and a wall, the cylinder of
//! radius 6 from z = 1 up to z = 8, thickened by 3 to the inside, up from the floor and in from the
//! wall.
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

const THICKNESS: f64 = 3.0;

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let profile = vec![
        Curve2::line(p2(0.0, 0.0), p2(5.0, 0.0)),
        arc(p2(5.0, 1.0), 1.0, 270.0, 90.0),
        Curve2::line(p2(6.0, 1.0), p2(6.0, 8.0)),
    ];
    let axis = Axis::new(p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0));
    let sheet = surface_revolve(k, &open_chain(Plane::xz(0.0), profile), axis, None)?;
    let on_sheet = p3(2.5, 0.0, 0.0);
    thicken_toward(k, sheet, on_sheet, v3(0.0, 0.0, 1.0), THICKNESS)
}
