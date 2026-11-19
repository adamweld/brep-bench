use super::chain_of_three_balls::chain;
use crate::solids::*;

/// The chain of `boolean/chain-of-three-balls` kept above the plane z = 0 through its three
/// centers, the positive side of `Plane::XY`, whose normal is +z.
pub fn build() -> Result<Body, crate::Refusal> {
    let (above, _) = solidcraft_kernel::half_space(&chain()?, &Plane::XY, true)?;
    Ok(above.ok_or(format!("{EMPTY}nothing of the chain lies above z = 0"))?)
}
