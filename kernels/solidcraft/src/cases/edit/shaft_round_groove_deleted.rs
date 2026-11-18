//! A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a round-bottomed
//! groove turned into it: the torus about the z axis whose tube, of radius 1.5, runs round the
//! circle of radius 5 at z = 10, cut away. Then the groove's face, the torus's inner half,
//! deleted: the cylinder below and above runs on across the groove.
use crate::bench::cut;
use crate::shared::defeature::delete_faces;
use crate::Refusal;
use solidcore_ops::{make_cylinder, make_torus, Body};

pub fn build() -> Result<Body, Refusal> {
    let shaft = make_cylinder([0.0, 0.0, 0.0], 5.0, 20.0)?;
    let tube = make_torus([0.0, 0.0, 10.0], 5.0, 1.5)?;
    let grooved = cut(shaft, tube)?;
    // the groove's bottom, at radius 3.5
    delete_faces(&grooved, &[[0.0, 3.5, 10.0]])
}
