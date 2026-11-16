//! A turned post with its shoulder rounded, shrunk by 1.5, past the round: the cylinder of radius 5 from z = 0 to 6 topped by the frustum from radius 5 to 2 over z = 6 to 10,
//! the edge between them rounded to radius 1, shrunk by 1.5. cadrs's cylinder and cone united, filleted, then its offset (shrunk).
use crate::bench::{cone, cylinder, p3, union, v3};
use crate::shared::fillet::fillet;
use crate::shared::offset::shrunk;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

const DISTANCE: f64 = 1.5;

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let z = v3(0.0, 0.0, 1.0);
    let c = cylinder(k, p3(0.0, 0.0, 0.0), z, 5.0, 6.0)?;
    let f = cone(k, p3(0.0, 0.0, 6.0), z, 5.0, 2.0, 4.0)?;
    let post = union(k, c, f)?;
    let shoulder = k.edge_at(post, p3(0.0, -5.0, 6.0))?;
    let rounded = fillet(k, post, &[shoulder], 1.0)?;
    shrunk(k, rounded, DISTANCE)
}
