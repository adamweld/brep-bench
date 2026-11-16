//! The box from the origin to (20, 12, 8), its four upright edges rounded to radius 3, then its
//! whole top rim rounded to radius 1 (one tangent chain of eight faces); then the chain deleted.
//! The block filleted twice, the rim's eight edges picked one by one; then Delete face.
use crate::bench::{block, p3};
use crate::shared::edit::deleted;
use crate::shared::fillet::fillet;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = block(k, p3(0.0, 0.0, 0.0), p3(20.0, 12.0, 8.0))?;
    let mut uprights = Vec::new();
    for (x, y) in [(0.0, 0.0), (20.0, 0.0), (20.0, 12.0), (0.0, 12.0)] {
        uprights.push(k.edge_at(b, p3(x, y, 4.0))?);
    }
    let rounded = fillet(k, b, &uprights, 3.0)?;
    let (q, q3) = (0.5f64.sqrt(), 3.0 * 0.5f64.sqrt());
    let mut rim = Vec::new();
    for (x, y) in [
        (10.0, 0.0), (20.0, 6.0), (10.0, 12.0), (0.0, 6.0),
        (3.0 - q3, 3.0 - q3), (17.0 + q3, 3.0 - q3), (17.0 + q3, 9.0 + q3), (3.0 - q3, 9.0 + q3),
    ] {
        rim.push(k.edge_at(rounded, p3(x, y, 8.0))?);
    }
    let chained = fillet(k, rounded, &rim, 1.0)?;
    // each face of the chain halfway round its tube
    let (s, a) = (q, 1.0 - q);
    let corner = |cx: f64, cy: f64, dx: f64, dy: f64| p3(cx + dx * (2.0 + s), cy + dy * (2.0 + s), 7.0 + s);
    let chain = k.faces_at(chained, &[
        p3(10.0, a, 7.0 + s), p3(10.0, 12.0 - a, 7.0 + s), p3(a, 6.0, 7.0 + s), p3(20.0 - a, 6.0, 7.0 + s),
        corner(3.0, 3.0, -q, -q), corner(17.0, 3.0, q, -q), corner(17.0, 9.0, q, q), corner(3.0, 9.0, -q, q),
    ])?;
    deleted(k, chained, &chain)
}
