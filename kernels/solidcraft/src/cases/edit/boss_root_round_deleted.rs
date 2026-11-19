//! A plate, the cylinder of radius 8 on the z axis from z = -4 to z = 0, with a boss on it, the
//! cylinder of radius 3 from z = 0 to z = 8, the concave edge at the boss's foot rounded to
//! radius 1; then the round deleted, the plate's top and the boss's side run on until they meet
//! in the foot's circle again.
use crate::bench::fuse;
use crate::shared::edit::{delete_rounds, rounded};
use crate::Refusal;
use solidcore_ops::{make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let bossed = fuse(make_cylinder([0.0, 0.0, -4.0], 8.0, 4.0)?, make_cylinder([0.0; 3], 3.0, 8.0)?)?;
    let foot = [0.0, 3.0, 0.0];
    let bossed = rounded(&bossed, &[foot], 1.0)?;
    // the round: the face nearest where the foot's edge was
    delete_rounds(&bossed, &[foot])
}
