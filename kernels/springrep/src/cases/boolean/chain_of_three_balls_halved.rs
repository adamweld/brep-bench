//! The chain of three balls of `boolean/chain-of-three-balls` cut by the plane z = 0 through all
//! three centres, keeping what lies above it.
use springrep::boolean::split;
use springrep::math::Vec3;
use springrep::topo::Brep;

use crate::shared::boolean::chain_of_three_balls;
use crate::shared::e;

pub fn build() -> Result<Brep, String> {
    let chain = chain_of_three_balls()?;
    // the part on the plane normal's side comes first
    let (above, _) = split(&chain, Vec3::ZERO, Vec3::Z).map_err(e)?;
    Ok(above)
}
