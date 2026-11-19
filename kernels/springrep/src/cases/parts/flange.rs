//! A round flange on the z axis: a plate of radius 32 (z 0 to 6) and a hub of radius 15 (z 6 to
//! 16), bored through at radius 8, with six bolt holes of radius 3 on the circle of radius 24.
use std::f64::consts::{PI, TAU};

use springrep::boolean::difference_all;
use springrep::math::Vec3;
use springrep::primitive::cylinder;
use springrep::sweep::revolve;
use springrep::topo::Brep;

use crate::shared::parts::{polygon, xz};
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    // the half section, from the bore out to the rim and up the hub, turned about the z axis
    let section = polygon(&[(8., 0.), (32., 0.), (32., 6.), (15., 6.), (15., 16.), (8., 16.)], "turned");
    let body = revolve(&section, &xz(0.)?, Vec3::ZERO, Vec3::Z, TAU, "flange").map_err(e)?;
    let holes: Vec<Brep> = (0..6)
        .map(|k| {
            let a = k as f64 * PI / 3.;
            cylinder(&at([24. * a.cos(), 24. * a.sin(), -1.]), 3., 8., "bolt").map_err(e)
        })
        .collect::<Result<_, _>>()?;
    difference_all(&body, &holes.iter().collect::<Vec<_>>()).map_err(e)
}
