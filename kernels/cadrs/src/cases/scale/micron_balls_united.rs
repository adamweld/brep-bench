//! Two balls united, a thousand times smaller than `scale/balls-united-a-million-from-the-origin`
//! and at the origin: radius 0.005 about the origin and radius 0.004 about (0.002, 0.003, 0.006),
//! 0.007 from the first. Each ball is cadrs's sphere primitive.
use crate::bench::{ball, p3, union};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let a = ball(k, p3(0.0, 0.0, 0.0), 0.005)?;
    let b = ball(k, p3(0.002, 0.003, 0.006), 0.004)?;
    union(k, a, b)
}
