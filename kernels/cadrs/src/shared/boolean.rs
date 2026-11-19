//! What the cadrs implementations of the boolean cases share.
use crate::bench::{ball, p3, union_each};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

/// The chain of `boolean/chain-of-three-balls`: balls of radius 5, 4 and 3 about the origin,
/// (7, 0, 0) and (10, 4, 0) (cadrs's sphere primitive), the first two united, then the third
/// with that.
pub fn chain_of_three_balls(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let first = ball(k, p3(0.0, 0.0, 0.0), 5.0)?;
    let middle = ball(k, p3(7.0, 0.0, 0.0), 4.0)?;
    let last = ball(k, p3(10.0, 4.0, 0.0), 3.0)?;
    union_each(k, first, &[middle, last])
}
