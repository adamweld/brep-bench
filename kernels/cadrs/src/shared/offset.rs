//! What the cadrs implementations of the offset cases share: cadrs's offset of a solid's faces.
use crate::bench::{ball, cone, p3, union, v3};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Kernel, OffsetSpec};

/// `body` offset by `distance` (inward when negative). With `sharp`, every face moves and the
/// moved faces are extended until they meet; without it, the result is the solid of the points
/// within `distance` of `body` (or at least −`distance` from its outside), its edges rounded
/// where the distance makes them round.
pub fn offset(
    k: &mut DefaultKernel,
    body: BodyId,
    distance: f64,
    sharp: bool,
) -> Result<BodyId, Refusal> {
    let spec = OffsetSpec::by(distance);
    Ok(k.offset(body, &if sharp { spec.mitred() } else { spec })?.first()?)
}

/// `body` grown by `distance`: the solid of every point within `distance` of it.
pub fn grown(k: &mut DefaultKernel, body: BodyId, distance: f64) -> Result<BodyId, Refusal> {
    offset(k, body, distance, false)
}

/// `body` shrunk by `distance`: the solid of every point at least `distance` from its outside.
pub fn shrunk(k: &mut DefaultKernel, body: BodyId, distance: f64) -> Result<BodyId, Refusal> {
    offset(k, body, -distance, false)
}

/// The cone on the z axis from its base of radius 6 at z = 0 to its apex at z = 12, its tip rounded
/// by a ball of radius 1 tangent to its side: the frustum up to the circle of tangency (radius 2/√5
/// at z = 12 − 4/√5) united with the ball about (0, 0, 12 − √5).
pub fn ball_tipped_cone(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let root = 5f64.sqrt();
    let (base, z) = (p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0));
    let frustum = cone(k, base, z, 6.0, 2.0 / root, 12.0 - 4.0 / root)?;
    let tip = ball(k, p3(0.0, 0.0, 12.0 - root), 1.0)?;
    union(k, frustum, tip)
}
