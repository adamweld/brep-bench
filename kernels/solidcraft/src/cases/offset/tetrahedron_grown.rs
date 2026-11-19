//! The regular tetrahedron with corners (3, 3, 3), (3, −3, −3), (−3, 3, −3) and (−3, −3, 3) grown
//! by 1: every point within 1 of it, its edges pieces of cylinders and its corners pieces of a
//! ball, the kernel's offset with its convex edges rounded. The tetrahedron is SolidCraft's
//! convex polyhedron of the four half-spaces its faces bound, taken to ops through its record.
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::Body;
use solidcraft_kernel::{brep_record, convex_polyhedron, HalfSpace};
use solidcore_record::Vec3;

pub fn build() -> Result<Body, Refusal> {
    // the face opposite corner c lies on the plane −c·x = 9, at √3 from the centre, facing away
    // from c
    let corners = [[3.0, 3.0, 3.0], [3.0, -3.0, -3.0], [-3.0, 3.0, -3.0], [-3.0, -3.0, 3.0]];
    let s = 27f64.sqrt();
    let faces: Vec<HalfSpace> = corners
        .iter()
        .map(|c| HalfSpace { n: Vec3::new(-c[0] / s, -c[1] / s, -c[2] / s), d: 3f64.sqrt() })
        .collect();
    let tetrahedron = convex_polyhedron(&faces)?;
    let (tetrahedron, _) = Body::from_record(&brep_record(&tetrahedron)?, true)?;
    offset_rounded(&tetrahedron, 1.0)
}
