//! Two balls united, ten thousand times `scale/balls-united-a-million-from-the-origin` and at the
//! origin: radius 50000 about the origin and radius 40000 about (20000, 30000, 60000), 70000 from
//! the first. Each ball is cadrs's sphere primitive.
use crate::bench::{ball, p3, union};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let a = ball(k, p3(0.0, 0.0, 0.0), 50000.0)?;
    let b = ball(k, p3(20000.0, 30000.0, 60000.0), 40000.0)?;
    union(k, a, b)
}
