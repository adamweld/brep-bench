//! Two cone frustums base to base, the upper made turned two radians about the axis, their rounded ridge deleted: the two cones meet again.
//! cadrs's primitives, the upper cone turned, the ridge filleted; then Delete face on the round.
use crate::bench::{cone, p3, turned, union, v3};
use crate::shared::edit::deleted;
use crate::shared::fillet::fillet;
use crate::Refusal;
use cadrs_kernel::{Axis, BodyId, DefaultKernel, Pick};
pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let z = v3(0.0, 0.0, 1.0);
    let c = cone(k, p3(0.0, 0.0, 0.0), z, 3.0, 5.0, 4.0)?;
    let f = cone(k, p3(0.0, 0.0, 4.0), z, 5.0, 2.0, 4.0)?;
    let f = turned(k, f, Axis::new(p3(0.0, 0.0, 0.0), z), 2f64.to_degrees())?;
    let b = union(k, c, f)?;
    let mut e = Vec::new();
    for t in (0..8).map(|i| 0.1 + i as f64 * std::f64::consts::FRAC_PI_4) {
        let x = k.edge_at(b, p3(5.0 * t.cos(), 5.0 * t.sin(), 4.0))?;
        if !e.contains(&x) {
            e.push(x);
        }
    }
    let r = fillet(k, b, &e, 1.0)?;
    let mut face = Vec::new();
    for t in (0..8).map(|i| 0.1 + i as f64 * std::f64::consts::FRAC_PI_4) {
        for x in k.faces_at(r, &[p3(4.825138920703175 * t.cos(), 4.825138920703175 * t.sin(), 3.984232786315833)])? {
            if !face.contains(&x) {
                face.push(x);
            }
        }
    }
    deleted(k, r, &face)
}
