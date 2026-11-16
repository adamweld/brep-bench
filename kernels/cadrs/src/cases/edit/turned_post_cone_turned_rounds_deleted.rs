//! A turned post's rounded shoulder deleted, its cone made turned a radian about the axis: the cylinder and the cone meet again.
//! cadrs's primitives, the cone turned, the shoulder filleted; then Delete face on the round.
use crate::bench::{cone, cylinder, p3, turned, union, v3};
use crate::shared::edit::deleted;
use crate::shared::fillet::fillet;
use crate::Refusal;
use cadrs_kernel::{Axis, BodyId, DefaultKernel, Pick};
pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let z = v3(0.0, 0.0, 1.0);
    let c = cylinder(k, p3(0.0, 0.0, 0.0), z, 5.0, 6.0)?;
    let f = cone(k, p3(0.0, 0.0, 6.0), z, 5.0, 2.0, 4.0)?;
    let f = turned(k, f, Axis::new(p3(0.0, 0.0, 0.0), z), 1f64.to_degrees())?;
    let b = union(k, c, f)?;
    let mut e = Vec::new();
    for t in (0..8).map(|i| 0.1 + i as f64 * std::f64::consts::FRAC_PI_4) {
        let x = k.edge_at(b, p3(5.0 * t.cos(), 5.0 * t.sin(), 6.0))?;
        if !e.contains(&x) {
            e.push(x);
        }
    }
    let r = fillet(k, b, &e, 1.0)?;
    let mut face = Vec::new();
    for t in (0..8).map(|i| 0.1 + i as f64 * std::f64::consts::FRAC_PI_4) {
        for x in k.faces_at(r, &[p3(4.948683298050514 * t.cos(), 4.948683298050514 * t.sin(), 5.982894432683505)])? {
            if !face.contains(&x) {
                face.push(x);
            }
        }
    }
    deleted(k, r, &face)
}
