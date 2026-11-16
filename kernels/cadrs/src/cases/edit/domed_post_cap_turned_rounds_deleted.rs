//! A post capped by a ball, the cap made turned 1.5 radians about the axis, its rounded rim deleted: the cylinder and the ball meet again.
//! cadrs's primitives, the cap turned, the rim filleted; then Delete face on the round.
use crate::bench::{ball, block, common, cylinder, p3, turned, union, v3};
use crate::shared::edit::deleted;
use crate::shared::fillet::fillet;
use crate::Refusal;
use cadrs_kernel::{Axis, BodyId, DefaultKernel, Pick};
pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let z = v3(0.0, 0.0, 1.0);
    let b0 = ball(k, p3(0.0, 0.0, 3.0), 5.0)?;
    let above = block(k, p3(-6.0, -6.0, 6.0), p3(6.0, 6.0, 10.0))?;
    let cap = common(k, b0, above)?;
    let cap = turned(k, cap, Axis::new(p3(0.0, 0.0, 0.0), z), 1.5f64.to_degrees())?;
    let c = cylinder(k, p3(0.0, 0.0, 0.0), z, 4.0, 6.0)?;
    let b = union(k, c, cap)?;
    let mut e = Vec::new();
    for t in (0..8).map(|i| 0.1 + i as f64 * std::f64::consts::FRAC_PI_4) {
        let x = k.edge_at(b, p3(4.0 * t.cos(), 4.0 * t.sin(), 6.0))?;
        if !e.contains(&x) {
            e.push(x);
        }
    }
    let r = fillet(k, b, &e, 1.0)?;
    let mut f = Vec::new();
    for t in (0..8).map(|i| 0.1 + i as f64 * std::f64::consts::FRAC_PI_4) {
        for x in k.faces_at(r, &[p3(3.9426028923173684 * t.cos(), 3.9426028923173684 * t.sin(), 5.979667149854743)])? {
            if !f.contains(&x) {
                f.push(x);
            }
        }
    }
    deleted(k, r, &f)
}
