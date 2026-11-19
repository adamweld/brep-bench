//! A pulley on the z axis: a hub of radius 12 (|z| ≤ 14) and a rim of radius 28 (|z| ≤ 9), bored
//! at radius 6, less a V groove turned from the triangle (22, 0), (29, −5), (29, 5).
use std::f64::consts::TAU;

use springrep::boolean::difference;
use springrep::math::Vec3;
use springrep::sweep::revolve;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::parts::{polygon, xz};

pub fn build() -> Result<Brep, String> {
    let half = [(6., -14.), (12., -14.), (12., -9.), (28., -9.), (28., 9.), (12., 9.), (12., 14.), (6., 14.)];
    let body = revolve(&polygon(&half, "turned"), &xz(0.)?, Vec3::ZERO, Vec3::Z, TAU, "pulley")
        .map_err(e)?;
    let groove = polygon(&[(22., 0.), (29., -5.), (29., 5.)], "groove");
    let groove = revolve(&groove, &xz(0.)?, Vec3::ZERO, Vec3::Z, TAU, "groove").map_err(e)?;
    difference(&body, &groove).map_err(e)
}
