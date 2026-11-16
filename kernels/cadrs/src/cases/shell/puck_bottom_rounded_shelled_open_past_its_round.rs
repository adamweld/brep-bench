//! A puck with its bottom rim rounded, shelled open at its top past the round's radius: the cylinder of radius 6 from z = 0 to 10, its bottom rim rounded to radius 2,
//! shelled inward open at its top to walls 3 thick. cadrs's cylinder, filleted, then its shell.
use crate::bench::{cylinder, p3, v3};
use crate::shared::fillet::fillet;
use crate::shared::shell::{shell_inward, Joins};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let puck = cylinder(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 6.0, 10.0)?;
    let edges = k.edges_at(puck, &[p3(0.0, -6.0, 0.0)])?;
    let rounded = fillet(k, puck, &edges, 2.0)?;
    shell_inward(k, rounded, &[p3(0.0, 0.0, 10.0)], 3.0, Joins::Rounded)
}
