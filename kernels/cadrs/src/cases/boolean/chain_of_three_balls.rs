//! Three balls united one after another: radius 5 about the origin, radius 4 about (7, 0, 0) and
//! radius 3 about (10, 4, 0). The middle ball overlaps both others; the first and the last do not
//! touch. Each ball is cadrs's sphere primitive.
use crate::shared::boolean::chain_of_three_balls;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    chain_of_three_balls(k)
}
