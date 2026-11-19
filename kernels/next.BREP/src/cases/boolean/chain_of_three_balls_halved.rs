//! The chain of `boolean/chain-of-three-balls` with everything below the plane z = 0, through all
//! three centres, cut away by a block reaching past it.
use crate::cases::boolean::r#chain_of_three_balls;
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let chain = chain_of_three_balls::build()?;
    let below = make_box_brep(p(-20.0, -20.0, -20.0), 50.0, 50.0, 20.0)?;
    Ok(cut(&chain, &below)?)
}
