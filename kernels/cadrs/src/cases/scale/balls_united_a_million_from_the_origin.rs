//! Two balls a million from the origin, united: radius 5 about c = (600000, −800000, 0) and
//! radius 4 about c + (2, 3, 6), 7 from the first along a direction no axis lines up with. Each
//! ball is cadrs's sphere primitive, placed where it is.
use crate::bench::{ball, p3, union};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let a = ball(k, p3(600000.0, -800000.0, 0.0), 5.0)?;
    let b = ball(k, p3(600002.0, -799997.0, 6.0), 4.0)?;
    union(k, a, b)
}
