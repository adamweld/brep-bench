//! A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a V-groove turned
//! into it: everything farther from the axis than 3 + |z − 10| cut away, by the section through
//! (r, z) = (3, 10), (7, 6), (7, 14) turned about z. Then both of the groove's cones deleted
//! together: the cylinder below and above runs on across the groove.
use crate::bench::{cut, polygon};
use crate::shared::defeature::delete_faces;
use crate::Refusal;
use solidcore_ops::sweep::revolve;
use solidcore_ops::{make_cylinder, Body};
use std::f64::consts::TAU;

pub fn build() -> Result<Body, Refusal> {
    let shaft = make_cylinder([0.0, 0.0, 0.0], 5.0, 20.0)?;
    let section = polygon(&[[3.0, 0.0, 10.0], [7.0, 0.0, 6.0], [7.0, 0.0, 14.0]]);
    let cutter = revolve(&section, [0.0; 3], [0.0, 0.0, 1.0], TAU, true)?;
    let grooved = cut(shaft, cutter)?;
    // the lower cone and the upper, each at radius 4
    delete_faces(&grooved, &[[0.0, 4.0, 9.0], [0.0, 4.0, 11.0]])
}
